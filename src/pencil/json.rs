// This module implements helpers for the JSON support in Pencil.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error::Error;
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
        Err(e) => {
            let error = UserError::new("Json encoder error!");
            return Err(PenUserError(error));
        },
    }
}
