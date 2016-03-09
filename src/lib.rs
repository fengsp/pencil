//! Pencil is a microframework for Rust inspired by [Flask](http://flask.pocoo.org/).
//!
//! # Installation
//!
//! This crate is called `pencil` and you can depend on it via cargo:
//!
//! ```ini
//! [dependencies]
//! pencil = "*"
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
//! use pencil::{Request, PencilResult, Response};
//! use pencil::method::Get;
//!
//!
//! fn hello(_: &mut Request) -> PencilResult {
//!     Ok(Response::from("Hello World!"))
//! }
//!
//!
//! fn main() {
//!     let mut app = Pencil::new("/web/hello");
//!     app.route("/", &[Get], "hello", hello);
//!     app.run("127.0.0.1:5000");
//! }
//! ```

#![allow(unused_attributes)]
#![crate_name = "pencil"]
#![crate_type = "lib"]
#![doc(html_logo_url = "https://raw.githubusercontent.com/fengsp/pencil/master/logo/pencil.png",
       html_favicon_url = "https://raw.githubusercontent.com/fengsp/pencil/master/logo/favicon.ico",
       html_root_url = "http://fengsp.github.io/pencil/")]

#![deny(non_camel_case_types)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy))]
#![cfg_attr(feature="clippy", warn(cyclomatic_complexity))]

#[macro_use]
extern crate log;
extern crate hyper;
extern crate rustc_serialize;
extern crate regex;
extern crate url;
extern crate formdata;
extern crate handlebars;
extern crate typemap;
extern crate mime;
extern crate mime_guess;

/* public api */
pub use app::Pencil;
pub use types::{
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
pub use wrappers::{
    Request,
    Response,
};
pub use http_errors::{
    HTTPError
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
pub use module::Module;

#[macro_use]
mod utils;
pub mod http_errors;
pub mod datastructures;
pub mod wrappers;
pub mod routing;
pub mod json;
pub mod config;
pub mod helpers;
pub mod method;
mod testing;
mod app;
mod types;
mod logging;
mod serving;
mod httputils;
mod templating;
mod formparser;
mod module;
