// This module implements a number of types.

use std::error;
use std::convert;
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
#[derive(Clone, Debug)]
pub struct UserError {
    pub desc: String,
}

impl UserError {
    pub fn new<T>(desc: T) -> UserError where T: AsRef<str> {
        UserError {
            desc: desc.as_ref().to_owned(),
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.desc)
    }
}

impl error::Error for UserError {
    fn description(&self) -> &str {
        &self.desc
    }
}


/// The Pencil Error type.
#[derive(Clone, Debug)]
pub enum PencilError {
    PenHTTPError(HTTPError),
    PenUserError(UserError),
}

impl convert::From<HTTPError> for PencilError {
    fn from(err: HTTPError) -> PencilError {
        PenHTTPError(err)
    }
}

impl convert::From<UserError> for PencilError {
    fn from(err: UserError) -> PencilError {
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


/// HTTP Error handler type.
pub type HTTPErrorHandler = fn(HTTPError) -> PencilResult;
/// User Error handler type.
pub type UserErrorHandler = fn(UserError) -> PencilResult;


/// Before request func type.
pub type BeforeRequestFunc = fn(&Request) -> Option<PencilResult>;


/// After request func type.
pub type AfterRequestFunc = fn(&mut Response);


/// Teardown request func type.
pub type TeardownRequestFunc = fn(Option<&PencilError>);
