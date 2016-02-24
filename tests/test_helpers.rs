// Test helpers.

extern crate pencil;
extern crate url;
extern crate hyper;

use std::path::PathBuf;

use hyper::header::Location;

use pencil::{PenHTTPError, PenUserError};
use pencil::{PenString, PenResponse};
use pencil::{abort, redirect, safe_join, escape};


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
    let result = redirect("http://localhost/füübär", 302);
    let pencil_value = result.ok().unwrap();
    let response = match pencil_value {
        PenString(_) => None,
        PenResponse(response) => Some(response),
    };
    let response = response.unwrap();
    let location: Option<&Location> = response.headers.get();
    let location_str = url::percent_encoding::lossy_utf8_percent_decode(location.unwrap().as_bytes());
    assert!(location_str.contains("/füübär"));
    assert!(response.status_code == 302);

    let result = redirect("http://example.com/", 301);
    let pencil_value = result.ok().unwrap();
    let response = match pencil_value {
        PenString(_) => None,
        PenResponse(response) => Some(response),
    };
    let response = response.unwrap();
    let location: Option<&Location> = response.headers.get();
    assert!(*location.unwrap() == Location("http://example.com/".to_owned()));
    assert!(response.status_code == 301);
}


#[test]
fn test_safe_join() {
    let path = safe_join("foo", "bar/baz").unwrap();
    assert!(path == PathBuf::from("foo/bar/baz"));
    assert!(safe_join("foo", "../bar/baz").is_none());
}


#[test]
fn test_escape() {
    assert!(escape(String::from("42")) == "42");
    assert!(escape(String::from("<>")) == "&lt;&gt;");
    assert!(escape(String::from("\"foo\"")) == "&quot;foo&quot;");
}
