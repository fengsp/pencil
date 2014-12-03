// A microframework for Rust.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

#![allow(unused_attributes)]
#![crate_name = "pencil"]
#![crate_type = "lib"]
#![license = "BSD"]
#![comment = "A microframework for Rust."]

#![deny(non_camel_case_types)]

#![experimental]

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
    Response,
};
pub use json::jsonify;
pub use config::{
    Config,
};
pub use helpers::{
    make_response,
    PathBound,
};
pub use serving::{
    run_server,
};
pub use errors::{
    HTTPError,
};

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
