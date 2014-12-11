// This module implements a number of http errors.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error;

use httputils::get_name_by_http_code;


/// The HTTP Error type.
#[deriving(Clone)]
pub struct HTTPError {
    pub code: int,
    pub name: &'static str,
    pub desc: &'static str,
}

impl HTTPError {
    /// Create a new `HTTPError`.
    pub fn new(code: int) -> HTTPError {
        let name = match get_name_by_http_code(code) {
            Some(name) => name,
            None => "Unknown Error"
        };
        let desc = "desc";
        HTTPError {
            code: code,
            name: name,
            desc: desc,
        }
    }
}

impl error::Error for HTTPError {

    fn description(&self) -> &str {
        self.desc
    }

    fn detail(&self) -> Option<String> {
        None
    }
}
