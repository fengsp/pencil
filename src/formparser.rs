// This module implements the form parsing. It supports url-encoded forms
// as well as multipart uploads.

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

    pub fn parse(&self, request: &mut hyper::server::request::Request, mimetype: &Mime) -> (MultiDict, Vec<(String, UploadedFile)>) {
        let default = (MultiDict::new(), Vec::new());
        match *mimetype {
            Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, _) => {
                let mut body: Vec<u8> = Vec::new();
                match request.read_to_end(&mut body) {
                    Ok(_) => {
                        let mut form = MultiDict::new();
                        for (ref k, ref v) in form_urlencoded::parse(&body) {
                            form.add(k, v);
                        }
                        (form, Vec::new())
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
                                for (ref name, ref value) in form_data.fields {
                                    form.add(name, value);
                                }
                                (form, form_data.files)
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
