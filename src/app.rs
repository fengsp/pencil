//! This module implements the central application object.

use std::convert::Into;
use std::sync::RwLock;
use std::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::net::ToSocketAddrs;

use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use handlebars::Handlebars;
use hyper;
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::server::Request as HTTPRequest;
use hyper::server::Response as HTTPResponse;

use types::{
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
use helpers::{PathBound, send_from_directory, redirect};
use config::Config;
use logging;
use serving::run_server;
use routing::{Map, Rule, Matcher};
use testing::PencilClient;
use http_errors::{HTTPError, NotFound, InternalServerError};
use templating::{render_template, render_template_string, load_template};
use module::Module;


/// The pencil type.  It acts as the central application object.  Once it is created it
/// will act as a central registry for the view functions, the URL rules and much more.
pub struct Pencil {
    /// The path where your application locates.
    pub root_path: String,
    /// The name of the application.  By default it's guessed from the root path.
    pub name: String,
    /// The folder with static files that should be served at `static_url_path`.
    /// Defaults to the `"static"` folder in the root path of the application.
    pub static_folder: String,
    /// The url path for the static files on the web, defaults to be `"/static"`.
    pub static_url_path: String,
    /// The folder that contains the templates that should be used for the application.
    /// Defaults to `''templates''` folder in the root path of the application.
    pub template_folder: String,
    /// The configuration for this application.
    pub config: Config,
    /// The Handlebars registry used to load templates and register helpers.
    pub handlebars_registry: RwLock<Box<Handlebars>>,
    /// The url map for this pencil application.
    pub url_map: Map,
    /// All the attached modules in a hashmap by name.
    pub modules: HashMap<String, Module>,
    /// A dictionary of all view functions registered.  The key will be endpoint.
    view_functions: HashMap<String, ViewFunc>,
    before_request_funcs: Vec<BeforeRequestFunc>,
    after_request_funcs: Vec<AfterRequestFunc>,
    teardown_request_funcs: Vec<TeardownRequestFunc>,
    http_error_handlers: HashMap<u16, HTTPErrorHandler>,
    user_error_handlers: HashMap<String, UserErrorHandler>,
}

fn default_config() -> Config {
    let mut config = Config::new();
    config.set("DEBUG", Json::Boolean(false));
    config.set("TESTING", Json::Boolean(false));
    config
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
            name: root_path.to_string(),
            static_folder: String::from("static"),
            static_url_path: String::from("/static"),
            template_folder: String::from("templates"),
            config: default_config(),
            handlebars_registry: RwLock::new(Box::new(Handlebars::new())),
            url_map: Map::new(),
            modules: HashMap::new(),
            view_functions: HashMap::new(),
            before_request_funcs: vec![],
            after_request_funcs: vec![],
            teardown_request_funcs: vec![],
            http_error_handlers: HashMap::new(),
            user_error_handlers: HashMap::new(),
        }
    }

    /// The debug flag.  This field is configured from the config
    /// with the `DEBUG` configuration key.  Defaults to `False`.
    pub fn is_debug(&self) -> bool {
        self.config.get_boolean("DEBUG", false)
    }

    /// The testing flag.  This field is configured from the config
    /// with the `TESTING` configuration key.  Defaults to `False`.
    pub fn is_testing(&self) -> bool {
        self.config.get_boolean("TESTING", false)
    }

    /// Set the debug flag.  This field is configured from the config
    /// with the `DEBUG` configuration key.  Set this to `True` to
    /// enable debugging of the application.
    pub fn set_debug(&mut self, flag: bool) {
        self.config.set("DEBUG", Json::Boolean(flag));
    }

    /// Set the testing flag.  This field is configured from the config
    /// with the `TESTING` configuration key.  Set this to `True` to
    /// enable the test mode of the application.
    pub fn set_testing(&mut self, flag: bool) {
        self.config.set("TESTING", Json::Boolean(flag));
    }

    /// Set global log level based on the application's debug flag.
    /// This is only useful for `env_logger` crate users.
    /// On debug mode, this turns on all debug logging.
    pub fn set_log_level(&self) {
        logging::set_log_level(self);
    }

    /// This is used to register a view function for a given URL rule.
    /// Basically this example:
    ///
    /// ```rust,ignore
    /// app.route("/home", &[Get], "home", home);
    /// app.route("/user/<user_id:int>", &[Get], "user", user);
    /// ```
    ///
    /// A rule that listens for `GET` will implicitly listen for `HEAD`.
    ///
    pub fn route<M: Into<Matcher>, N: AsRef<[Method]>>(&mut self, rule: M, methods: N, endpoint: &str, view_func: ViewFunc) {
        self.add_url_rule(rule.into(), methods.as_ref(), endpoint, view_func);
    }

    /// This is a shortcut for `route`, register a view function for
    /// a given URL rule with just `GET` method (implicitly `HEAD`).
    pub fn get<M: Into<Matcher>>(&mut self, rule: M, endpoint: &str, view_func: ViewFunc) {
        self.route(rule, &[Method::Get], endpoint, view_func);
    }

    /// This is a shortcut for `route`, register a view function for
    /// a given URL rule with just `POST` method.
    pub fn post<M: Into<Matcher>>(&mut self, rule: M, endpoint: &str, view_func: ViewFunc) {
        self.route(rule, &[Method::Post], endpoint, view_func);
    }

    /// This is a shortcut for `route`, register a view function for
    /// a given URL rule with just `DELETE` method.
    pub fn delete<M: Into<Matcher>>(&mut self, rule: M, endpoint: &str, view_func: ViewFunc) {
        self.route(rule, &[Method::Delete], endpoint, view_func);
    }

    /// This is a shortcut for `route`, register a view function for
    /// a given URL rule with just `PATCH` method.
    pub fn patch<M: Into<Matcher>>(&mut self, rule: M, endpoint: &str, view_func: ViewFunc) {
        self.route(rule, &[Method::Patch], endpoint, view_func);
    }

    /// This is a shortcut for `route`, register a view function for
    /// a given URL rule with just `PUT` method.
    pub fn put<M: Into<Matcher>>(&mut self, rule: M, endpoint: &str, view_func: ViewFunc) {
        self.route(rule, &[Method::Put], endpoint, view_func);
    }

    /// Connects a URL rule.
    pub fn add_url_rule(&mut self, matcher: Matcher, methods: &[Method], endpoint: &str, view_func: ViewFunc) {
        let url_rule = Rule::new(matcher, methods, endpoint);
        self.url_map.add(url_rule);
        self.view_functions.insert(endpoint.to_string(), view_func);
    }

    /// Register a module on the application.
    pub fn register_module(&mut self, module: Module) {
        module.register(self);
    }

    /// Enables static file handling.
    pub fn enable_static_file_handling(&mut self) {
        let mut rule = self.static_url_path.clone();
        rule = rule + "/<filename:path>";
        let rule_str: &str = &rule;
        self.route(rule_str, &[Method::Get], "static", send_app_static_file);
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
    /// Same to `httperrorhandler`.
    pub fn register_http_error_handler(&mut self, status_code: u16, f: HTTPErrorHandler) {
        self.http_error_handlers.insert(status_code, f);
    }

    /// Registers a function as one user error handler.
    /// Same to `usererrorhandler`.
    pub fn register_user_error_handler(&mut self, error_desc: &str, f: UserErrorHandler) {
        self.user_error_handlers.insert(error_desc.to_string(), f);
    }

    /// Registers a function as one http error handler.  Example:
    ///
    /// ```rust,no_run
    /// use pencil::{Pencil, PencilResult, Response};
    /// use pencil::HTTPError;
    ///
    ///
    /// fn page_not_found(error: HTTPError) -> PencilResult {
    ///     let mut response = Response::from("The page does not exist");
    ///     response.status_code = 404;
    ///     return Ok(response);
    /// }
    ///
    ///
    /// fn main() {
    ///     let mut app = Pencil::new("/web/demo");
    ///     app.httperrorhandler(404, page_not_found);
    /// }
    /// ```
    pub fn httperrorhandler(&mut self, status_code: u16, f: HTTPErrorHandler) {
        self.register_http_error_handler(status_code, f);
    }

    /// Registers a function as one user error handler.  There are two ways to handle
    /// user errors currently, you can do it in your own view like this:
    ///
    /// ```rust,no_run
    /// use pencil::Request;
    /// use pencil::{PencilResult, Response};
    ///
    ///
    /// #[derive(Clone, Copy)]
    /// struct MyErr(isize);
    ///
    ///
    /// fn some_operation() -> Result<isize, MyErr> {
    ///     return Err(MyErr(10));
    /// }
    ///
    ///
    /// fn my_err_handler(_: MyErr) -> PencilResult {
    ///     Ok(Response::from("My err occurred!"))
    /// }
    ///
    ///
    /// fn hello(_: &mut Request) -> PencilResult {
    ///     match some_operation() {
    ///         Ok(_) => Ok(Response::from("Hello!")),
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
    /// use std::convert;
    ///
    /// use pencil::Request;
    /// use pencil::{Pencil, PencilResult, Response};
    /// use pencil::{PencilError, PenUserError, UserError};
    ///
    ///
    /// #[derive(Clone, Copy)]
    /// pub struct MyErr(isize);
    ///
    /// impl convert::From<MyErr> for PencilError {
    ///     fn from(err: MyErr) -> PencilError {
    ///         let user_error = UserError::new("MyErr");
    ///         return PenUserError(user_error);
    ///     }
    /// }
    ///
    ///
    /// fn my_err_handler(_: UserError) -> PencilResult {
    ///     Ok(Response::from("My err occurred!"))
    /// }
    ///
    ///
    /// fn some_operation() -> Result<String, MyErr> {
    ///     return Err(MyErr(10));
    /// }
    ///
    ///
    /// fn hello(_: &mut Request) -> PencilResult {
    ///     let rv = try!(some_operation());
    ///     return Ok(rv.into());
    /// }
    ///
    ///
    /// fn main() {
    ///     let mut app = Pencil::new("/web/demo");
    ///     // Use error description as key to store handlers, really ugly...
    ///     app.usererrorhandler("MyErr", my_err_handler);
    /// }
    /// ```
    pub fn usererrorhandler(&mut self, error_desc: &str, f: UserErrorHandler) {
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
    #[allow(dead_code)]
    fn test_client(&self) -> PencilClient {
        PencilClient::new(self)
    }

    /// Called before the actual request dispatching, you can return value
    /// from here and stop the further request handling.
    fn preprocess_request(&self, request: &mut Request) -> Option<PencilResult> {
        if let Some(module) = self.get_module(request.module_name()) {
            for func in &module.before_request_funcs {
                if let Some(result) = func(request) {
                    return Some(result);
                }
            }
        }
        for func in &self.before_request_funcs {
            if let Some(result) = func(request) {
                return Some(result);
            }
        }
        None
    }

    /// Does the request dispatching.  Matches the URL and returns the return
    /// value of the view.
    fn dispatch_request(&self, request: &mut Request) -> PencilResult {
        if let Some(ref routing_error) = request.routing_error {
            return Err(PenHTTPError(routing_error.clone()));
        }
        if let Some((ref redirect_url, redirect_code)) = request.routing_redirect {
            return redirect(redirect_url, redirect_code);
        }
        if let Some(default_options_response) = self.make_default_options_response(request) {
            return Ok(default_options_response);
        }
        match self.view_functions.get(&request.endpoint().unwrap()) {
            Some(&view_func) => {
                view_func(request)
            },
            None => {
                Err(PenHTTPError(NotFound))
            }
        }
    }

    /// This method is called to create the default `OPTIONS` response.
    fn make_default_options_response(&self, request: &Request) -> Option<Response> {
        if let Some(ref rule) = request.url_rule {
            // if we provide automatic options for this URL and the request
            // came with the OPTIONS method, reply automatically
            if rule.provide_automatic_options && request.method() == Method::Options {
                let url_adapter = request.url_adapter();
                let mut response = Response::new_empty();
                response.headers.set(hyper::header::Allow(url_adapter.allowed_methods()));
                return Some(response);
            }
        }
        None
    }

    /// Get a module by its name.
    fn get_module(&self, module_name: Option<String>) -> Option<&Module> {
        if let Some(name) = module_name {
            self.modules.get(&name)
        } else {
            None
        }
    }

    /// Modify the response object before it's sent to the HTTP server.
    fn process_response(&self, request: &Request, response: &mut Response) {
        if let Some(module) = self.get_module(request.module_name()) {
            for func in module.after_request_funcs.iter().rev() {
                func(response);
            }
        }
        for func in self.after_request_funcs.iter().rev() {
            func(response);
        }
    }

    /// Called after the actual request dispatching.
    fn do_teardown_request(&self, request: &Request, e: Option<&PencilError>) {
        if let Some(module) = self.get_module(request.module_name()) {
            for func in module.teardown_request_funcs.iter().rev() {
                func(e);
            }
        }
        for func in self.teardown_request_funcs.iter().rev() {
            func(e);
        }
    }

    /// This method is called whenever an error occurs that should be handled.
    fn handle_all_error(&self, request: &Request, e: PencilError) -> PencilResult {
        match e {
            PenHTTPError(e) => self.handle_http_error(request, e),
            PenUserError(e) => self.handle_user_error(request, e),
        }
    }

    /// Handles an User error.
    fn handle_user_error(&self, request: &Request, e: UserError) -> PencilResult {
        if let Some(module) = self.get_module(request.module_name()) {
            if let Some(handler) = module.user_error_handlers.get(&e.desc) {
                return handler(e);
            }
        }
        if let Some(handler) = self.user_error_handlers.get(&e.desc) {
            return handler(e);
        }
        Err(PenUserError(e))
    }

    /// Handles an HTTP error.
    fn handle_http_error(&self, request: &Request, e: HTTPError) -> PencilResult {
        if let Some(module) = self.get_module(request.module_name()) {
            if let Some(handler) = module.http_error_handlers.get(&e.code()) {
                return handler(e);
            }
        }
        if let Some(handler) = self.http_error_handlers.get(&e.code()) {
            return handler(e);
        }
        Ok(e.to_response())
    }

    /// Default error handing that kicks in when an error occurs that is not
    /// handled.
    fn handle_error(&self, request: &Request, e: &PencilError) -> Response {
        self.log_error(request, e);
        let internal_server_error = InternalServerError;
        if let Ok(response) = self.handle_http_error(request, internal_server_error) {
            return response;
        } else {
            let e = InternalServerError;
            return e.to_response();
        }
    }

    /// Logs an error.
    fn log_error(&self, request: &Request, e: &PencilError) {
        error!("Error on {} [{}]: {}", request.path(), request.method(), e.description());
    }

    /// Dispatches the request and performs request pre and postprocessing
    /// as well as HTTP error handling and User error handling.
    fn full_dispatch_request(&self, request: &mut Request) -> Result<Response, PencilError> {
        let result = match self.preprocess_request(request) {
            Some(result) => result,
            None => self.dispatch_request(request),
        };
        let rv = match result {
            Ok(response) => Ok(response),
            Err(e) => self.handle_all_error(request, e),
        };
        match rv {
            Ok(mut response) => {
                self.process_response(request, &mut response);
                Ok(response)
            },
            Err(e) => Err(e),
        }
    }

    /// Load and compile and register a template.
    pub fn register_template(&mut self, template_name: &str) {
        let registry_write_rv = self.handlebars_registry.write();
        if registry_write_rv.is_err() {
            panic!("Can't write handlebars registry");
        }
        let mut registry = registry_write_rv.unwrap();
        match load_template(self, template_name) {
            Some(source_rv) => {
                match source_rv {
                    Ok(source) => {
                        if let Err(err) = registry.register_template_string(template_name, source) {
                            panic!(format!("Template compile error: {}", err));
                        }
                    },
                    Err(err) => {
                        panic!(format!("Template {} can't be loaded: {}", template_name, err));
                    }
                }
            },
            None => {
                panic!(format!("Template not found: {}", template_name));
            }
        }
    }

    /// We use `handlebars-rs` as template engine.
    /// Renders a template from the template folder with the given context.
    /// The template name is the name of the template to be rendered.
    /// The context is the variables that should be available in the template.
    pub fn render_template<T: ToJson>(&self, template_name: &str, context: &T) -> PencilResult {
        render_template(self, template_name, context)
    }

    /// We use `handlebars-rs` as template engine.
    /// Renders a template from the given template source string
    /// with the given context.
    /// The source is the sourcecode of the template to be rendered.
    /// The context is the variables that should be available in the template.
    pub fn render_template_string<T: ToJson>(&self, source: &str, context: &T) -> PencilResult {
        render_template_string(self, source, context)
    }

    /// The actual application handler.
    pub fn handle_request(&self, request: &mut Request) -> Response {
        request.match_request();
        match self.full_dispatch_request(request) {
            Ok(response) => {
                self.do_teardown_request(request, None);
                return response;
            },
            Err(e) => {
                let response = self.handle_error(request, &e);
                self.do_teardown_request(request, Some(&e));
                return response;
            }
        };
    }

    /// Runs the application on a hyper HTTP server.
    pub fn run<A: ToSocketAddrs>(self, addr: A) {
        run_server(self, addr);
    }
}

impl hyper::server::Handler for Pencil {
    fn handle(&self, req: HTTPRequest, mut res: HTTPResponse) {
        match Request::new(self, req) {
            Ok(mut request) => {
                let response = self.handle_request(&mut request);
                response.write(request.method(), res);
            }
            Err(_) => {
                *res.status_mut() = StatusCode::BadRequest;
                if let Ok(w) = res.start() {
                    let _ = w.end();
                }
            }
        };
    }
}

impl PathBound for Pencil {
    fn open_resource(&self, resource: &str) -> File {
        let mut pathbuf = PathBuf::from(&self.root_path);
        pathbuf.push(resource);
        File::open(&pathbuf.as_path()).unwrap()
    }
}

impl fmt::Display for Pencil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Pencil application {}>", self.name)
    }
}

impl fmt::Debug for Pencil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Pencil application {}>", self.name)
    }
}

/// View function used internally to send static files from the static folder
/// to the browser.
fn send_app_static_file(request: &mut Request) -> PencilResult {
    let mut static_path = PathBuf::from(&request.app.root_path);
    static_path.push(&request.app.static_folder);
    let static_path_str = static_path.to_str().unwrap();
    let filename = request.view_args.get("filename").unwrap();
    send_from_directory(static_path_str, filename, false)
}
