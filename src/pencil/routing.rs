// This module implements the dispatcher.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::collections::HashSet;
use regex::Regex;
use std::ascii::AsciiExt;

use errors::{HTTPError, MethodNotAllowed, NotFound};
use types::ViewArgs;


/// A Rule represents one URL pattern.
#[deriving(Clone)]
pub struct Rule {
    pub rule: String,
    pub methods: HashSet<String>,
    pub endpoint: String,
    pub regex: Regex,
}

impl Rule {
    /// Create a new `Rule`.  Rule strings basically are just normal url
    /// regular expressions.  Rule endpoint is a string that is used for
    /// URL generation.  Rule methods is an array of http methods this rule
    /// applies to, if `GET` is present in it and `HEAD` is not, `HEAD` is
    /// added automatically.
    pub fn new(string: &'static str, methods: &[&str], endpoint: &str) -> Rule {
        if !string.starts_with("/") {
            panic!("urls must start with a leading slash");
        }
        let mut full_string = String::from_str(r"^");
        full_string = full_string + string;
        full_string = full_string + r"$";

        let mut upper_methods: HashSet<String> = HashSet::new();
        for &method in methods.iter() {
            let upper_method = method.to_string().to_ascii_upper();
            upper_methods.insert(upper_method);
        }
        if upper_methods.contains(&String::from_str("GET")) {
            upper_methods.insert(String::from_str("HEAD"));
        }
        Rule {
            endpoint: endpoint.to_string(),
            methods: upper_methods,
            regex: Rule::compile(full_string.as_slice()),
            rule: full_string,
        }
    }

    /// Compiles the regular expression.
    fn compile(string: &str) -> Regex {
        Regex::new(string).unwrap()
    }

    /// Check if the rule matches a given path.
    pub fn captures(&self, path: String) -> Option<ViewArgs> {
        match self.regex.captures(path.as_slice()) {
            Some(caps) => {
                let mut view_args: Vec<String> = vec![];
                let mut iter = caps.iter();
                iter.next();
                for c in iter {
                    view_args.push(c.to_string());
                }
                Some(view_args)
            },
            None => None,
        }
    }
}


/// The map stores all the URL rules.
#[deriving(Clone)]
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

    pub fn bind(&self, path: String, method: String) -> MapAdapter {
        MapAdapter::new(self, path, method)
    }
}


/// Does the URL matching and building based on runtime information.
pub struct MapAdapter<'m> {
    map: &'m Map,
    path: String,
    method: String,
}

impl<'m> MapAdapter<'m> {
    pub fn new(map: &Map, path: String, method: String) -> MapAdapter {
        MapAdapter {
            map: map,
            path: path,
            method: method,
        }
    }

    pub fn captures(&self) -> Result<(Rule, ViewArgs), HTTPError> {
        let mut have_match_for = HashSet::new();
        for rule in self.map.rules.iter() {
            let rv: ViewArgs;
            match rule.captures(self.path.clone()) {
                Some(view_args) => { rv = view_args; },
                None => { continue; },
            }
            if !rule.methods.contains(&self.method) {
                for method in rule.methods.iter() {
                    have_match_for.insert(method);
                }
                continue;
            }
            return Ok((rule.clone(), rv))
        }
        if !have_match_for.is_empty() {
            return Err(MethodNotAllowed)
        }
        return Err(NotFound)
    }
}

#[test]
fn test_basic_routing() {
    let mut map = Map::new();
    map.add(Rule::new(r"/", &["GET"], "index"));
    map.add(Rule::new(r"/foo", &["GET"], "foo"));
    map.add(Rule::new(r"/bar/", &["GET"], "bar"));
    let adapter = map.bind(String::from_str("/bar/"), String::from_str("GET"));
    match adapter.captures() {
        Ok((rule, view_args)) => {
            assert!(rule.rule.as_slice() == r"^/bar/$");
            assert!(rule.methods.contains("GET"));
            assert!(!rule.methods.contains("POST"));
            assert!(rule.endpoint == String::from_str("bar"));
            assert!(view_args.len() == 0);
        },
        _ => { panic!("Basic routing failed!"); }
    }
}
