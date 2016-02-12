// A microframework for Rust.

//! Pencil is a microframework for Rust inspired by [Flask](http://flask.pocoo.org/).
//!
//! # Installation
//!
//! This crate is called `pencil` and you can depend on it via cargo:
//!
//! ```ini
//! [dependencies.pencil]
//! git = "https://github.com/fengsp/pencil.git"
//! ```
//!
//! # Quickstart
//!
//! A short introduction to Pencil.
//!
//! ## A Minimal Application
//!
//! A minimal Pencil application looks something like this:
//!
//! ```rust,no_run
//! extern crate pencil;
//!
//! use pencil::Pencil;
//! use pencil::{Request, PencilResult, PenString};
//!
//!
//! fn hello(_: &Request) -> PencilResult {
//!     Ok(PenString(String::from("Hello World!")))
//! }
//!
//!
//! fn main() {
//!     let mut app = Pencil::new("/web/hello");
//!     app.route("/", &["GET"], "hello", hello);
//!     app.run();
//! }
//! ```

#![allow(unused_attributes)]
#![crate_name = "pencil"]
#![crate_type = "lib"]
#![doc(html_logo_url = "https://raw.githubusercontent.com/fengsp/pencil/master/logo/pencil.png",
       html_favicon_url = "https://raw.githubusercontent.com/fengsp/pencil/master/logo/favicon.ico",
       html_root_url = "http://fengsp.github.io/pencil/")]

#![deny(non_camel_case_types)]

#[macro_use]
extern crate log;
extern crate hyper;
extern crate rustc_serialize;
extern crate regex;
extern crate url;
extern crate handlebars;

/* public api */
pub use app::Pencil;
pub use types::{
    PencilValue,
        PenString,
        PenResponse,
    PencilError,
        PenHTTPError,
        PenUserError,
    UserError,
    PencilResult,
    ViewArgs,
    ViewFunc,
    UserErrorHandler,
    HTTPErrorHandler,
    BeforeRequestFunc,
    AfterRequestFunc,
    TeardownRequestFunc,
};
pub use datastructures::{
    MultiDict,
};
pub use wrappers::{
    Request,
    Response,
};
pub use json::jsonify;
pub use config::{
    Config,
};
pub use helpers::{
    PathBound,
    safe_join,
    abort,
    redirect,
    escape,
    send_file,
    send_from_directory,
};
pub use serving::{
    run_server,
};
pub use http_errors::{
    HTTPError,
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
pub use testing::PencilClient;

#[macro_use]
mod utils;
mod app;
mod types;
mod datastructures;
mod wrappers;
mod json;
mod config;
mod logging;
mod helpers;
mod serving;
mod http_errors;
mod routing;
mod testing;
mod httputils;
mod templating;
