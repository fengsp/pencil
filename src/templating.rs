//! This module implements the bridge to handlebars.
use std::convert;
use std::io::Read;
use std::io::Result as IOResult;
use std::fs::File;
use std::path::PathBuf;
use std::error::Error;

use rustc_serialize::json::ToJson;
use handlebars::{RenderError, TemplateRenderError};

use app::Pencil;
use types::{PencilResult, PenUserError, UserError, PencilError};
use wrappers::Response;

impl convert::From<RenderError> for PencilError {
    fn from(err: RenderError) -> PencilError {
        PenUserError(UserError::new(err.description()))
    }
}

impl convert::From<TemplateRenderError> for PencilError {
    fn from(err: TemplateRenderError) -> PencilError {
        PenUserError(UserError::new(err.description()))
    }
}

pub fn render_template<T: ToJson>(app: &Pencil, template_name: &str, context: &T) -> PencilResult {
    let registry_read_rv = app.handlebars_registry.read();
    if registry_read_rv.is_err() {
        return Err(PenUserError(UserError::new("Can't acquire handlebars registry")));
    }
    let registry = registry_read_rv.unwrap();
    let rv = try!(registry.render(template_name, context));
    Ok(Response::from(rv))
}

pub fn render_template_string<T: ToJson>(app: &Pencil, source: &str, context: &T) -> PencilResult {
    let registry_read_rv = app.handlebars_registry.read();
    if registry_read_rv.is_err() {
        return Err(PenUserError(UserError::new("Can't acquire handlebars registry")));
    }
    let registry = registry_read_rv.unwrap();
    let rv = try!(registry.template_render(source, context));
    Ok(Response::from(rv))
}

/// The template loader trait allows for loading template source.
trait TemplateLoader {
    /// Get the template source for a template name.
    fn get_source(&self, template_name: &str) -> Option<IOResult<String>>;
}

/// A template loader that loads templates from the file system.
#[derive(Debug)]
pub struct FileSystemLoader {
    search_path: String,
}

impl FileSystemLoader {
    /// Create one file system loader.
    /// This loader can find templates in folders on the file system.
    ///
    /// The loader takes the path to the templates:
    ///
    /// ```ignore
    /// let loader = FileSystemLoader::new("/path/to/templates");
    /// let source = loader.get_source("index.html");
    /// ```
    pub fn new(search_path: &str) -> FileSystemLoader {
        FileSystemLoader {
            search_path: search_path.to_string(),
        }
    }
}

impl TemplateLoader for FileSystemLoader {
    fn get_source(&self, template_name: &str) -> Option<IOResult<String>> {
        let mut pathbuf = PathBuf::from(&self.search_path);
        pathbuf.push(template_name);
        match File::open(&pathbuf.as_path()) {
            Ok(mut file) => {
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Ok(_) => Some(Ok(s)),
                    Err(err) => Some(Err(err)),
                }
            },
            Err(_) => None
        }
    }
}

pub fn load_template(app: &Pencil, template_name: &str) -> Option<IOResult<String>> {
    let mut template_path = PathBuf::from(&app.root_path);
    template_path.push(&app.template_folder);
    let template_loader = FileSystemLoader::new(template_path.to_str().unwrap());
    if let Some(source) = template_loader.get_source(template_name) {
        return Some(source);
    }
    for module in app.modules.values() {
        if let Some(ref module_template_folder) = module.template_folder {
            let mut template_path = PathBuf::from(&module.root_path);
            template_path.push(module_template_folder);
            let template_loader = FileSystemLoader::new(template_path.to_str().unwrap());
            if let Some(source) = template_loader.get_source(template_name) {
                return Some(source);
            }
        }
    }
    None
}
