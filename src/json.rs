// This module implements helpers for the JSON support in Pencil.

use rustc_serialize::json;
use rustc_serialize::Encodable;

use wrappers::{Response};
use types::{PencilResult, PenResponse, PenUserError, UserError};


/// Creates a view result with the JSON representation of the given object
/// with an *application/json* mimetype. Example usage:
///
/// ```ignore
/// extern crate rustc_serialize;
///
/// use pencil::{Request, PencilResult, jsonify};
///
/// #[derive(RustcEncodable)]
/// struct User {
///     id: u8,
///     name: String,
/// }
///
/// fn get_user(_: &Request) -> PencilResult {
///     let user = User {
///         id: 1,
///         name: String::from("admin"),
///     };
///     return jsonify(&user);
/// }
/// ```
pub fn jsonify<T: Encodable>(object: &T) -> PencilResult {
    match json::encode(object) {
        Ok(encoded) => {
            let mut response = Response::new(encoded);
            response.set_content_type("application/json");
            return Ok(PenResponse(response));
        },
        Err(err) => {
            let error = UserError::new(format!("Json encoder error: {}", err));
            return Err(PenUserError(error));
        },
    }
}
