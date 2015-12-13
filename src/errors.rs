// This module implements a number of http errors.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error::Error;
use std::fmt;

use httputils::get_name_by_http_code;

use types::PenString;
use wrappers::Response;
use helpers::make_response;

pub use self::HTTPError::{
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    RequestURITooLarge,
    UnsupportedMediaType,
    RequestedRangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    UnprocessableEntity,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
};


/// The HTTP Error type you can return from within your views to trigger a
/// non-200 response.  Here is one usage example:
///
/// ```rust,no_run
/// use pencil::{Request, PencilResult};
/// use pencil::{NotFound, PenHTTPError};
///
///
/// fn view(_: Request) -> PencilResult {
///     return Err(PenHTTPError(NotFound))
/// }
/// ```
///
/// Pencil comes with a shortcut that can be used to return non-200 HTTP error easily:
///
/// ```rust,no_run
/// use pencil::{Request, PencilResult};
/// use pencil::abort;
///
///
/// fn view(_: Request) -> PencilResult {
///     return abort(404)
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub enum HTTPError {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,  // Set "Allow" header key for valid_methods seperated by ", "
    NotAcceptable,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    RequestURITooLarge,
    UnsupportedMediaType,
    RequestedRangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    UnprocessableEntity,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
}

impl HTTPError {
    /// Create a new `HTTPError`.
    pub fn new(code: isize) -> HTTPError {
        match code {
            400 => BadRequest,
            401 => Unauthorized,
            403 => Forbidden,
            404 => NotFound,
            405 => MethodNotAllowed,
            406 => NotAcceptable,
            408 => RequestTimeout,
            409 => Conflict,
            410 => Gone,
            411 => LengthRequired,
            412 => PreconditionFailed,
            413 => RequestEntityTooLarge,
            414 => RequestURITooLarge,
            415 => UnsupportedMediaType,
            416 => RequestedRangeNotSatisfiable,
            417 => ExpectationFailed,
            418 => ImATeapot,
            422 => UnprocessableEntity,
            428 => PreconditionRequired,
            429 => TooManyRequests,
            431 => RequestHeaderFieldsTooLarge,
            500 => InternalServerError,
            501 => NotImplemented,
            502 => BadGateway,
            503 => ServiceUnavailable,
            _ => InternalServerError,
        }
    }

    /// The status code.
    pub fn code(&self) -> isize {
        match *self {
            BadRequest => 400,
            Unauthorized => 401,
            Forbidden => 403,
            NotFound => 404,
            MethodNotAllowed => 405,
            NotAcceptable => 406,
            RequestTimeout => 408,
            Conflict => 409,
            Gone => 410,
            LengthRequired => 411,
            PreconditionFailed => 412,
            RequestEntityTooLarge => 413,
            RequestURITooLarge => 414,
            UnsupportedMediaType => 415,
            RequestedRangeNotSatisfiable => 416,
            ExpectationFailed => 417,
            ImATeapot => 418,
            UnprocessableEntity => 422,
            PreconditionRequired => 428,
            TooManyRequests => 429,
            RequestHeaderFieldsTooLarge => 431,
            InternalServerError => 500,
            NotImplemented => 501,
            BadGateway => 502,
            ServiceUnavailable => 503,
        }
    }

    /// The status name.
    pub fn name(&self) -> &str {
        match get_name_by_http_code(self.code()) {
            Some(name) => name,
            None => "Unknown Error"
        }
    }

    /// Get description.
    fn get_description(&self) -> &str {
        match *self {
            BadRequest => "The browser (or proxy) sent a request that this server \
                           could not understand.",
            Unauthorized => "The server could not verify that you are authorized \
                             to access the URL requested.  You either supplied the \
                             wrong credentials (e.g. a bad password), or your \
                             browser doesn't understand how to supply the \
                             credentials required.",
            Forbidden => "You don't have the permission to access the requested \
                          resource.  It is either read-protected or not readable \
                          by the server.",
            NotFound => "The requested URL was not found on the server.  If you \
                         entered the URL manually please check your spelling and try again.",
            MethodNotAllowed => "The method is not allowed for the requested URL.",
            NotAcceptable => "The resource identified by the request is only capable \
                              of generating response entities which have content \
                              characteristics not acceptable according to the accept \
                              headers sent in the request.",
            RequestTimeout => "The server closed the network connection because the \
                               browser didn't finish the request within the specified time.",
            Conflict => "A conflict happened while processing the request.  The resource \
                         might have been modified while the request was being processed.",
            Gone => "The requested URL is no longer available on this server and there is \
                     no forwarding address.  If you followed a link from a foreign page, \
                     please contact the author of this page.",
            LengthRequired => "A request with this method requires a valid Content-Length header.",
            PreconditionFailed => "The precondition on the request for the URL failed \
                                   positive evaluation.",
            RequestEntityTooLarge => "The data value transmitted exceeds the capacity limit.",
            RequestURITooLarge => "The length of the requested URL exceeds the capacity \
                                   limit for this server.  The request cannot be processed.",
            UnsupportedMediaType => "The server does not support the media type transmitted \
                                     in the request.",
            RequestedRangeNotSatisfiable => "The server cannot provide the requested range.",
            ExpectationFailed => "The server could not meet the requirements of the Expect header",
            ImATeapot => "This server is a teapot, not a coffee machine",
            UnprocessableEntity => "The request was well-formed but was unable to be \
                                    followed due to semantic errors.",
            PreconditionRequired => "This request is required to be conditional; try \
                                     using \"If-Match\" or \"If-Unmodified-Since\".",
            TooManyRequests => "This user has exceeded an allotted request count. Try again later.",
            RequestHeaderFieldsTooLarge => "One or more header fields exceeds the maximum size.",
            InternalServerError => "The server encountered an internal error and was unable \
                                    to complete your request.  Either the server is overloaded \
                                    or there is an error in the application.",
            NotImplemented => "The server does not support the action requested by the browser.",
            BadGateway => "The proxy server received an invalid response from an upstream server.",
            ServiceUnavailable => "The server is temporarily unable to service your request \
                                   due to maintenance downtime or capacity problems.  Please \
                                   try again later.",
        }
    }

    /// Get the HTML body.
    pub fn get_body(&self) -> String {
        format!(
"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 3.2 Final//EN\">
<title>{} {}</title>
<h1>{}</h1>
<p>{}</p>
", self.code().to_string(), self.name(), self.name(), self.get_description())
    }

    /// Get a response object.
    pub fn to_response(&self) -> Response {
        let mut response = make_response(PenString(self.get_body()));
        response.status_code = self.code();
        response.set_content_type("text/html");
        return response;
    }
}

impl fmt::Display for HTTPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.get_description())
    }
}

impl Error for HTTPError {
    fn description(&self) -> &str {
        self.get_description()
    }
}
