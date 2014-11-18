// This module implements a number of types.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.


/// The HTTP Exception type.
#[deriving(Clone)]
pub struct HTTPError {
    code: int,
    description: String,
}


/// Result type.
pub type PencilResult = Result<String, HTTPError>;
