//! This module implements various helpers.

use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

use hyper::header::{Location, ContentType};
use mime_guess::guess_mime_type;
use mime::Mime;

use wrappers::Response;
use types::{
    PenHTTPError,
    PencilResult,
    UserError,
};
use http_errors::{
    HTTPError,
        NotFound,
};


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
    /// use std::io::Read;
    ///
    /// use pencil::PathBound;
    ///
    ///
    /// fn main() {
    ///     let app = pencil::Pencil::new("/web/demo");
    ///     let mut file = app.open_resource("user.sql");
    ///     let mut content = String::from("");
    ///     file.read_to_string(&mut content).unwrap();
    /// }
    /// ```
    fn open_resource(&self, resource: &str) -> File;
}


/// Safely join directory and filename, otherwise this returns None.
pub fn safe_join(directory: &str, filename: &str) -> Option<PathBuf> {
    let directory = Path::new(directory);
    let filename = Path::new(filename);
    match filename.to_str() {
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
pub fn abort(code: isize) -> PencilResult {
    let error = HTTPError::new(code);
    return Err(PenHTTPError(error));
}


/// Returns a response that redirects the client to the target location.
pub fn redirect(location: &str, code: isize) -> PencilResult {
    let mut response = Response::from(format!(
"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 3.2 Final//EN\">
<title>Redirecting...</title>
<h1>Redirecting...</h1>
<p>You should be redirected automatically to target URL: 
<a href=\"{}\">{}</a>.  If not click the link.
", location, location));
    response.status_code = code;
    response.set_content_type("text/html");
    response.headers.set(Location(location.to_string()));
    return Ok(response);
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
pub fn send_file(filepath: &str, mimetype: Mime, as_attachment: bool) -> PencilResult {
    let filepath = Path::new(filepath);
    if !filepath.is_file() {
        return Err(PenHTTPError(NotFound));
    }
    let file = match File::open(&filepath) {
        Ok(file) => file,
        Err(e) => {
            return Err(UserError::new(format!("couldn't open {}: {}", filepath.display(), e.description())).into());
        }
    };
    let mut response: Response = file.into();
    response.headers.set(ContentType(mimetype));
    if as_attachment {
        match filepath.file_name() {
            Some(file) => {
                match file.to_str() {
                    Some(filename) => {
                        let content_disposition = format!("attachment; filename={}", filename);
                        response.headers.set_raw("Content-Disposition", vec![content_disposition.as_bytes().to_vec()]);
                    },
                    None => {
                        return Err(UserError::new("filename unavailable, required for sending as attachment.").into());
                    }
                }
            },
            None => {
                return Err(UserError::new("filename unavailable, required for sending as attachment.").into());
            }
        }
    }
    return Ok(response);
}


/// Send a file from a given directory with `send_file`.  This is a secure way to
/// quickly expose static files from an folder.  This will guess the mimetype
/// for you.
pub fn send_from_directory(directory: &str, filename: &str,
                           as_attachment: bool) -> PencilResult {
    match safe_join(directory, filename) {
        Some(filepath) => {
            let mimetype = guess_mime_type(filepath.as_path());
            match filepath.as_path().to_str() {
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
