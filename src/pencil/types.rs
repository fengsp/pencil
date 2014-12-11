// This module implements a number of types.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error;

use wrappers::{Request, Response};
use errors::HTTPError;
use routing::Params;

pub use self::PencilResult::{
    PenValue, PenResponse, PenError
};
pub use self::PencilError::{
    PencilHTTPError
};


/// The Pencil Error type.
#[deriving(Clone)]
pub enum PencilError {
    PencilHTTPError(HTTPError),
}

impl error::FromError<HTTPError> for PencilError {

    fn from_error(err: HTTPError) -> PencilError {
        PencilHTTPError(err)
    }
}

impl error::Error for PencilError {

    fn description(&self) -> &str {
        match *self {
            PencilHTTPError(ref err) => err.description(),
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            PencilHTTPError(ref err) => err.detail(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &PencilHTTPError(ref err) => Some(&*err as &error::Error),
        }
    }
}


/// Result type.
#[deriving(Clone)]
pub enum PencilResult {
    PenValue(String),
    PenResponse(Response),
    PenError(PencilError),
}


/// View function type.
pub type ViewFunc = fn(Request, Params) -> PencilResult;
