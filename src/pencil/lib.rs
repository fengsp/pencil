// A microframework for Rust.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

#![crate_name = "pencil"]
#![crate_type = "lib"]
#![license = "BSD"]
#![comment = "A microframework for Rust."]

#![deny(non_camel_case_types)]

#![experimental]

extern crate http;

/* public api */
pub use app::Pencil;
pub use types::{
    PencilResult,
        PenValue,
        PenError,
};
pub use wrappers::Response;

mod app;
mod types;
mod wrappers;
