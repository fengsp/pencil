// This module implements a number of types.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error;

use wrappers::{Request, Response};
use errors::HTTPError;
use routing::Params;

pub use self::PencilValue::{
    PenString, PenResponse
};
pub use self::PencilError::{
    PencilHTTPError,
    PencilUserError
};


/// The Pencil User Error type.
#[deriving(Clone)]
pub struct UserError {
    pub desc: &'static str,
    pub detail: Option<String>,
}

impl error::Error for UserError {
    fn description(&self) -> &str {
        self.desc
    }

    fn detail(&self) -> Option<String> {
        self.detail.clone()
    }
}


/// The Pencil Error type.
#[deriving(Clone)]
pub enum PencilError {
    PencilHTTPError(HTTPError),
    PencilUserError(UserError),
}

impl error::FromError<HTTPError> for PencilError {
    fn from_error(err: HTTPError) -> PencilError {
        PencilHTTPError(err)
    }
}

impl error::FromError<UserError> for PencilError {
    fn from_error(err: UserError) -> PencilError {
        PencilUserError(err)
    }
}

impl error::Error for PencilError {

    fn description(&self) -> &str {
        match *self {
            PencilHTTPError(ref err) => err.description(),
            PencilUserError(ref err) => err.description(),
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            PencilHTTPError(ref err) => err.detail(),
            PencilUserError(ref err) => err.detail(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &PencilHTTPError(ref err) => Some(&*err as &error::Error),
            &PencilUserError(_) => None,
        }
    }
}


/// Pencil view function return value type.
#[deriving(Clone)]
pub enum PencilValue {
    PenString(String),
    PenResponse(Response),
}


/// The Pencil Result type.
pub type PencilResult = Result<PencilValue, PencilError>;


/// View function type.
pub type ViewFunc = fn(Request, Params) -> PencilResult;
