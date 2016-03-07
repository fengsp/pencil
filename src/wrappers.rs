//! This module implements simple request and response objects.

use std::fmt;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::io;
use std::fs::File;
use std::io::{Read, Write};
use std::convert;

use hyper;
use hyper::uri::RequestUri::{AbsolutePath, AbsoluteUri, Authority, Star};
use hyper::header::{Headers, ContentLength, ContentType, Cookie};
use hyper::mime::Mime;
use hyper::method::Method;
use url::UrlParser;
use url::form_urlencoded;
use formdata::uploaded_file::UploadedFile;
use rustc_serialize::json;
use typemap::TypeMap;

use app::Pencil;
use datastructures::MultiDict;
use httputils::{get_name_by_http_code, get_content_type, get_host_value};
use httputils::get_status_from_code;
use routing::Rule;
use types::ViewArgs;
use http_errors::{HTTPError, NotFound};
use formparser::FormDataParser;


/// Request type.
pub struct Request<'r, 'a, 'b: 'a> {
    pub app: &'r Pencil,
    /// The original hyper request object.
    pub request: hyper::server::request::Request<'a, 'b>,
    /// The URL rule that matched the request.  This is
    /// going to be `None` if nothing matched.
    pub url_rule: Option<Rule>,
    /// A dict of view arguments that matched the request.
    pub view_args: ViewArgs,
    /// If matching the URL failed, this will be the error.
    pub routing_error: Option<HTTPError>,
    /// Storage for data of extensions.
    pub extensions_data: TypeMap,
    args: Option<MultiDict<String>>,
    form: Option<MultiDict<String>>,
    files: Option<MultiDict<UploadedFile>>,
    cached_json: Option<Option<json::Json>>
}

