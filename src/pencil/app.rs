// This module implements the central application object.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use hyper;
use hyper::server::Request as HTTPRequest;
use hyper::server::Response as HTTPResponse;
use hyper::net::Fresh;

use types::{
    PencilValue,
        PenResponse,

    PencilError,
        PenHTTPError,
        PenUserError,

    UserError,
    PencilResult,
    ViewFunc,
    HTTPErrorHandler,
    UserErrorHandler,
    BeforeRequestFunc,
    AfterRequestFunc,
    TeardownRequestFunc,
};
use wrappers::{
    Request,
    Response,
};
use helpers::{PathBound, send_static_file};
use helpers;
use config;
use logging;
use serving::run_server;
use routing::{Map, Rule};
use testing::PencilClient;
use errors::{HTTPError, NotFound, InternalServerError};


/// The pencil type.  It acts as the central application object.  Once it is created it
/// will act as a central registry for the view functions, the URL rules and much more.
#[derive(Clone)]
pub struct Pencil {
    pub root_path: String,
    pub static_folder: String,
    pub static_url_path: String,
    pub config: config::Config,
    pub url_map: Map,
    // A dictionary of all view functions registered.
    pub view_functions: HashMap<String, ViewFunc>,
    pub before_request_funcs: Vec<BeforeRequestFunc>,
    pub after_request_funcs: Vec<AfterRequestFunc>,
    pub teardown_request_funcs: Vec<TeardownRequestFunc>,
    pub http_error_handlers: HashMap<isize, HTTPErrorHandler>,
    pub user_error_handlers: HashMap<&'static str, UserErrorHandler>,
}

impl Pencil {
    /// Create a new pencil object.  It is passed the root path of your application.
    /// The root path is used to resolve resources from inside it, for more information
    /// about resource loading, see method `open_resource`.
    ///
    /// Usually you create a pencil object in your main function like this:
    ///
    /// ```rust,no_run
    /// use pencil::Pencil;
    ///
    /// fn main() {
    ///     let mut app = Pencil::new("/web/myapp");
    /// }
    /// ```
    pub fn new(root_path: &str) -> Pencil {
        Pencil {
            root_path: root_path.to_string(),
            static_folder: String::from_str("static"),
            static_url_path: String::from_str("/static"),
            config: config::Config::new(),
            url_map: Map::new(),
            view_functions: HashMap::new(),
            before_request_funcs: vec![],
            after_request_funcs: vec![],
            teardown_request_funcs: vec![],
            http_error_handlers: HashMap::new(),
            user_error_handlers: HashMap::new(),
        }
    }

    /// Set global log level based on the application's debug flag.
    pub fn set_log_level(&self) {
        logging::set_log_level(self);
    }

    /// A shortcut that is used to register a view function for a given
    /// URL rule.
    pub fn route(&mut self, rule: &str, methods: &[&str], endpoint: &str, view_func: ViewFunc) {
        self.add_url_rule(rule, methods, endpoint, view_func);
    }

    /// Connects a URL rule.
    fn add_url_rule(&mut self, rule: &str, methods: &[&str], endpoint: &str, view_func: ViewFunc) {
        let url_rule = Rule::new(rule, methods, endpoint);
        self.url_map.add(url_rule);
        self.view_functions.insert(endpoint.to_string(), view_func);
    }

    /// Enables static file handling.
    pub fn enable_static_file_handle(&mut self) {
        let mut rule = self.static_url_path.clone();
        rule = rule + "/([^/].*?)";
        self.add_url_rule(rule.as_slice(), &["GET"], "static", send_static_file);
    }

    /// Registers a function to run before each request.
    pub fn before_request(&mut self, f: BeforeRequestFunc) {
        self.before_request_funcs.push(f);
    }

    /// Registers a function to run after each request.  Your function
    /// must take a response object and modify it.
    pub fn after_request(&mut self, f: AfterRequestFunc) {
        self.after_request_funcs.push(f);
    }

    /// Registers a function to run at the end of each request,
    /// regardless of whether there was an error or not.
    pub fn teardown_request(&mut self, f: TeardownRequestFunc) {
        self.teardown_request_funcs.push(f);
    }

    /// Registers a function as one http error handler.
    fn register_http_error_handler(&mut self, status_code: isize, f: HTTPErrorHandler) {
        self.http_error_handlers.insert(status_code, f);
    }

