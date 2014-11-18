// This module implements a number of types.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.


/// The HTTP Error type.
pub struct HTTPError {
    code: int,
    desc: String,
}


/// The Pencil Error type.
#[deriving(Clone)]
pub struct PencilError {
    pub desc: String,
}


/// Result type.
#[deriving(Clone)]
pub enum PencilResult {
    PenValue(String),
    PenError(PencilError),
}


/// Response type.
pub type Response = String;
