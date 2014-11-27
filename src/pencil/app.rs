// This module implements the central application object.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::collections::HashMap;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::error::Error;

use http;
use http::server::{Server, Request, ResponseWriter};
use http::server::request::AbsolutePath;
use http::headers::content_type::MediaType;

use types::{
    PencilResult,
        PenValue,
        PenResponse,
        PenError,

    PencilError,
};
use wrappers::{
    Response,
};
use config;
use logging;


/// The pencil type.
#[deriving(Clone)]
pub struct Pencil {
    pub config: config::Config,
    url_map: HashMap<String, String>,
    // A dictionary of all view functions registered.
    view_functions: HashMap<String, PencilResult>,
    before_request_funcs: Vec<String>,
    after_request_funcs: Vec<String>,
    teardown_request_funcs: Vec<String>,
    error_handlers: HashMap<String, PencilResult>,
}

/// The pencil object acts as the central application object.
impl Pencil {

    /// Create a new pencil object.
    pub fn new() -> Pencil {
        Pencil {
            config: config::Config::new(),
            url_map: HashMap::new(),
            view_functions: HashMap::new(),
            before_request_funcs: vec![],
            after_request_funcs: vec![String::from_str("after")],
            teardown_request_funcs: vec![],
            error_handlers: HashMap::new(),
        }
    }

    /// Set global log level based on the application's debug flag.
    pub fn set_log_level(&self) {
        logging::set_log_level(self);
    }

    /// Connects a URL rule.
    pub fn add_url_rule(&mut self, rule: String, endpoint: String, view_func: PencilResult) {
        self.url_map.insert(rule, endpoint.clone());
        self.view_functions.insert(endpoint, view_func);
    }

    /// Registers a function to run before each request.
    pub fn before_request(&mut self, f: String) {
        self.before_request_funcs.push(f);
    }

    /// Registers a function to run after each request.  Your function
    /// must take a response object and modify it.
    pub fn after_request(&mut self, f: String) {
        self.after_request_funcs.push(f);
    }

    /// Registers a function to run at the end of each request,
    /// regardless of whether there was an error or not.
    pub fn teardown_request(&mut self, f: String) {
        self.teardown_request_funcs.push(f);
    }

    /// Registers a function as one error handler.
    pub fn register_error_handler(&mut self, error: PencilError, f: PencilResult) {
        // TODO: seperate http code and others
        self.error_handlers.insert(error.description().to_string(), f);
    }

    /// Called before the actual request dispatching, you can return value
    /// from here and stop the further request handling.
    fn preprocess_request(&self) {
        for x in self.before_request_funcs.iter() {
            println!("{}", x);
        }
    }

    /// Does the request dispatching.  Matches the URL and returns the return
    /// value of the view.
    fn dispatch_request(&self, request: Request) -> PencilResult {
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
        let rv = match self.url_map.get(&request_url) {
            Some(endpoint) => {
                match self.view_functions.get(endpoint) {
                    Some(response) => response.clone(),
                    _ => PenValue(String::from_str("No such handler")),
                }
            },
            _ => PenValue(String::from_str("404")),
        };
        return rv;
    }

    /// Converts the return value from a view function to a real
    /// response object.
    fn make_response(&self, rv: PencilResult) -> Response {
        match rv {
            PenValue(rv) => Response::new(rv),
            PenResponse(response) => response,
            PenError(e) => Response::new(e.description().to_string()),
        }
    }

    /// Modify the response object before it's sent to the HTTP server.
    fn process_response(&self, response: &mut Response) {
        // TODO: reverse order
        for x in self.after_request_funcs.iter() {
            response.body.push_str(x.as_slice());
        }
    }

    /// Called after the actual request dispatching.
    fn do_teardown_request(&self) {
        // TODO: reverse order
        for x in self.teardown_request_funcs.iter() {
            println!("{}", x);
        }
    }

    /// This method is called whenever an error occurs that should be handled.
    fn handle_user_error(&self, e: PencilError) -> PencilResult {
        match self.error_handlers.get(&e.description().to_string()) {
            Some(handler) => handler.clone(),
            _ => self.handle_http_error(e),
        }
    }

    /// Handles an HTTP error.
    fn handle_http_error(&self, e: PencilError) -> PencilResult {
        match self.error_handlers.get(&e.description().to_string()) {
            Some(handler) => handler.clone(),
            _ => PenError(e),
        }
    }

    /// Default error handing that kicks in when an error occurs that is not
    /// handled.
    fn handle_error(&self, e: PencilError) -> PencilResult {
        self.log_error(e);
        match self.error_handlers.get(&e.description().to_string()) {
            Some(handler) => handler.clone(),
            _ => PenError(e),  // 500
        }
    }

    /// Logs an error.
    fn log_error(&self, e: PencilError) {
        println!("{}", e.description());
    }

    /// Dispatches the request and performs request pre and postprocessing
    /// as well as HTTP error handling.
    fn full_dispatch_request(&self, request: Request) -> Result<Response, PencilError> {
        self.preprocess_request();
        let rv = match self.dispatch_request(request) {
            PenValue(rv) => PenValue(rv),
            PenResponse(response) => PenResponse(response),
            PenError(e) => self.handle_user_error(e),
        };
        let mut response = self.make_response(rv);
        self.process_response(&mut response);
        return Ok(response);
    }

    /// The actual application.  Middlewares can be applied here.
    /// You can do this:
    ///     application.app = MyMiddleware(application.app)
    pub fn app(&self, request: Request, w: &mut ResponseWriter) {
        let response = match self.full_dispatch_request(request) {
            Ok(response) => response,
            Err(e) => self.make_response(self.handle_error(e)),
        };

        w.headers.content_type = Some(MediaType {
            type_ : String::from_str("text"),
            subtype: String::from_str("plain"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });
        w.headers.server = Some(String::from_str("Pencil"));
        w.write(response.body.as_bytes()).unwrap();

        self.do_teardown_request();
    }

    /// Runs the application on a local development server.
    pub fn run(self) {
        self.serve_forever();
    }
}

impl Server for Pencil {

    fn get_config(&self) -> http::server::Config {
        http::server::Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8000 } }
    }

    fn handle_request(&self, r: Request, w: &mut ResponseWriter) {
        self.app(r, w);
    }
}
