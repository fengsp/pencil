//! This module exports the http method.

pub use hyper::method::Method;
pub use hyper::method::Method::{
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    Extension,
};