impl<'r, 'a, 'b: 'a> Request<'r, 'a, 'b> {
    /// Create a `Request`.
    pub fn new(app: &'r Pencil, request: hyper::server::request::Request<'a, 'b>) -> Request<'r, 'a, 'b> {
        Request {
            app: app,
            request: request,
            url_rule: None,
            view_args: HashMap::new(),
            routing_error: None,
            extensions_data: TypeMap::new(),
            args: None,
            form: None,
            files: None,
            cached_json: None,
        }
    }

    /// Match the request, set the `url_rule` and `view_args` field.
    pub fn match_request(&mut self) {
        match self.path() {
            Some(path) => {
                let url_adapter = self.app.url_map.bind(path, self.method());
                match url_adapter.matched() {
                    Ok((rule, view_args)) => {
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

    /// The current module name.
    pub fn module_name(&self) -> Option<String> {
        if let Some(endpoint) = self.endpoint() {
            if endpoint.contains(".") {
                let v: Vec<&str> = endpoint.rsplitn(2, '.').collect();
                return Some(v[1].to_string());
            }
        }
        None
    }

    /// The parsed URL parameters.
    pub fn args(&mut self) -> &MultiDict<String> {
        if self.args.is_none() {
            let mut args = MultiDict::new();
            let query_pairs = self.query_string().map(|query| form_urlencoded::parse(query.as_bytes()));
            match query_pairs {
                Some(pairs) => {
                    for (k, v) in pairs {
                        args.add(k, v);
                    }
                },
                None => {},
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

    /// Parses the incoming JSON request data.
    pub fn get_json(&mut self) -> &Option<json::Json> {
        if self.cached_json.is_none() {
            let mut data = String::from("");
            let rv = match self.request.read_to_string(&mut data) {
                Ok(_) => {
                    match json::Json::from_str(&data) {
                        Ok(json) => Some(json),
                        Err(_) => None
                    }
                },
                Err(_) => {
                    None
                }
            };
            self.cached_json = Some(rv);
        }
        return self.cached_json.as_ref().unwrap();
    }

    /// This method is used internally to retrieve submitted data.
    fn load_form_data(&mut self) {
        if self.form.is_some() {
            return
        }
        let (form, files) = match self.content_type() {
            Some(ContentType(mimetype)) => {
                let parser = FormDataParser::new();
                parser.parse(&mut self.request, &mimetype)
            },
            None => {
                (MultiDict::new(), MultiDict::new())
            }
        };
        self.form = Some(form);
        self.files = Some(files);
    }

    /// The form parameters.
    pub fn form(&mut self) -> &MultiDict<String> {
        self.load_form_data();
        self.form.as_ref().unwrap()
    }

    /// All uploaded files.
    pub fn files(&mut self) -> &MultiDict<UploadedFile> {
        self.load_form_data();
        self.files.as_ref().unwrap()
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

    /// The query string.
    pub fn query_string(&self) -> Option<String> {
        match self.request.uri {
            AbsolutePath(ref path) => {
                match UrlParser::new().parse_path(path) {
                    Ok((_, query, _)) => {
                        query
                    },
                    Err(_) => None
                }
            },
            AbsoluteUri(ref url) => {
                url.query.clone()
            },
            Authority(_) | Star => None
        }
    }

    /// The retrieved cookies.
    pub fn cookies(&self) -> Option<&Cookie> {
        self.request.headers.get()
    }

    /// The request method.
    pub fn method(&self) -> Method {
        self.request.method.clone()
    }

    /// The remote address of the client.
    pub fn remote_addr(&self) -> SocketAddr {
        self.request.remote_addr.clone()
    }

    /// URL scheme (http or https)
    pub fn scheme(&self) -> String {
        match self.request.uri {
            AbsoluteUri(ref url) => {
                return url.scheme.clone();
            },
            _ => {}
        }
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
                write!(f, "<Pencil Request '{}' {}>", url, self.method())
            },
            None => {
                write!(f, "<Pencil Request>")
            }
        }
    }
}

impl<'r, 'a, 'b: 'a> Read for Request<'r, 'a, 'b> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.request.read(buf)
    }
}


/// The response body.
pub struct ResponseBody<'a>(Box<Write + 'a>);

impl<'a> ResponseBody<'a> {
    /// Create a new ResponseBody.
    pub fn new<W: Write + 'a>(writer: W) -> ResponseBody<'a> {
        ResponseBody(Box::new(writer))
    }
}

impl<'a> Write for ResponseBody<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}


/// A trait which writes the body of one response.
pub trait BodyWrite: Send {
    fn write_body(&mut self, body: &mut ResponseBody) -> io::Result<()>;
}

impl BodyWrite for Vec<u8> {
    fn write_body(&mut self, body: &mut ResponseBody) -> io::Result<()> {
        body.write_all(self)
    }
}

impl<'a> BodyWrite for &'a [u8] {
    fn write_body(&mut self, body: &mut ResponseBody) -> io::Result<()> {
        body.write_all(self)
    }
}

impl BodyWrite for String {
    fn write_body(&mut self, body: &mut ResponseBody) -> io::Result<()> {
        self.as_bytes().write_body(body)
    }
}

impl<'a> BodyWrite for &'a str {
    fn write_body(&mut self, body: &mut ResponseBody) -> io::Result<()> {
        self.as_bytes().write_body(body)
    }
}

impl BodyWrite for File {
    fn write_body(&mut self, body: &mut ResponseBody) -> io::Result<()> {
        io::copy(self, body).map(|_| ())
    }
}


/// Response type.  It is just one container with a couple of parameters
/// (headers, body, status code etc).
pub struct Response {
    /// The HTTP Status code number
    pub status_code: isize,
    pub headers: Headers,
    pub body: Option<Box<BodyWrite>>,
}

impl Response {
    /// Create a `Response`.  By default, the status code is 200
    /// and content type is "text/html; charset=UTF-8".
    /// Remember to set content length if necessary.
    /// Mostly you should just get a response that is converted
    /// from other types, which set the content length automatically.
    /// For example:
    ///
    /// ```rust,ignore
    /// // Content length is set automatically
    /// let response = Response::from("Hello");
    /// ```
    pub fn new<T: 'static + BodyWrite>(body: T) -> Response {
        let mut response = Response {
            status_code: 200,
            headers: Headers::new(),
            body: Some(Box::new(body)),
        };
        let mime: Mime = "text/html; charset=UTF-8".parse().unwrap();
        let content_type = ContentType(mime);
        response.headers.set(content_type);
        return response;
    }

    /// Create an empty response without body.
    pub fn new_empty() -> Response {
        Response {
            status_code: 200,
            headers: Headers::new(),
            body: None,
        }
    }

    /// Get status name.
    pub fn status_name(&self) -> &str {
        match get_name_by_http_code(self.status_code) {
            Some(name) => name,
            None => "UNKNOWN",
        }
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

    /// Sets cookie.
    pub fn set_cookie(&mut self, cookie: hyper::header::SetCookie) {
        self.headers.set(cookie);
    }

    /// Write the response out.  Mostly you shouldn't use this directly.
    #[doc(hidden)]
    pub fn write(self, request_method: Method, mut res: hyper::server::Response) {
        // write status.
        let status_code = self.status_code;
        *res.status_mut() = get_status_from_code(status_code);

        // write headers.
        *res.headers_mut() = self.headers;

        // write data.
        if request_method == Method::Head ||
           (100 <= status_code && status_code < 200) || status_code == 204 || status_code == 304 {
            res.headers_mut().set(ContentLength(0));
            try_return!(res.start().and_then(|w| w.end()));
        } else {
            match self.body {
                Some(mut body) => {
                    let mut res = try_return!(res.start());
                    try_return!(body.write_body(&mut ResponseBody::new(&mut res)));
                    try_return!(res.end());
                },
                None => {
                    res.headers_mut().set(ContentLength(0));
                    try_return!(res.start().and_then(|w| w.end()));
                }
            }
        }
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Pencil Response [{}]>", self.status_code)
    }
}

impl convert::From<Vec<u8>> for Response {
    /// Convert to response body.  The content length is set
    /// automatically.
    fn from(bytes: Vec<u8>) -> Response {
        let content_length = bytes.len();
        let mut response = Response::new(bytes);
        response.set_content_length(content_length);
        return response;
    }
}

impl<'a> convert::From<&'a [u8]> for Response {
    /// Convert to response body.  The content length is set
    /// automatically.
    fn from(bytes: &'a [u8]) -> Response {
        bytes.to_vec().into()
    }
}

impl<'a> convert::From<&'a str> for Response {
    /// Convert to response body.  The content length is set
    /// automatically.
    fn from(s: &'a str) -> Response {
        s.to_owned().into()
    }
}

impl convert::From<String> for Response {
    /// Convert a new string to response body.  The content length is set
    /// automatically.
    fn from(s: String) -> Response {
        s.into_bytes().into()
    }
}

impl convert::From<File> for Response {
    /// Convert to response body.  The content length is set
    /// automatically if file size is available from metadata.
    fn from(f: File) -> Response {
        let content_length = match f.metadata() {
            Ok(metadata) => {
                Some(metadata.len())
            },
            Err(_) => None
        };
        let mut response = Response::new(f);
        if let Some(content_length) = content_length {
            response.set_content_length(content_length as usize);
        }
        return response;
    }
}
