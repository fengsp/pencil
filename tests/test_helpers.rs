// Test helpers.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

extern crate pencil;

use pencil::{PenHTTPError, PenUserError};
use pencil::{PenString, PenResponse};
use pencil::{abort, redirect};


#[test]
fn test_abort() {
    let result = abort(404);
    let pencil_error = result.err().unwrap();
    let http_error = match pencil_error {
        PenHTTPError(e) => Some(e),
        PenUserError(_) => None,
    };
    assert!(http_error.unwrap().code() == 404);
}


#[test]
fn test_redirect() {
    let result = redirect("/füübär", 302);
    let pencil_value = result.ok().unwrap();
    let response = match pencil_value {
        PenString(_) => None,
        PenResponse(response) => Some(response),
    };
    let response = response.unwrap();
    assert!(response.body.as_slice().contains("/füübär"));
    let location = response.headers.get("Location").unwrap();
    assert!(location.as_slice().contains("/füübär"));
    assert!(response.status_code == 302);

    let result = redirect("http://example.com/", 301);
    let pencil_value = result.ok().unwrap();
    let response = match pencil_value {
        PenString(_) => None,
        PenResponse(response) => Some(response),
    };
    let response = response.unwrap();
    let location = response.headers.get("Location").unwrap();
    assert!(location.as_slice() == "http://example.com/");
    assert!(response.status_code == 301);
}
