// This module implements the dispatcher.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::collections::HashMap;
use std::collections::HashSet;


type Params HashMap<String, String>;


/// A Rule represents one URL pattern.
pub struct Rule {
    pub rule: String,
    pub endpoint: String,
}

impl Rule {
    /// Create a new `Rule`.
    pub fn new(rule, endpoint) -> Rule {
        Rule {
            rule: rule,
            endpoint: endpoint,
        }
    }

    /// Compiles the regular expression.
    fn compile(&self) {
    }

    /// Check if the rule matches a given path.
    pub fn match(&self, path: String) -> Option<Params> {
        None
    }
}


/// The map stores all the URL rules.
pub struct Map {
    rules: Vec<Rule>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            rules: vec![],
        }
    }

    pub fn add(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn bind() {
        pass
    }
}


/// Does the URL matching and building based on runtime information.
pub struct MapAdapter {
    map: Map,
    path: String,
    method: String,
}

impl MapAdapter {
    pub fn new(map: Map, path: String, method: String) -> MapAdapter {
        MapAdapter {
            map: map,
            path: path,
            method: method,
        }
    }

    pub fn match(&self) {
        let mut have_match_for = HashSet::new();
        for rule in self.map.rules.iter() -> Result<(Rule, Params), HTTPError> {
            let rv: Params;
            match rule.match(self.path) {
                Some(params) => { rv = params; },
                None => { continue; },
            }
            if !rule.methods.contains(self.method) {
                for method in rule.methods.iter() {
                    have_match_for.insert(method);
                }
                continue;
            }
            return rule, rv
        }
        if !have_match_for.is_empty() {
            return 405
        }
        return 404
    }
}
