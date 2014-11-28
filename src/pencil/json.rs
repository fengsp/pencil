// This module implements helpers for the JSON support in Pencil.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::io::IoError;
use serialize::json;
use serialize::Encodable;
use serialize::json::Encoder;

use wrappers::{Response};
use types::{PencilResult, PenResponse};


/// Creates a `Response` with the JSON representation of the given object
/// with an *application/json* mimetype.
pub fn jsonify<'a, T: Encodable<Encoder<'a>, IoError>>(object: &T) -> PencilResult {
    let encoded = json::encode(object);
    let mut response = Response::new(encoded);
    response.set_content_type("application/json");
    return PenResponse(response);
}
