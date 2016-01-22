// This module implements the bridge to handlebars.
use std::convert;

use serialize::json::ToJson;
use handlebars::RenderError;
use handlebars::Handlebars;

use app::Pencil;
use types::{PencilResult, PenString, PenUserError, UserError, PencilError};


impl convert::From<RenderError> for PencilError {
    fn from(err: RenderError) -> PencilError {
        return PenUserError(UserError::new(&err.desc));
    }
}


pub fn render_template<T: ToJson>(app: &mut Pencil, template_name: &str, context: &T) -> PencilResult {
    let registry = app.handlebars_registry.read().unwrap();
    match registry.get_template(template_name) {
        Some(_) => {
            let rv = try!(registry.render(template_name, context));
            Ok(PenString(rv))
        },
        None => {
            Err(PenUserError(UserError::new(format!("Template not found: {}", template_name))))
        }
    }
}
