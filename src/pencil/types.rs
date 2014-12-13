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
    PencilHTTPError,
    PencilUserError
};


/// The Pencil Error type.
#[deriving(Clone)]
pub enum PencilError {
    PencilHTTPError(HTTPError),
    PencilUserError(&'static str, Option<String>),
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
            PencilUserError(desc, _) => desc,
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            PencilHTTPError(ref err) => err.detail(),
            PencilUserError(_, ref detail) => detail.clone(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &PencilHTTPError(ref err) => Some(&*err as &error::Error),
            &PencilUserError(_, _) => None,
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
