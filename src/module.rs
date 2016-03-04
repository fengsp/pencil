//! Modules are the recommended way to implement larger or more
//! pluggable applications.

use std::collections::HashMap;
use std::mem;

use hyper::method::Method;

use app::Pencil;
use routing::Matcher;
use types::ViewFunc;
use types::{BeforeRequestFunc, AfterRequestFunc, TeardownRequestFunc};
use types::{HTTPErrorHandler, UserErrorHandler};
use helpers::send_static_file;


/// Represents a module.
pub struct Module {
    /// The name of the module.
    pub name: String,
    /// The path where your module locates.
    pub root_path: String,
    /// The folder with static files that should be served at `static_url_path`.
    pub static_folder: Option<String>,
    /// The url path for the static files on the web.
    pub static_url_path: Option<String>,
    /// The folder that contains the templates that should be used for the module.
    pub template_folder: Option<String>,
    before_request_funcs: Vec<BeforeRequestFunc>,
    after_request_funcs: Vec<AfterRequestFunc>,
    teardown_request_funcs: Vec<TeardownRequestFunc>,
    http_error_handlers: HashMap<isize, HTTPErrorHandler>,
    user_error_handlers: HashMap<String, UserErrorHandler>,
    deferred_functions: Vec<Box<Fn(&mut Pencil) + Send + Sync>>,
    deferred_routes: Vec<(Matcher, Vec<Method>, String, ViewFunc)>,
}

impl Module {
    pub fn new(name: &str, root_path: &str) -> Module {
        Module {
            name: name.to_string(),
            root_path: root_path.to_string(),
            static_folder: None,
            static_url_path: None,
            template_folder: None,
            before_request_funcs: Vec::new(),
            after_request_funcs: Vec::new(),
            teardown_request_funcs: Vec::new(),
            http_error_handlers: HashMap::new(),
            user_error_handlers: HashMap::new(),
            deferred_functions: Vec::new(),
            deferred_routes: Vec::new(),
        }
    }

    fn record<F: Fn(&mut Pencil) + Send + Sync + 'static>(&mut self, f: F) {
        self.deferred_functions.push(Box::new(f));
    }

    /// The endpoint is automatically prefixed with the module's name.
    pub fn route<M: Into<Matcher>, N: AsRef<[Method]>>(&mut self, rule: M, methods: N, endpoint: &str, view_func: ViewFunc) {
        let mut methods_vec: Vec<Method> = Vec::new();
        methods_vec.extend(methods.as_ref().iter().cloned());
        if endpoint.contains(".") {
            panic!("Module endpoint should not contain dot");
        }
        let endpoint = format!("{}.{}", self.name, endpoint);
        self.deferred_routes.push((rule.into(), methods_vec, endpoint, view_func));
    }

    /// Before request for a module.  This is only executed before each request
    /// that is handled by a view function of that module.
    pub fn before_request(&mut self, f: BeforeRequestFunc) {
        self.before_request_funcs.push(f);
    }

    /// Before request for the app that this module is registered on.  This is
    /// executed before each request, even if outside of a module.
    pub fn before_app_request(&mut self, f: BeforeRequestFunc) {
        self.record(move |app| app.before_request(f));
    }

    /// After request for a module.  This is only executed after each request
    /// that is handled by a view function of that module.
    pub fn after_request(&mut self, f: AfterRequestFunc) {
        self.after_request_funcs.push(f);
    }

    /// After request for the app that this module is registered on.  This is
    /// executed after each request, even if outside of a module.
    pub fn after_app_request(&mut self, f: AfterRequestFunc) {
        self.record(move |app| app.after_request(f));
    }
 
    /// Teardown request for a module.  This is only executed when tearing down
    /// each request that is handled by a view function of that module.
    pub fn teardown_request(&mut self, f: TeardownRequestFunc) {
        self.teardown_request_funcs.push(f);
    }

    /// Teardown request for the app that this module is registered on.  This is
    /// executed when tearing down each request, even if outside of a module.
    pub fn teardown_app_request(&mut self, f: TeardownRequestFunc) {
        self.record(move |app| app.teardown_request(f));
    }

    /// Registers a http error handler that becomes active for this module only.
    pub fn httperrorhandler(&mut self, status_code: isize, f: HTTPErrorHandler) {
        self.http_error_handlers.insert(status_code, f);
    }

    /// Registers an user error handler that becomes active for this module only.
    pub fn usererrorhandler(&mut self, error_desc: &str, f: UserErrorHandler) {
        self.user_error_handlers.insert(error_desc.to_string(), f);
    }

    /// Registers a http error handler for all requests of the application.
    pub fn app_httperrorhandler(&mut self, status_code: isize, f: HTTPErrorHandler) {
        self.record(move |app| app.httperrorhandler(status_code, f));
    }

    /// Registers an user error handler for all requests of the application.
    pub fn app_usererrorhandler(&mut self, error_desc: &str, f: UserErrorHandler) {
        let desc = error_desc.to_string();
        self.record(move |app| app.register_user_error_handler(&desc, f));
    }

    /// Register this module.
    pub fn register(&mut self, app: &mut Pencil) {
        let static_url_path = match self.static_folder {
            Some(_) => {
                match self.static_url_path {
                    Some(ref static_url_path) => Some(static_url_path.clone()),
                    None => None,
                }
            },
            None => None
        };
        if let Some(static_url_path) = static_url_path {
            let mut rule = static_url_path.clone();
            rule = rule + "/<path:filename>";
            // TODO implement send_module_static_file
            self.route(rule, &[Method::Get], "static", send_static_file);
        }
        let deferred_routes = mem::replace(&mut self.deferred_routes, Vec::new());
        for (matcher, methods, endpoint, view_func) in deferred_routes {
            app.add_url_rule(matcher, methods.as_ref(), &endpoint, view_func);
        }
        let deferred_functions = mem::replace(&mut self.deferred_functions, Vec::new());
        for deferred in deferred_functions {
            deferred(app);
        }
    }
}
