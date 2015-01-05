// This module implements a number of types.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error;

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
