// This module implements a number of types.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error;
use std::error::Error;
use std::fmt;

use wrappers::{Request, Response};
use errors::HTTPError;

pub use self::PencilValue::{
    PenString, PenResponse
};
pub use self::PencilError::{
    PenHTTPError,
    PenUserError
};


/// The Pencil User Error type.
#[derive(Clone)]
pub struct UserError {
    pub desc: &'static str,
}

impl UserError {
    pub fn new(desc: &'static str) -> UserError {
        UserError {
            desc: desc,
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.desc)
    }
}

impl error::Error for UserError {
    fn description(&self) -> &str {
        self.desc
    }
}


/// The Pencil Error type.
#[derive(Clone)]
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

impl fmt::Display for PencilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PenHTTPError(ref err) => f.write_str(err.description()),
            PenUserError(ref err) => f.write_str(err.description()),
        }
    }
}

impl error::Error for PencilError {

    fn description(&self) -> &str {
        match *self {
            PenHTTPError(ref err) => err.description(),
            PenUserError(ref err) => err.description(),
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
#[derive(Clone)]
pub enum PencilValue {
    PenString(String),
    PenResponse(Response),
}


/// The Pencil Result type.
pub type PencilResult = Result<PencilValue, PencilError>;


/// View arguments type.
pub type ViewArgs = Vec<String>;
/// View function type.
pub type ViewFunc = fn(Request) -> PencilResult;

impl Clone for ViewFunc {
    fn clone(&self) -> ViewFunc {
        *self
    }
}


/// HTTP Error handler type.
pub type HTTPErrorHandler = fn(HTTPError) -> PencilResult;
/// User Error handler type.
pub type UserErrorHandler = fn(UserError) -> PencilResult;


/// Before request func type.
pub type BeforeRequestFunc = fn(&Request) -> Option<PencilResult>;

impl Clone for BeforeRequestFunc {
    fn clone(&self) -> BeforeRequestFunc {
        *self
    }
}


/// After request func type.
pub type AfterRequestFunc = fn(&mut Response);

impl Clone for AfterRequestFunc {
    fn clone(&self) -> AfterRequestFunc {
        *self
    }
}


/// Teardown request func type.
pub type TeardownRequestFunc = fn(Option<&PencilError>);

impl Clone for TeardownRequestFunc {
    fn clone(&self) -> TeardownRequestFunc {
        *self
    }
}
