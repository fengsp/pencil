// This module implements various helpers.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error::Error;

use wrappers::Response;
use types::{
    PencilResult,
        PenValue,
        PenResponse,
        PenError,
};


/// Sometimes it is necessary to set additional headers in a view.  Because
/// views do not have to return `Response` objects but can return a value that
/// is converted into a response by Pencil.  You can call this function to
/// get a response object which you can use to attach headers:
///
///```ignore
///fn index() {
///    let mut response = make_response(PencilValue(String::from_str("Hello!")));
///    response.headers.set(String::from_str("X-TEST"), String::from_str("value"));
///    return response;
///}
///```
///
pub fn make_response(rv: PencilResult) -> Response {
    match rv {
        PenValue(rv) => Response::new(rv),
        PenResponse(response) => response,
        PenError(e) => Response::new(e.description().to_string()),
    }
}