    /// Registers a function as one user error handler.
    fn register_user_error_handler(&mut self, error_desc: &'static str, f: UserErrorHandler) {
        self.user_error_handlers.insert(error_desc, f);
    }

    /// Registers a function as one http error handler.  Example:
    ///
    /// ```rust,no_run
    /// use pencil::{Pencil, PencilResult, Response, PenResponse};
    /// use pencil::HTTPError;
    ///
    ///
    /// fn page_not_found(error: HTTPError) -> PencilResult {
    ///     let mut response = Response::new(String::from_str("The page does not exist"));
    ///     response.status_code = 404;
    ///     return Ok(PenResponse(response));
    /// }
    ///
    ///
    /// fn main() {
    ///     let mut app = Pencil::new("/web/demo");
    ///     app.httperrorhandler(404, page_not_found);
    /// }
    /// ```
    pub fn httperrorhandler(&mut self, status_code: isize, f: HTTPErrorHandler) {
        self.register_http_error_handler(status_code, f);
    }

    /// Registers a function as one user error handler.  There are two ways to handle
    /// user errors currently, you can do it in your own view like this:
    ///
    /// ```rust,no_run
    /// use pencil::Request;
    /// use pencil::{PencilResult, PenString};
    ///
    ///
    /// struct MyErr(isize);
    ///
    ///
    /// fn some_operation() -> Result<isize, MyErr> {
    ///     return Err(MyErr(10));
    /// }
    ///
    ///
    /// fn my_err_handler(_: MyErr) -> PencilResult {
    ///     Ok(PenString(String::from_str("My err occurred!")))
    /// }
    ///
    ///
    /// fn hello(_: Request) -> PencilResult {
    ///     match some_operation() {
    ///         Ok(_) => Ok(PenString(String::from_str("Hello!"))),
    ///         Err(e) => my_err_handler(e),
    ///     }
    /// }
    /// ```
    ///
    /// The problem with this is that you have to do it in all of your views, it brings
    /// a lot of redundance, so pencil provides another solution, currently I still
    /// haven't got any better idea on how to store user error handlers, this feature is
    /// really just experimental, if you have any good idea, please wake me up.  Here is
    /// one simple example:
    ///
    /// ```rust,no_run
    /// use std::error::FromError;
    ///
    /// use pencil::Request;
    /// use pencil::{Pencil, PencilResult, PenString};
    /// use pencil::{PencilError, PenUserError, UserError};
    ///
    ///
    /// #[derive(Copy)]
    /// pub struct MyErr(isize);
    ///
    /// impl FromError<MyErr> for PencilError {
    ///     fn from_error(err: MyErr) -> PencilError {
    ///         let user_error = UserError::new("MyErr", None);
    ///         return PenUserError(user_error);
    ///     }
    /// }
    ///
    ///
    /// fn my_err_handler(_: UserError) -> PencilResult {
    ///     Ok(PenString(String::from_str("My err occurred!")))
    /// }
    ///
    ///
    /// fn some_operation() -> Result<String, MyErr> {
    ///     return Err(MyErr(10));
    /// }
    ///
    ///
    /// fn hello(_: Request) -> PencilResult {
    ///     let rv = try!(some_operation());
    ///     return Ok(PenString(rv));
    /// }
    ///
    ///
    /// fn main() {
    ///     let mut app = Pencil::new("/web/demo");
    ///     // Use error description as key to store handlers, really ugly...
    ///     app.usererrorhandler("MyErr", my_err_handler);
    /// }
    /// ```
    pub fn usererrorhandler(&mut self, error_desc: &'static str, f: UserErrorHandler) {
        self.register_user_error_handler(error_desc, f);
    }

    /// Creates a test client for this application, you can use it
    /// like this:
    ///
    /// ```ignore
    /// let client = app.test_client();
    /// let response = client.get('/');
    /// assert!(response.code, 200);
    /// ```
    pub fn test_client(&self) -> PencilClient {
        PencilClient::new(self)
    }

    /// Called before the actual request dispatching, you can return value
    /// from here and stop the further request handling.
    fn preprocess_request(&self, request: &Request) -> Option<PencilResult> {
        let mut result: Option<PencilResult>;
        for &func in self.before_request_funcs.iter() {
            result = func(request);
            if result.is_some() {
                return result;
            }
        }
        return None;
    }

