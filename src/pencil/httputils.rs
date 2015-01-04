// This module implements a bunch of utilities that help Pencil
// to deal with HTTP data.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use http::headers::content_type::MediaType;


/// Get HTTP status name by status code.
pub fn get_name_by_http_code(code: int) -> Option<&'static str> {
    match code {
        100 => Some("Continue"),
        101 => Some("Switching Protocols"),
        102 => Some("Processing"),
        200 => Some("OK"),
        201 => Some("Created"),
        202 => Some("Accepted"),
        203 => Some("Non Authoritative Information"),
        204 => Some("No Content"),
        205 => Some("Reset Content"),
        206 => Some("Partial Content"),
        207 => Some("Multi Status"),
        226 => Some("IM Used"),
        300 => Some("Multiple Choices"),
        301 => Some("Moved Permanently"),
        302 => Some("Found"),
        303 => Some("See Other"),
        304 => Some("Not Modified"),
        305 => Some("Use Proxy"),
        307 => Some("Temporary Redirect"),
        400 => Some("Bad Request"),
        401 => Some("Unauthorized"),
        402 => Some("Payment Required"),
        403 => Some("Forbidden"),
        404 => Some("Not Found"),
        405 => Some("Method Not Allowed"),
        406 => Some("Not Acceptable"),
        407 => Some("Proxy Authentication Required"),
        408 => Some("Request Timeout"),
        409 => Some("Conflict"),
        410 => Some("Gone"),
        411 => Some("Length Required"),
        412 => Some("Precondition Failed"),
        413 => Some("Request Entity Too Large"),
        414 => Some("Request URI Too Long"),
        415 => Some("Unsupported Media Type"),
        416 => Some("Requested Range Not Satisfiable"),
        417 => Some("Expectation Failed"),
        418 => Some("I'm a teapot"),
        422 => Some("Unprocessable Entity"),
        423 => Some("Locked"),
        424 => Some("Failed Dependency"),
        426 => Some("Upgrade Required"),
        428 => Some("Precondition Required"),
        429 => Some("Too Many Requests"),
        431 => Some("Request Header Fields Too Large"),
        449 => Some("Retry With"),
        500 => Some("Internal Server Error"),
        501 => Some("Not Implemented"),
        502 => Some("Bad Gateway"),
        503 => Some("Service Unavailable"),
        504 => Some("Gateway Timeout"),
        505 => Some("HTTP Version Not Supported"),
        507 => Some("Insufficient Storage"),
        510 => Some("Not Extended"),
        _ => None
    }
}


/// Return the full content type with charset for a mimetype.
pub fn get_content_type(type_: &str, subtype: &str, charset: &str) -> MediaType {
    let mut content_type = MediaType {
        type_ : String::from_str(type_),
        subtype: String::from_str(subtype),
        parameters: vec!()
    };
    if type_ == "text" || (type_ == "application" && subtype == "xml") ||
       (type_ == "application" && subtype.ends_with("+xml")) {
        content_type.parameters = vec!((String::from_str("charset"), charset.to_string()));
    }
    return content_type;
}
