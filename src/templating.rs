//! This module implements the bridge to handlebars.
use std::convert;
use std::io::Read;
use std::io::Write;
use std::io::Result as IOResult;
use std::fs::File;
use std::path::PathBuf;

use rustc_serialize::json::ToJson;
use handlebars::RenderError;
use handlebars::Context;
use handlebars::RenderContext;
use handlebars::Template;
use handlebars::Renderable;

use app::Pencil;
use types::{PencilResult, PenUserError, UserError, PencilError};
use wrappers::Response;


impl convert::From<RenderError> for PencilError {
    fn from(err: RenderError) -> PencilError {
        return PenUserError(UserError::new(&err.desc));
    }
}

pub fn render_template<T: ToJson>(app: &Pencil, template_name: &str, context: &T) -> PencilResult {
    let registry_read_rv = app.handlebars_registry.read();
    if registry_read_rv.is_err() {
        return Err(PenUserError(UserError::new("Can't acquire handlebars registry")));
    }
    let registry = registry_read_rv.unwrap();
    let rv = try!(registry.render(template_name, context));
    return Ok(Response::from(rv));
}

struct StringWriter {
    buf: Vec<u8>
}

impl StringWriter {
    pub fn new() -> StringWriter {
        StringWriter {
            buf: Vec::new()
        }
    }

    pub fn to_string(self) -> String {
        if let Ok(s) = String::from_utf8(self.buf) {
            s
        } else {
            String::new()
        }
    }
}

impl Write for StringWriter {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        for b in buf {
            self.buf.push(*b);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> IOResult<()> {
        Ok(())
    }
}

pub fn render_template_string<T: ToJson>(app: &Pencil, source: &str, context: &T) -> PencilResult {
    match app.handlebars_registry.read() {
        Ok(registry) => {
            match Template::compile(source.to_owned()) {
                Ok(template) => {
                    let c = Context::wraps(context);
                    let mut writer = StringWriter::new();
                    {
                        let mut render_context = RenderContext::new(&mut writer);
                        try!(template.render(&c, &**registry, &mut render_context));
                    }
                    Ok(Response::from(writer.to_string()))
                },
                Err(err) => {
                    Err(PenUserError(UserError::new(format!("Template compile error: {}", err))))
                }
            }
        },
        Err(_) => {
            Err(PenUserError(UserError::new("Can't acquire handlebars registry")))
        }
    }
}

/// The template loader trait allows for loading template source.
trait TemplateLoader {
    /// Get the template source for a template name.
    fn get_source(&self, template_name: &str) -> Option<IOResult<String>>;
}

/// A template loader that loads templates from the file system.
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
    return None;
}
