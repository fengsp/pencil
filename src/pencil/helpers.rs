// This module implements various helpers.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::error::Error;
use std::io::File;

use wrappers::Response;
use types::{
    PencilResult,
        PenValue,
        PenResponse,
        PenError,
};


/// Sometimes it is necessary to set additional headers in a view.  Because
/// views do not have to return `Response` objects but can return a value that
/// is converted into a response by Pencil.  You can call this function to
/// get a response object which you can use to attach headers:
///
///```ignore
///fn index() {
///    let mut response = make_response(PencilValue(String::from_str("Hello!")));
///    response.headers.set("X-TEST", "value");
///    return response;
///}
///```
///
pub fn make_response(rv: PencilResult) -> Response {
    match rv {
        PenValue(rv) => Response::new(rv),
        PenResponse(response) => response,
        PenError(e) => Response::new(e.description().to_string()),
    }
}


/// Path bound trait.
pub trait PathBound {
    /// Opens a resource from the root path folder.  Consider the following
    /// folder structure:
    ///
    ///```ignore
    /// /myapp.rs
    /// /user.sql
    /// /templates
    ///     /index.html
    ///```
    ///
    /// If you want to open the `user.sql` file you should do the following:
    ///
    ///```ignore
    ///let mut file = app.open_resource("user.sql");
    ///let content = file.read_to_string().unwrap();
    ///do_something(contents);
    ///```
    ///
    fn open_resource(&self, resource: &str) -> File;
}
