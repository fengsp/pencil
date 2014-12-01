// This module implements a number of http errors.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error;


/// The HTTP Error type.
#[deriving(Clone)]
pub struct HTTPError {
    pub code: int,
    pub desc: &'static str,
}

impl error::Error for HTTPError {

    fn description(&self) -> &str {
        self.desc
    }

    fn detail(&self) -> Option<String> {
        None
    }
}
