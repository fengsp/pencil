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
    before_request_funcs: Vec<String>,
    after_request_funcs: Vec<String>,
}

/// The pencil object acts as the central application object.
impl Pencil {

    /// Create a new pencil object.
    pub fn new() -> Pencil {
        Pencil {
            url_map: HashMap::new(),
            view_functions: HashMap::new(),
            before_request_funcs: vec![],
            after_request_funcs: vec![String::from_str("after")],
        }
    }

    /// Connects a URL rule.
    pub fn add_url_rule(&mut self, rule: String, endpoint: String, view_func: String) {
        self.url_map.insert(rule, endpoint.clone());
        self.view_functions.insert(endpoint, view_func);
    }

    /// Called before the actual request dispatching, you can return value
    /// from here and stop the further request handling.
    fn preprocess_request(&self) {
        for x in self.before_request_funcs.iter() {
            println!("{}", x);
        }
    }

    fn dispatch_request(&self, request: Request) -> String {
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
        let rv = match self.url_map.find(&request_url) {
            Some(endpoint) => {
                match self.view_functions.find(endpoint) {
                    Some(response) => response.clone(),
                    _ => String::from_str("No such handler"),
                }
            },
            _ => String::from_str("404"),
        };
        return rv;
    }

    /// Converts the return value from a view function to a real
    /// response object.
    fn make_response(&self, rv: String) -> String {
        return rv;
    }

    /// Modify the response object before it's sent to the HTTP server.
    fn process_response(&self, response: &mut String) {
        // TODO: reverse order
        for x in self.after_request_funcs.iter() {
            response.push_str(x.as_slice());
        }
    }

    /// Dispatches the request and performs request pre and postprocessing
    /// as well as HTTP error handling.
    fn full_dispatch_request(&self, request: Request) -> String {
        self.preprocess_request();
        let rv = self.dispatch_request(request);
        // self.handle_user_exception(e)
        let mut response = self.make_response(rv);
        self.process_response(&mut response);
        return response;
    }

    /// The actual application.  Middlewares can be applied here.
    pub fn app(&self, request: Request, w: &mut ResponseWriter) {
        let response = self.full_dispatch_request(request);

        w.headers.content_type = Some(MediaType {
            type_ : String::from_str("text"),
            subtype: String::from_str("plain"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });
        w.headers.server = Some(String::from_str("Pencil"));
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
