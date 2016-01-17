// This module implements helpers for the JSON support in Pencil.

use serialize::json;
use serialize::Encodable;

use wrappers::{Response};
use types::{PencilResult, PenResponse, PenUserError, UserError};


/// Creates a `Response` with the JSON representation of the given object
/// with an *application/json* mimetype.
pub fn jsonify<T: Encodable>(object: &T) -> PencilResult {
    match json::encode(object) {
        Ok(encoded) => {
            let mut response = Response::new(encoded);
            response.set_content_type("application/json");
            return Ok(PenResponse(response));
        },
        Err(_) => {
            let error = UserError::new("Json encoder error!");
            return Err(PenUserError(error));
        },
    }
}
