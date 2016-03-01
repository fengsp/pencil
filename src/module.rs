//! Modules are the recommended way to implement larger or more
//! pluggable applications.

use std::collections::HashMap;

use hyper::method::Method;

use app::Pencil;
use routing::Matcher;
use types::ViewFunc;
use types::{BeforeRequestFunc, AfterRequestFunc, TeardownRequestFunc};
use types::{HTTPErrorHandler, UserErrorHandler};


/// Represents a module.
pub struct Module {
    /// The name of the module.
    pub name: String,
    /// The path where your module locates.
    pub root_path: String,
    /// The url prefix.
    pub url_prefix: String,
    /// The folder with static files that should be served at `static_url_path`.
    /// Defaults to the `"static"` folder in the root path of the module.
    pub static_folder: Option<String>,
    /// The url path for the static files on the web.
    pub static_url_path: Option<String>,
    /// The folder that contains the templates that should be used for the module.
    /// Defaults to `''templates''` folder in the root path of the module.
    pub template_folder: Option<String>,
    before_request_funcs: Vec<BeforeRequestFunc>,
    after_request_funcs: Vec<AfterRequestFunc>,
    teardown_request_funcs: Vec<TeardownRequestFunc>,
    http_error_handlers: HashMap<isize, HTTPErrorHandler>,
    user_error_handlers: HashMap<String, UserErrorHandler>,
    deferred_functions: Vec<Box<Fn(&mut Pencil)>>,
}

impl Module {
    pub fn new(name: &str, root_path: &str, url_prefix: &str) -> Module {
        Module {
            name: name.to_string(),
            root_path: root_path.to_string(),
            url_prefix: url_prefix.to_string(),
            static_folder: None,
            static_url_path: None,
            template_folder: None,
            before_request_funcs: Vec::new(),
            after_request_funcs: Vec::new(),
            teardown_request_funcs: Vec::new(),
            http_error_handlers: HashMap::new(),
            user_error_handlers: HashMap::new(),
            deferred_functions: Vec::new(),
        }
    }

    fn record<F: Fn(&mut Pencil) + 'static>(&mut self, f: F) {
        self.deferred_functions.push(Box::new(f));
    }

    pub fn route<M: Into<Matcher>>(&mut self, rule: M, methods: &[Method], endpoint: &str, view_func: ViewFunc) {
        if endpoint.contains(".") {
            panic!("Module endpoint should not contain dot");
        }
        // self.record(|app| app.add_url_rule(rule, methods, endpoint, view_func));
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
}
