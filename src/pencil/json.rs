// This module implements helpers for the JSON support in Pencil.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use core::fmt::Error;
use serialize::json;
use serialize::Encodable;
use serialize::json::Encoder;

use wrappers::{Response};
use types::{PencilResult, PenResponse};


/// Creates a `Response` with the JSON representation of the given object
/// with an *application/json* mimetype.
pub fn jsonify<T>(object: &T) -> PencilResult where T: for<'a> Encodable<Encoder<'a>, Error>{
    let encoded = json::encode(object);
    let mut response = Response::new(encoded);
    response.set_content_type("application", "json");
    return Ok(PenResponse(response));
}
