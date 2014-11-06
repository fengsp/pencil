// This module implements the central application object.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::collections::HashMap;

/// The pencil type.
pub struct Pencil {
    url_map: HashMap<String, String>,
    view_functions: HashMap<String, String>,
}

/// The pencil object acts as the central application object.
impl Pencil {

    /// Create a new pencil object.
    pub fn new() -> Pencil {
        Pencil {
            url_map: HashMap::new(),
            view_functions: HashMap::new(),
        }
    }

    /// Connects a URL rule.
    pub fn add_url_rule(&mut self, rule: String, endpoint: String, view_func: String) {
        self.url_map.insert(rule, endpoint.clone());
        self.view_functions.insert(endpoint, view_func);
    }

    /// Runs the application on a local development server.
    pub fn run(&self, request_url: String) {
        match self.url_map.find(&request_url) {
            Some(endpoint) => {
                match self.view_functions.find(endpoint) {
                    Some(response) => println!("Response: {}", response),
                    _ => println!("No such handler"),
                }
            },
            _ => println!("404"),
        }
    }
}
