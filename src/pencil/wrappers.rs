// This module implements simple request and response objects.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::io::net::ip::SocketAddr;
use std::path::BytesContainer;

use http;
use http::server::request::RequestUri::AbsolutePath;
use http::server::ResponseWriter;
use http::headers::request::HeaderCollection as RequestHeaders;
use http::headers::response::HeaderCollection as ResponseHeaders;
use http::headers::HeaderConvertible;
use http::headers::content_type::MediaType;
use url;
use url::form_urlencoded::parse as form_urlencoded_parse;

use app::Pencil;
use datastructures::MultiDict;
use httputils::{get_name_by_http_code, get_content_type};
use serving::get_status_from_code;
use routing::Rule;
use types::ViewArgs;
use errors::{HTTPError, NotFound};


/// Request type.
pub struct Request<'r> {
    pub app: &'r Pencil,
    pub request: http::server::Request,
    url: Option<url::Url>,
    pub url_rule: Option<Rule>,
    pub view_args: ViewArgs,
    pub routing_error: Option<HTTPError>,
    args: Option<MultiDict>,
    form: Option<MultiDict>,
}

impl<'r> Request<'r> {
    /// Create a `Request`.
    pub fn new(app: &Pencil, request: http::server::Request) -> Request {
        let url = match request.request_uri {
            AbsolutePath(ref url) => {
                match request.headers.host {
                    Some(ref host) => {
                        let full_url = String::from_str("http://") + host.http_value().as_slice() +
                            "/" + url.as_slice().trim_left_matches('/');
                        match url::Url::parse(full_url.as_slice()) {
                            Ok(url) => Some(url),
                            Err(_) => None,
                        }
                    },
                    None => None
                }
            },
            _ => None,
        };
        Request {
            app: app,
            request: request,
            url: url,
            url_rule: None,
            view_args: vec![],
            routing_error: None,
            args: None,
            form: None,
        }
    }

    /// Match the request, set the `url_rule` and `view_args` field.
    pub fn match_request(&mut self) {
        match self.path() {
            Some(path) => {
                let url_adapter = self.app.url_map.bind(path, self.method());
                match url_adapter.captures() {
                    Ok(caps) => {
                        let (rule, view_args) = caps;
                        self.url_rule = Some(rule);
                        self.view_args = view_args;
                    },
                    Err(e) => {
                        self.routing_error = Some(e);
                    },
                }
            },
            None => {
                self.routing_error = Some(NotFound);
            }
        }
    }

    /// The endpoint that matched the request.
    pub fn endpoint(&self) -> Option<String> {
        match self.url_rule {
            Some(ref rule) => Some(rule.endpoint.clone()),
            None => None,
        }
    }

    /// The parsed URL parameters.
    pub fn args(&mut self) -> &MultiDict {
        if self.args.is_none() {
            let mut args = MultiDict::new();
            if self.url.is_some() {
                match self.url.as_ref().unwrap().query_pairs() {
                    Some(pairs) => {
                        for &(ref k, ref v) in pairs.iter() {
                            args.add(k.as_slice(), v.as_slice());
                        }
                    },
                    None => (),
                }
            }
            self.args = Some(args);
        }
        return self.args.as_ref().unwrap();
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

    /// The headers.
    pub fn headers(&self) -> &RequestHeaders {
        &self.request.headers
    }

    /// Requested path.
    pub fn path(&self) -> Option<String> {
        if self.url.is_some() {
            match self.url.as_ref().unwrap().serialize_path() {
                Some(path) => {
                    return Some(url::percent_encoding::
                                lossy_utf8_percent_decode(path.container_as_bytes()));
                },
                None => {
                    return None;
                }
            }
        } else {
            return None;
        }
    }

    /// Requested path including the query string.
    pub fn full_path(&self) -> Option<String> {
        let path = self.path();
        let query_string = self.query_string();
        if path.is_some() && query_string.is_some() {
            return Some(path.unwrap() + "?" + query_string.unwrap().as_slice());
        } else  {
            return path;
        }
    }

    /// The host including the port if available.
    pub fn host(&self) -> Option<String> {
        match self.request.headers.host {
            Some(ref host) => Some(host.http_value()),
            None => None,
        }
    }

    /// The URL parameters as raw String.
    pub fn query_string(&self) -> Option<String> {
        if self.url.is_some() {
            return self.url.as_ref().unwrap().query.clone();
        } else {
            return None;
        }
    }

    /// The requested method.
    pub fn method(&self) -> String {
        self.request.method.http_value()
    }

    /// The remote address of the client.
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.request.remote_addr.clone()
    }

