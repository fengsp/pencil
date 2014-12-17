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
    PenHTTPError,
    PenUserError
};


/// The Pencil User Error type.
#[deriving(Clone)]
pub struct UserError {
    pub desc: &'static str,
    pub detail: Option<String>,
}

impl UserError {
    pub fn new(desc: &'static str, detail: Option<String>) -> UserError {
        UserError {
            desc: desc,
            detail: detail,
        }
    }
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
    PenHTTPError(HTTPError),
    PenUserError(UserError),
}

impl error::FromError<HTTPError> for PencilError {
    fn from_error(err: HTTPError) -> PencilError {
        PenHTTPError(err)
    }
}

impl error::FromError<UserError> for PencilError {
    fn from_error(err: UserError) -> PencilError {
        PenUserError(err)
    }
}

impl error::Error for PencilError {

    fn description(&self) -> &str {
        match *self {
            PenHTTPError(ref err) => err.description(),
            PenUserError(ref err) => err.description(),
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            PenHTTPError(ref err) => err.detail(),
            PenUserError(ref err) => err.detail(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &PenHTTPError(ref err) => Some(&*err as &error::Error),
            &PenUserError(_) => None,
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


/// Error handler type.
pub type HTTPErrorHandler = fn(HTTPError) -> PencilResult;
pub type UserErrorHandler = fn(UserError) -> PencilResult;
