// This module implements simple request and response objects.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use http::status;
pub use http::server::Request;

use datastructures::Headers;


/// Response type.  It is just one container with a couple of parameters
/// (headers, body, status code etc).
#[deriving(Clone)]
pub struct Response {
    status: status::Status,
    pub headers: Headers,
    pub body: String,
}

impl Response {
    /// Create a `Response`.
    pub fn new(body: String) -> Response {
        let mut response = Response {
            status: status::Ok,
            headers: Headers::new(None),
            body: body,
        };
        let content_length = response.body.len().to_string();
        response.headers.set("Content-Type", "text/html; charset=utf-8");
        response.headers.set("Content-Length", content_length.as_slice());
        return response;
    }

    pub fn status_code(&self) -> u16 {
        self.status.code()
    }

    pub fn set_status_code(&mut self, code: u16) {
        match FromPrimitive::from_u64(code as u64) {
            Some(status) => { self.status = status; },
            None => { self.status = status::UnregisteredStatus(code as u16, String::from_str("UNKNOWN")); },
        }
    }

    pub fn status(&self) -> status::Status {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: status::Status) {
        self.status = status;
    }

    /// Sets a new string as response body.  The content length is set
    /// automatically.
    pub fn set_data(&mut self, value: String) {
        self.body = value;
        let content_length = self.body.len().to_string();
        self.headers.set("Content-Length", content_length.as_slice());
    }

    /// Returns the response content type if available.
    pub fn content_type(&self) -> Option<String> {
        let rv = self.headers.get("Content-Type");
        rv.map(|content_type| content_type.clone())
    }

    /// Set response content type.
    pub fn set_content_type(&mut self, value: &str) {
        self.headers.set("Content-Type", value.to_string().as_slice());
    }

    /// Returns the response content length if available.
    pub fn content_length(&self) -> Option<uint> {
        let rv = self.headers.get("Content-Length");
        match rv {
            Some(content_length) => from_str(content_length.as_slice()),
            None => None,
        }
    }

    /// Set content length.
    pub fn set_content_length(&mut self, value: uint) {
        self.headers.set("Content-Length", value.to_string().as_slice());
    }
}
