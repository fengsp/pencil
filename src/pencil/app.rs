// This module implements the central application object.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::collections::HashMap;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

use http::server::{Config, Server, Request, ResponseWriter};
use http::server::request::AbsolutePath;
use http::headers::content_type::MediaType;

/// The pencil type.
#[deriving(Clone)]
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

    /// The actual application.  Middlewares can be applied here.
    pub fn app(&self, request: Request, w: &mut ResponseWriter) {
        w.headers.content_type = Some(MediaType {
            type_ : String::from_str("text"),
            subtype: String::from_str("plain"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });
        w.headers.server = Some(String::from_str("Pencil"));

        let request_url = match request.request_uri {
            AbsolutePath(url) => {
                println!("{}", url);
                url
            },
            _ => {
                println!("{}", "WTF!");
                "wtf".to_string()
            },
        };
        let response = match self.url_map.find(&request_url) {
            Some(endpoint) => {
                match self.view_functions.find(endpoint) {
                    Some(response) => response.clone(),
                    _ => String::from_str("No such handler"),
                }
            },
            _ => String::from_str("404"),
        };

        w.write(response.as_bytes()).unwrap();
    }

    /// Runs the application on a local development server.
    pub fn run(self) {
        self.serve_forever();
    }
}

impl Server for Pencil {

    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8000 } }
    }

    fn handle_request(&self, r: Request, w: &mut ResponseWriter) {
        self.app(r, w);
    }
}
