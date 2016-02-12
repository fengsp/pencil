// This module implements simple request and response objects.

use std::fmt;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::io::{Read, Write};

use hyper;
use hyper::uri::RequestUri::{AbsolutePath, AbsoluteUri, Authority, Star};
use hyper::header::{Headers, ContentLength, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::method::Method;
use url;
use url::UrlParser;
use url::form_urlencoded::parse as form_urlencoded_parse;

use app::Pencil;
use datastructures::MultiDict;
use httputils::{get_name_by_http_code, get_content_type, get_host_value};
use httputils::get_status_from_code;
use routing::Rule;
use types::ViewArgs;
use http_errors::{HTTPError, NotFound};


/// Request type.
pub struct Request<'r, 'a, 'b: 'a> {
    pub app: &'r Pencil,
    /// The original hyper request object.
    pub request: hyper::server::request::Request<'a, 'b>,
    url: Option<url::Url>,
    /// The URL rule that matched the request.  This is
    /// going to be `None` if nothing matched.
    pub url_rule: Option<Rule>,
    /// A dict of view arguments that matched the request.
    pub view_args: ViewArgs,
    /// If matching the URL failed, this will be the error.
    pub routing_error: Option<HTTPError>,
    args: Option<MultiDict>,
    form: Option<MultiDict>,
}

impl<'r, 'a, 'b: 'a> Request<'r, 'a, 'b> {
    /// Create a `Request`.
    pub fn new(app: &'r Pencil, request: hyper::server::request::Request<'a, 'b>) -> Request<'r, 'a, 'b> {
        let url = match request.uri {
            AbsolutePath(ref url) => {
                let host: Option<&hyper::header::Host> = request.headers.get();
                match host {
                    Some(host) => {
                        let full_url = String::from("http://") + &get_host_value(host) +
                            "/" + &url.trim_left_matches('/');
                        match url::Url::parse(&full_url) {
                            Ok(url) => Some(url),
                            Err(_) => None,
                        }
                    },
                    None => None,
                }
            },
            _ => None,
        };
        Request {
            app: app,
            request: request,
            url: url,
            url_rule: None,
            view_args: HashMap::new(),
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
                match url_adapter.matched() {
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
                            args.add(&k, &v);
                        }
                    },
                    None => (),
                }
            }
            self.args = Some(args);
        }
        return self.args.as_ref().unwrap();
    }

    /// Get content type.
    fn content_type(&self) -> Option<ContentType> {
        let content_type: Option<&ContentType> = self.request.headers.get();
        content_type.map(|c| c.clone())
    }

    /// This method is used internally to retrieve submitted data.
    fn load_form_data(&mut self) {
        if self.form.is_some() {
            return
        }
        let form = match self.content_type() {
            Some(ContentType(Mime(toplevel, sublevel, _))) => {
                if toplevel == TopLevel::Application && sublevel == SubLevel::WwwFormUrlEncoded {
                    let mut body: Vec<u8> = Vec::new();
                    self.request.read_to_end(&mut body).unwrap();
                    let mut form = MultiDict::new();
                    for &(ref k, ref v) in form_urlencoded_parse(&body).iter() {
                        form.add(&k, &v);
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
    pub fn headers(&self) -> &Headers {
        &self.request.headers
    }

    /// Requested path.
    pub fn path(&self) -> Option<String> {
        match self.request.uri {
            AbsolutePath(ref path) => {
                Some(path.splitn(2, '?').next().unwrap().to_string())
            },
            AbsoluteUri(ref url) => {
                url.serialize_path()
            },
            Authority(_) | Star => None
        }
    }

    /// Requested path including the query string.
    pub fn full_path(&self) -> Option<String> {
        let path = self.path();
        let query_string = self.query_string();
        if path.is_some() && query_string.is_some() {
            return Some(path.unwrap() + "?" + &query_string.unwrap());
        } else  {
            return path;
        }
    }

    /// The host including the port if available.
    pub fn host(&self) -> Option<String> {
        let host: Option<&hyper::header::Host> = self.request.headers.get();
        host.map(|host| get_host_value(host))
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
        match self.request.method {
            Method::Options => "OPTIONS".to_string(),
            Method::Get => "GET".to_string(),
            Method::Post => "POST".to_string(),
            Method::Put => "PUT".to_string(),
            Method::Delete => "DELETE".to_string(),
            Method::Head => "HEAD".to_string(),
            Method::Trace => "TRACE".to_string(),
            Method::Connect => "CONNECT".to_string(),
            Method::Patch => "PATCH".to_string(),
            Method::Extension(ref method) => method.clone(),
        }
    }

    /// The remote address of the client.
    pub fn remote_addr(&self) -> SocketAddr {
        self.request.remote_addr.clone()
    }

    /// URL scheme (http or https), currently I do not know how to get
    /// this, the result will always be http.
    pub fn scheme(&self) -> String {
        String::from("http")
    }

    /// Just the host with scheme.
    pub fn host_url(&self) -> Option<String> {
        match self.host() {
            Some(host) => {
                Some(self.scheme() + "://" + &host + "/")
            },
            None => None,
        }
    }

    /// The current url.
    pub fn url(&self) -> Option<String> {
        let host_url = self.host_url();
        let full_path = self.full_path();
        if host_url.is_some() && full_path.is_some() {
            Some(host_url.unwrap() + &(full_path.unwrap()).trim_left_matches('/'))
        } else {
            None
        }
    }

    /// The current url without the query string.
    pub fn base_url(&self) -> Option<String> {
        let host_url = self.host_url();
        let path = self.path();
        if host_url.is_some() && path.is_some() {
            Some(host_url.unwrap() + &path.unwrap().trim_left_matches('/'))
        } else {
            None
        }
    }

    /// Whether the request is secure (https).
    pub fn is_secure(&self) -> bool {
        self.scheme() == "https".to_string()
    }
}

impl<'r, 'a, 'b: 'a> fmt::Debug for Request<'r, 'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.url() {
            Some(url) => {
                write!(f, "Pencil Request '{}' {}", url, self.method())
            },
            None => {
                write!(f, "Pencil Request")
            }
        }
    }
}


/// Response type.  It is just one container with a couple of parameters
/// (headers, body, status code etc).
#[derive(Clone)]
pub struct Response {
    pub status_code: isize,
    pub headers: Headers,
    pub body: String,
}

impl Response {
    /// Create a `Response`.
    pub fn new(body: String) -> Response {
        let mut response = Response {
            status_code: 200,
            headers: Headers::new(),
            body: body,
        };
        let mime: Mime = "text/html; charset=UTF-8".parse().unwrap();
        let content_type = ContentType(mime);
        let content_length = ContentLength(response.body.len() as u64);
        response.headers.set(content_type);
        response.headers.set(content_length);
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
    pub fn content_type(&self) -> Option<&ContentType> {
        self.headers.get()
    }

    /// Set response content type.
    pub fn set_content_type(&mut self, mimetype: &str) {
        let mimetype = get_content_type(mimetype, "UTF-8");
        let mime: Mime = (&mimetype).parse().unwrap();
        let content_type = ContentType(mime);
        self.headers.set(content_type);
    }

    /// Returns the response content length if available.
    pub fn content_length(&self) -> Option<usize> {
        let content_length: Option<&ContentLength> = self.headers.get();
        match content_length {
            Some(&ContentLength(length)) => Some(length as usize),
            None => None,
        }
    }

    /// Set content length.
    pub fn set_content_length(&mut self, value: usize) {
        let content_length = ContentLength(value as u64);
        self.headers.set(content_length);
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
    pub fn write(self, request_method: String, mut res: hyper::server::Response) {
        // write status.
        let status_code = self.status_code;
        *res.status_mut() = get_status_from_code(status_code);

        // write headers.
        *res.headers_mut() = self.headers;

        // write data.
        if request_method == String::from("HEAD") ||
           (100 <= status_code && status_code < 200) ||
           status_code == 204 || status_code == 304 {
            res.headers_mut().set(ContentLength(0));
            try_return!(res.start().and_then(|w| w.end()));
        } else {
            let mut res = try_return!(res.start());
            try_return!(res.write_all(self.body.as_bytes()));
            try_return!(res.end());
        }
    }
}
