//! This module implements the dispatcher.

use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use regex::quote as regex_quote;

use hyper::method::Method;

use http_errors::{HTTPError, MethodNotAllowed, NotFound};
use types::ViewArgs;
use utils::join_string;

/// Parse a rule and return a list of tuples in the form
/// `(Option<converter>, variable)`.  If the converter
/// is `None`, it's a static url part.
fn parse_rule(rule: &str) -> Vec<(Option<&str>, &str)> {
    let rule_re = Regex::new(r"(?x)
        (?P<static>[^<]*)                            # static rule data
        <
        (?:
            (?P<converter>[a-zA-Z_][a-zA-Z0-9_]*)    # converter name
            :                                        # variable delimiter
        )?
        (?P<variable>[a-zA-Z_][a-zA-Z0-9_]*)         # variable name
        >
    ").unwrap();
    let mut rule_parts: Vec<(Option<&str>, &str)> = Vec::new();
    let mut remaining = rule;
    let mut used_names = HashSet::new();
    while !remaining.is_empty() {
        match rule_re.captures(remaining) {
            Some(caps) => {
                let static_part = caps.name("static");
                if static_part.is_some() {
                    rule_parts.push((None, static_part.unwrap()));
                }
                let variable = caps.name("variable").unwrap();
                let converter = match caps.name("converter") {
                    Some(converter) => { converter },
                    None => { "default" },
                };
                if used_names.contains(variable) {
                    panic!("variable name {} used twice.", variable);
                }
                used_names.insert(variable);
                rule_parts.push((Some(converter), variable));
                let end = caps.pos(0).unwrap().1;
                let (_, tail) = remaining.split_at(end);
                remaining = tail;
            },
            None => {
                break;
            }
        }
    }
    if !remaining.is_empty() {
        if remaining.contains(">") || remaining.contains("<") {
            panic!("malformed url rule: {}", rule);
        }
        rule_parts.push((None, remaining));
    }
    rule_parts
}

/// The matcher holds the url regex object.
#[derive(Clone)]
pub struct Matcher {
    pub regex: Regex
}

impl Matcher {
    pub fn new(regex: Regex) -> Matcher {
        Matcher {
            regex: regex
        }
    }
}

/// Rule strings basically are just normal URL paths with placeholders in
/// the format `<converter:name>` where the converter are optional.
/// Currently we support following converters:
///
/// - string(default)
/// - int
/// - float
/// - path
///
/// If no converter is defined the `default` converter is used which means `string`.
///
/// URL rules that end with a slash are branch URLs, others are leaves.
impl<'a> From<&'a str> for Matcher {
    fn from(rule: &'a str) -> Matcher {
        if !rule.starts_with("/") {
            panic!("urls must start with a leading slash");
        }
        let is_branch = rule.ends_with("/");

        // Compiles the regular expression
        let mut regex_parts: Vec<String> = Vec::new();
        for (converter, variable) in parse_rule(rule.trim_right_matches("/")) {
            match converter {
                Some(converter) => {
                    let re = match converter {
                        "string" => "[^/]{1,}",
                        "int" => r"\d+",
                        "float" => r"\d+\.\d+",
                        "path" => "[^/].*?",
                        "default" => "[^/]{1,}",
                        _ => { panic!("the converter {} does not exist", converter); }
                    };
                    regex_parts.push(format!("(?P<{}>{})", variable, re));
                },
                None => {
                    let escaped_variable = regex_quote(variable);
                    regex_parts.push(escaped_variable);
                }
            }
        }
        if is_branch {
            regex_parts.push(String::from("(?P<__suffix__>/?)"));
        }
        let regex = format!(r"^{}$", join_string(regex_parts, ""));
        Matcher::new(Regex::new(&regex).unwrap())
    }
}

/// Same rule as `&str`.
impl From<String> for Matcher {
    fn from(rule: String) -> Matcher {
        let rule_str: &str = &rule;
        rule_str.into()
    }
}

impl From<Regex> for Matcher {
    fn from(regex: Regex) -> Matcher {
        Matcher::new(regex)
    }
}


/// A Rule represents one URL pattern.
#[derive(Clone)]
pub struct Rule {
    /// The matcher is used to match the url path.
    pub matcher: Matcher,
    /// A set of http methods this rule applies to.
    pub methods: HashSet<Method>,
    /// The endpoint for this rule.
    pub endpoint: String,
    pub provide_automatic_options: bool,
}

impl Rule {
    /// Create a new `Rule`.  Matcher basically are used to hold url
    /// regular expressions.  Rule endpoint is a string that is used for
    /// URL generation.  Rule methods is an array of http methods this rule
    /// applies to, if `GET` is present in it and `HEAD` is not, `HEAD` is
    /// added automatically.
    pub fn new(matcher: Matcher, methods: &[Method], endpoint: &str) -> Rule {
        let mut all_methods = HashSet::new();
        for method in methods.iter() {
            all_methods.insert(method.clone());
        }
        if all_methods.contains(&Method::Get) {
            all_methods.insert(Method::Head);
        }
        let provide_automatic_options = if all_methods.contains(&Method::Options) {
            false
        } else {
            all_methods.insert(Method::Options);
            true
        };
        Rule {
            matcher: matcher,
            endpoint: endpoint.to_string(),
            methods: all_methods,
            provide_automatic_options: provide_automatic_options,
        }
    }

    /// Check if the rule matches a given path.
    pub fn matched(&self, path: String) -> Option<ViewArgs> {
        match self.matcher.regex.captures(&path) {
            Some(caps) => {
                // All branch URLs that are matched without a trailing slash will trigger a
                // redirect to the same URL with the missing slash appended.
                // We have a url without a trailing slash for branch url rule.
                // So we redirect to the same url but with a trailing slash.
                match caps.name("__suffix__") {
                    Some(suffix) => {
                        if suffix.is_empty() {
                            // TODO: we should redirect here
                            return None;
                        }
                    },
                    None => {}
                }
                let mut view_args: HashMap<String, String> = HashMap::new();
                for variable in self.matcher.regex.capture_names() {
                    match variable {
                        Some(variable) => {
                            if variable != "__suffix__" {
                                view_args.insert(variable.to_string(), caps.name(variable).unwrap().to_string());
                            }
                        },
                        None => {}
                    }
                }
                Some(view_args)
            },
            None => None,
        }
    }
}


/// The map stores all the URL rules.
#[derive(Clone)]
pub struct Map {
    rules: Vec<Rule>,
}

impl Map {
    pub fn new() -> Map {
        Map { rules: vec![] }
    }

    pub fn add(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn bind(&self, path: String, method: Method) -> MapAdapter {
        MapAdapter::new(self, path, method)
    }
}


/// Does the URL matching and building based on runtime information.
pub struct MapAdapter<'m> {
    map: &'m Map,
    path: String,
    method: Method,
}

impl<'m> MapAdapter<'m> {
    pub fn new(map: &Map, path: String, method: Method) -> MapAdapter {
        MapAdapter {
            map: map,
            path: path,
            method: method,
        }
    }

    pub fn matched(&self) -> Result<(Rule, ViewArgs), HTTPError> {
        let mut have_match_for = HashSet::new();
        for rule in self.map.rules.iter() {
            let rv: ViewArgs;
            match rule.matched(self.path.clone()) {
                Some(view_args) => { rv = view_args; },
                None => { continue; },
            }
            if !rule.methods.contains(&self.method) {
                for method in &rule.methods {
                    have_match_for.insert(method.clone());
                }
                continue;
            }
            return Ok((rule.clone(), rv))
        }
        if !have_match_for.is_empty() {
            let mut allowed_methods = Vec::new();
            allowed_methods.extend(have_match_for.into_iter());
            return Err(MethodNotAllowed(Some(allowed_methods)))
        }
        Err(NotFound)
    }

    /// Get the valid methods that match for the given path.
    pub fn allowed_methods(&self) -> Vec<Method> {
        let mut have_match_for = HashSet::new();
        for rule in self.map.rules.iter() {
            match rule.matched(self.path.clone()) {
                Some(_) => {
                    for method in &rule.methods {
                        have_match_for.insert(method.clone());
                    }
                    continue;
                },
                None => { continue; },
            }
        }
        let mut allowed_methods = Vec::new();
        allowed_methods.extend(have_match_for.into_iter());
        allowed_methods
    }
}


#[test]
fn test_basic_routing() {
    let mut map = Map::new();
    map.add(Rule::new("/".into(), &[Method::Get], "index"));
    map.add(Rule::new("/foo".into(), &[Method::Get], "foo"));
    map.add(Rule::new("/bar/".into(), &[Method::Get], "bar"));
    let adapter = map.bind(String::from("/bar/"), Method::Get);
    match adapter.matched() {
        Ok((rule, view_args)) => {
            assert!(rule.methods.contains(&Method::Get));
            assert!(!rule.methods.contains(&Method::Post));
            assert!(rule.endpoint == String::from("bar"));
            assert!(view_args.len() == 0);
        },
        _ => { panic!("Basic routing failed!"); }
    }
}
