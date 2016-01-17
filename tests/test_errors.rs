// Test the HTTP errors.

extern crate pencil;
extern crate hyper;

use pencil::NotFound;
use hyper::header::ContentType;


#[test]
fn test_http_error_code() {
    let error = NotFound;
    assert!(error.code() == 404);
}


#[test]
fn test_http_error_name() {
    let error = NotFound;
    assert!(error.name() == "Not Found");
}


#[test]
fn test_http_error_get_body() {
    let error = NotFound;
    assert!(error.get_body() == String::from(
"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 3.2 Final//EN\">
<title>404 Not Found</title>
<h1>Not Found</h1>
<p>The requested URL was not found on the server.  If you entered the \
URL manually please check your spelling and try again.</p>
"));
}


#[test]
fn test_http_error_to_response() {
    let error = NotFound;
    let response = error.to_response();
    assert!(response.status_code == 404);
    assert!(*response.content_type().unwrap() ==
            ContentType::html());
}
