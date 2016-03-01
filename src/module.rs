//! Modules are the recommended way to implement larger or more
//! pluggable applications.

use app::Pencil;


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
            deferred_functions: Vec::new(),
        }
    }

    pub fn record<F: Fn(&mut Pencil) + 'static>(&mut self, f: F) {
        self.deferred_functions.push(Box::new(f));
    }
}
