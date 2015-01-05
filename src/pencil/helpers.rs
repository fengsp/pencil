// This module implements various helpers.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::io::File;
use std::io::fs::PathExtensions;

use url;

use wrappers::{Request, Response};
use types::{
    PencilValue,
        PenString,
        PenResponse,
    PenHTTPError,
    PencilResult,
};
use errors::{
    HTTPError,
        NotFound,
};


/// Sometimes it is necessary to set additional headers in a view.  Because
/// views do not have to return `Response` objects but can return a value that
/// is converted into a response by Pencil.  You can call this function to
/// get a response object which you can use to attach headers:
///
/// ```rust,no_run
/// use pencil::{Request, PencilResult, PenString, PenResponse, make_response};
///
///
/// fn index(_: Request) -> PencilResult {
///     let mut response = make_response(PenString(String::from_str("Hello!")));
///     response.set_content_type("text", "css");
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


/// One helper function that can be used to return HTTP Error inside a view function.
pub fn abort(code: int) -> PencilResult {
    let error = HTTPError::new(code);
    return Err(PenHTTPError(error));
}


/// Returns a response that redirects the client to the target location.
pub fn redirect(location: &str, code: int) -> PencilResult {
    let mut response = Response::new(format!(
"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 3.2 Final//EN\">
<title>Redirecting...</title>
<h1>Redirecting...</h1>
<p>You should be redirected automatically to target URL: 
<a href=\"{}\">{}</a>.  If not click the link.
", location, location));
    response.status_code = code;
    response.set_content_type("text", "html");
    response.headers.location = Some(url::Url::parse(location).unwrap());
    return Ok(PenResponse(response));
}


/// Replace special characters "&", "<", ">" and (") to HTML-safe characters.
pub fn escape(s: String) -> String {
    return s.replace("&", "&amp;").replace("<", "&lt;")
            .replace(">", "&gt;").replace("\"", "&quot;");
}


/// Sends the contents of a file to the client.  Please never pass filenames to this
/// function from user sources without checking them first.  Set `as_attachment` to
/// `true` if you want to send this file with a `Content-Disposition: attachment`
/// header.  This will return `NotFound` if filepath is not one file.
pub fn send_file(filepath: &str, mimetype: &str, as_attachment: bool) -> PencilResult {
    let filepath = Path::new(filepath);
    if !filepath.is_file() {
        return Err(PenHTTPError(NotFound));
    }
    let mut file = match File::open(&filepath) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", filepath.display(), e.desc),
    };
    let mut response = match file.read_to_string() {
        Ok(data) => {
            Response::new(data)
        },
        Err(e) => panic!("couldn't read {}: {}", filepath.display(), e.desc),
    };
    let types: Vec<&str> = mimetype.split_str("/").collect();
    response.set_content_type(types[0], types[1]);
    if as_attachment {
        match filepath.filename_str() {
            Some(filename) => {
                response.headers.extensions.insert(String::from_str("Content-Disposition"),
                    format!("attachment; filename={}", filename));
            },
            None => {
                panic!("filename unavailable, required for sending as attachment.");
            }
        }
    }
    return Ok(PenResponse(response));
}


/// Send a file from a given directory with `send_file`.  This is a secure way to
/// quickly expose static files from an folder.
pub fn send_from_directory(directory: &str, filename: &str, mimetype: &str,
                           as_attachment: bool) -> PencilResult {
    match safe_join(directory, filename) {
        Some(filepath) => {
            match filepath.as_str() {
                Some(filepath) => {
                    return send_file(filepath, mimetype, as_attachment);
                },
                None => {
                    return Err(PenHTTPError(NotFound));
                }
            }
        },
        None => {
            return Err(PenHTTPError(NotFound));
        }
    }
}


/// View function used internally to send static files from the static folder
/// to the browser.
pub fn send_static_file(request: Request) -> PencilResult {
    let mut static_folder = Path::new(request.app.root_path.as_slice());
    static_folder.push(request.app.static_folder.as_slice());
    let static_folder_str = static_folder.as_str().unwrap();
    let filename = request.view_args[0].as_slice();
    let mimetype = "text/plain";
    return send_from_directory(static_folder_str, filename, mimetype, false);
}
