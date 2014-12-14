// This module implements various helpers.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::io::File;

use wrappers::Response;
use types::{
    PencilValue,
        PenString,
        PenResponse,
};


/// Sometimes it is necessary to set additional headers in a view.  Because
/// views do not have to return `Response` objects but can return a value that
/// is converted into a response by Pencil.  You can call this function to
/// get a response object which you can use to attach headers:
///
/// ```rust,no_run
/// use pencil::{Request, Params, PencilResult, PenString, PenResponse, make_response};
///
///
/// fn index(_: Request, _: Params) -> PencilResult {
///     let mut response = make_response(PenString(String::from_str("Hello!")));
///     response.headers.set("X-TEST", "value");
///     return Ok(PenResponse(response));
/// }
/// ```
pub fn make_response(rv: PencilValue) -> Response {
    match rv {
        PenString(rv) => Response::new(rv),
        PenResponse(response) => response,
    }
}


/// Path bound trait.
pub trait PathBound {
    /// Opens a resource from the root path folder.  Consider the following
    /// folder structure:
    ///
    /// ```ignore
    /// /myapp.rs
    /// /user.sql
    /// /templates
    ///     /index.html
    /// ```
    ///
    /// If you want to open the `user.sql` file you should do the following:
    ///
    /// ```rust,no_run
    /// use pencil::PathBound;
    ///
    ///
    /// fn main() {
    ///     let app = pencil::Pencil::new("/web/demo");
    ///     let mut file = app.open_resource("user.sql");
    ///     let content = file.read_to_string().unwrap();
    /// }
    /// ```
    fn open_resource(&self, resource: &str) -> File;
}


/// Safely join directory and filename, otherwise this returns None.
pub fn safe_join(directory: &str, filename: &str) -> Option<Path> {
    let directory = Path::new(directory);
    let filename = Path::new(filename);
    match filename.as_str() {
        Some(filename_str) => {
            if filename.is_absolute() | (filename_str == "..") | (filename_str.starts_with("../")) {
                None
            } else {
                Some(directory.join(filename_str))
            }
        },
        None => None,
    }
}
