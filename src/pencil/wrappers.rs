// This module implements simple request and response objects.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use http;
use http::server::request::RequestUri::AbsolutePath;
use url::Url;
use url::form_urlencoded::parse as form_urlencoded_parse;

use datastructures::{Headers, MultiDict};
use httputils::{get_name_by_http_code, get_content_type};


/// Request type.
pub struct Request {
    pub request: http::server::Request,
    form: Option<MultiDict>,
}

impl Request {
    /// Create a `Request`.
    pub fn new(request: http::server::Request) -> Request {
        Request {
            request: request,
            form: None,
        }
    }

    /// The parsed URL parameters.
    pub fn args(&self) -> MultiDict {
        let mut args = MultiDict::new();
        match self.request.request_uri {
            AbsolutePath(ref url) => {
                let url = Url::parse(url.as_slice()).unwrap();
                match url.query_pairs() {
                    Some(pairs) => {
                        for &(ref k, ref v) in pairs.iter() {
                            args.add(k.as_slice(), v.as_slice());
                        }
                    },
                    None => (),
                }
            },
            _ => (),
        }
        return args;
    }

    /// This method is used internally to retrieve submitted data.
    fn load_form_data(&mut self) {
        if self.form.is_some() {
            return
        }
        let form = match self.request.headers.content_type {
            Some(ref content_type) => {
                if content_type.type_ == String::from_str("application") &&
                    (content_type.subtype == String::from_str("x-www-form-urlencoded") ||
                     content_type.subtype == String::from_str("x-url-encoded")) {
                    let mut form = MultiDict::new();
                    for &(ref k, ref v) in form_urlencoded_parse(self.request.body.as_slice()).iter() {
                        form.add(k.as_slice(), v.as_slice());
                    }
                    form
                } else {
                    MultiDict::new()
                }
            },
            None => {
                MultiDict::new()
            }
        };
        self.form = Some(form);
    }

    /// The form parameters.
    pub fn form(&mut self) -> &MultiDict {
        self.load_form_data();
        self.form.as_ref().unwrap()
    }
}


/// Response type.  It is just one container with a couple of parameters
/// (headers, body, status code etc).
#[deriving(Clone)]
pub struct Response {
    pub status_code: int,
    pub headers: Headers,
    pub body: String,
}

impl Response {
    /// Create a `Response`.
    pub fn new(body: String) -> Response {
        let mut response = Response {
            status_code: 200,
            headers: Headers::new(None),
            body: body,
        };
        let content_length = response.body.len().to_string();
        response.headers.set("Content-Type", "text/html; charset=utf-8");
        response.headers.set("Content-Length", content_length.as_slice());
        return response;
    }

    /// Get status name.
    pub fn status_name(&self) -> &str {
        match get_name_by_http_code(self.status_code) {
            Some(name) => name,
            None => "UNKNOWN",
        }
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
        self.headers.set("Content-Type", get_content_type(value, "utf-8").as_slice());
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
