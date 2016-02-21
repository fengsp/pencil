//! This module implements the form parsing. It supports url-encoded forms
//! as well as multipart uploads.

use std::io::Read;

use hyper;
use hyper::mime::{Mime, TopLevel, SubLevel};
use formdata::{get_multipart_boundary, parse_multipart};
use formdata::uploaded_file::UploadedFile;
use url::form_urlencoded;

use datastructures::MultiDict;


/// This type implements parsing of form data for Pencil. It can parse
/// multipart and url encoded form data.
pub struct FormDataParser;

impl FormDataParser {
    pub fn new() -> FormDataParser {
        FormDataParser
    }

    pub fn parse(&self, request: &mut hyper::server::request::Request, mimetype: &Mime) -> (MultiDict<String>, MultiDict<UploadedFile>) {
        let default = (MultiDict::new(), MultiDict::new());
        match *mimetype {
            Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, _) => {
                let mut body: Vec<u8> = Vec::new();
                match request.read_to_end(&mut body) {
                    Ok(_) => {
                        let mut form = MultiDict::new();
                        for (k, v) in form_urlencoded::parse(&body) {
                            form.add(k, v);
                        }
                        (form, MultiDict::new())
                    },
                    Err(_) => {
                        default
                    }
                }
            },
            Mime(TopLevel::Multipart, SubLevel::FormData, _) => {
                match get_multipart_boundary(&request.headers) {
                    Ok(boundary) => {
                        match parse_multipart(request, boundary) {
                            Ok(form_data) => {
                                let mut form = MultiDict::new();
                                let mut files = MultiDict::new();
                                for (name, value) in form_data.fields {
                                    form.add(name, value);
                                }
                                for (name, file) in form_data.files {
                                    files.add(name, file);
                                }
                                (form, files)
                            },
                            Err(_) => {
                                default
                            }
                        }
                    },
                    Err(_) => {
                        default
                    }
                }
            },
            _ => {
                default
            }
        }
    }
}
