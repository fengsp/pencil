// A microframework for Rust.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

#![crate_name = "pencil"]
#![crate_type = "lib"]
#![license = "BSD"]
#![comment = "A microframework for Rust."]

#![deny(non_camel_case_types)]

#![experimental]

extern crate core;
extern crate serialize;
extern crate http;

/* public api */
pub use app::Pencil;
pub use types::{
    PencilResult,
        PenValue,
        PenResponse,
        PenError,
};
pub use wrappers::{
    Headers,
    Response,
};
pub use json::jsonify;
pub use config::{
    Config,
    ConfigTrait,
};

mod app;
mod types;
mod wrappers;
mod json;
mod config;