    /// Does the request dispatching.  Matches the URL and returns the return
    /// value of the view.
    fn dispatch_request(&self, request: Request) -> PencilResult {
        if request.routing_error.is_some() {
            return Err(PenHTTPError(request.routing_error.unwrap()));
        }
        match self.view_functions.get(&request.endpoint().unwrap()) {
            Some(&view_func) => {
                return view_func(request);
            },
            None => {
                return Err(PenHTTPError(NotFound));
            }
        }
    }

    /// Converts the return value from a view function to a real
    /// response object.
    fn make_response(&self, rv: PencilValue) -> Response {
        return helpers::make_response(rv);
    }

    /// Modify the response object before it's sent to the HTTP server.
    fn process_response(&self, response: &mut Response) {
        // TODO: reverse order
        for &func in self.after_request_funcs.iter() {
            func(response);
        }
    }

    /// Called after the actual request dispatching.
    fn do_teardown_request(&self, e: Option<&PencilError>) {
        // TODO: reverse order
        for &func in self.teardown_request_funcs.iter() {
            func(e);
        }
    }

    /// This method is called whenever an error occurs that should be handled.
    fn handle_all_error(&self, e: PencilError) -> PencilResult {
        match e {
            PenHTTPError(e) => self.handle_http_error(e),
            PenUserError(e) => self.handle_user_error(e),
        }
    }

    /// Handles an User error.
    fn handle_user_error(&self, e: UserError) -> PencilResult {
        match self.user_error_handlers.get(e.description()) {
            Some(&handler) => handler(e),
            None => Err(PenUserError(e)),
        }
    }

    /// Handles an HTTP error.
    fn handle_http_error(&self, e: HTTPError) -> PencilResult {
        match self.http_error_handlers.get(&e.code()) {
            Some(&handler) => handler(e),
            None => Ok(PenResponse(e.to_response())),
        }
    }

    /// Default error handing that kicks in when an error occurs that is not
    /// handled.
    fn handle_error(&self, e: &PencilError) -> PencilValue {
        self.log_error(e);
        let internal_server_error = InternalServerError;
        match self.http_error_handlers.get(&500) {
            Some(&handler) => {
                match handler(internal_server_error) {
                    Ok(value) => value,
                    Err(_) => {
                        let e = InternalServerError;
                        PenResponse(e.to_response())
                    }
                }
            },
            None => {
                let e = InternalServerError;
                PenResponse(e.to_response())
            }
        }
    }

    /// Logs an error.
    fn log_error(&self, e: &PencilError) {
        error!("Error: {}", e.description());
    }

    /// Dispatches the request and performs request pre and postprocessing
    /// as well as HTTP error handling.
    fn full_dispatch_request(&self, request: Request) -> Result<Response, PencilError> {
        let result = match self.preprocess_request(&request) {
            Some(result) => result,
            None => self.dispatch_request(request),
        };
        let rv = match result {
            Ok(value) => Ok(value),
            Err(e) => self.handle_all_error(e),
        };
        match rv {
            Ok(value) => {
                let mut response = self.make_response(value);
                self.process_response(&mut response);
                Ok(response)
            },
            Err(e) => Err(e),
        }
    }

    /// The actual application handler.
    pub fn handle_request(&self, mut request: Request) -> Response {
        request.match_request();
        match self.full_dispatch_request(request) {
            Ok(response) => {
                self.do_teardown_request(None);
                return response;
            },
            Err(e) => {
                let response = self.make_response(self.handle_error(&e));
                self.do_teardown_request(Some(&e));
                return response;
            }
        };
    }

    /// Runs the application on a local development server.
    pub fn run(self) {
        run_server(self);
    }
}

impl hyper::server::Handler for Pencil {
    fn handle<'a>(&'a self, mut req: HTTPRequest<'a>, mut res: HTTPResponse<Fresh>) {
        let request = Request::new(self, req);
        let request_method = request.method();
        let response = self.handle_request(request);
        response.write(request_method, res);
    }
}

impl PathBound for Pencil {
    fn open_resource(&self, resource: &str) -> File {
        let mut path = Path::new(self.root_path.as_slice());
        path.push(resource);
        return File::open(&path).unwrap();
    }
}