    /// URL scheme (http or https), currently I do not know how to get
    /// this, the result will always be http.
    pub fn scheme(&self) -> String {
        String::from_str("http")
    }

    /// Just the host with scheme.
    pub fn host_url(&self) -> Option<String> {
        match self.host() {
            Some(host) => {
                Some(self.scheme() + "://" + host.as_slice() + "/")
            },
            None => None,
        }
    }

    /// The current url.
    pub fn url(&self) -> Option<String> {
        let host_url = self.host_url();
        let full_path = self.full_path();
        if host_url.is_some() && full_path.is_some() {
            Some(host_url.unwrap() + full_path.unwrap().as_slice().trim_left_matches('/'))
        } else {
            None
        }
    }

    /// The current url without the query string.
    pub fn base_url(&self) -> Option<String> {
        let host_url = self.host_url();
        let path = self.path();
        if host_url.is_some() && path.is_some() {
            Some(host_url.unwrap() + path.unwrap().as_slice().trim_left_matches('/'))
        } else {
            None
        }
    }

    /// Whether the request is secure (https).
    pub fn is_secure(&self) -> bool {
        self.scheme() == "https".to_string()
    }
}


/// Response type.  It is just one container with a couple of parameters
/// (headers, body, status code etc).
#[derive(Clone)]
pub struct Response {
    pub status_code: int,
    pub headers: ResponseHeaders,
    pub body: String,
}

impl Response {
    /// Create a `Response`.
    pub fn new(body: String) -> Response {
        let mut response = Response {
            status_code: 200,
            headers: ResponseHeaders::new(),
            body: body,
        };
        response.headers.content_type = Some(MediaType {
            type_ : String::from_str("text"),
            subtype: String::from_str("html"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });
        response.headers.content_length = Some(response.body.len());
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
        let content_length = self.body.len();
        self.set_content_length(content_length);
    }

    /// Returns the response content type if available.
    pub fn content_type(&self) -> Option<MediaType> {
        self.headers.content_type.clone()
    }

    /// Set response content type.
    pub fn set_content_type(&mut self, type_: &str, subtype: &str) {
        self.headers.content_type = Some(get_content_type(type_, subtype, "utf-8"));
    }

    /// Returns the response content length if available.
    pub fn content_length(&self) -> Option<uint> {
        self.headers.content_length.clone()
    }

    /// Set content length.
    pub fn set_content_length(&mut self, value: uint) {
        self.headers.content_length = Some(value);
    }

    /// Sets a cookie(TODO).
    pub fn set_cookie(&mut self) {
    }

    /// Delete a cookie(TODO).
    pub fn delete_cookie(&mut self) {
    }

    /// Return `true` if the response is streamed(currently we do not support
    /// streaming yet, always return `false`).
    pub fn is_streamed(&self) -> bool {
        false
    }

    /// Write the response out.  Mostly you shouldn't use this directly.
    pub fn write(&self, request_method: String, w: &mut ResponseWriter) {
        // write status.
        let status_code = self.status_code;
        w.status = get_status_from_code(status_code);

        // write headers.
        for header in self.headers.iter() {
            w.headers.insert(header);
        }

        // write data.
        if request_method == String::from_str("HEAD") ||
           (100 <= status_code && status_code < 200) ||
           status_code == 204 || status_code == 304 {
            w.write(b"").unwrap();
        } else {
            w.write(self.body.as_bytes()).unwrap();
        }
    }
}
