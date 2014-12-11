// A microframework for Rust.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

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
//! use pencil::{Request, Params, PencilResult, PenValue};
//!
//!
//! fn hello(_: Request, _: Params) -> PencilResult {
//!     PenValue(String::from_str("Hello World!"))
//! }
//!
//!
//! fn main() {
//!     let mut app = Pencil::new("/web/hello");
//!     app.route(r"/", &["GET"], "hello", hello);
//!     app.run();
//! }
//! ```

#![allow(unused_attributes)]
#![crate_name = "pencil"]
#![crate_type = "lib"]
#![license = "BSD"]
#![comment = "A microframework for Rust."]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://fengsp.github.io/pencil/")]
#![experimental]
#![deny(non_camel_case_types)]

extern crate core;
extern crate serialize;
extern crate regex;
extern crate http;

/* public api */
pub use app::Pencil;
pub use types::{
    PencilResult,
        PenValue,
        PenResponse,
        PenError,
    PencilError,
};
pub use datastructures::{
    Headers,
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
    make_response,
    PathBound,
    safe_join,
};
pub use serving::{
    run_server,
};
pub use routing::Params;
pub use errors::{
    HTTPError,
};
pub use testing::PencilClient;

mod app;
mod types;
mod datastructures;
mod wrappers;
mod json;
mod config;
mod logging;
mod helpers;
mod serving;
mod errors;
mod routing;
mod testing;
mod httputils;
