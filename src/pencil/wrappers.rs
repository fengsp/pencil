// This module implements simple request and response objects.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use core;
use std::iter;
use std::ascii::AsciiExt;

use http::status;


/// Headers iterator.
pub type HeaderEntries<'a> = iter::Map<'static, &'a(String, String), (&'a String, &'a String), core::slice::Items<'a, (String, String)>>;
/// Header keys iterator.
pub type HeaderKeys<'a> = iter::Map<'static, (&'a String, &'a String), &'a String, HeaderEntries<'a>>;
/// Header values iterator.
pub type HeaderValues<'a> = iter::Map<'static, (&'a String, &'a String), &'a String, HeaderEntries<'a>>;


/// Headers type that stores some headers.  It has a HashMap like interface
/// but is ordered and can store the same keys multiple times.
#[deriving(Clone)]
pub struct Headers {
    list: Vec<(String, String)>,
}

impl Headers {
    /// Create `Headers`.
    pub fn new(list: Option<Vec<(String, String)>>) -> Headers {
        match list {
            Some(list) => Headers{list: list},
            None => Headers{list: Vec::new()},
        }
    }

    /// Return a reference to the value corresponding to the header key.
    pub fn get(&self, key: &str) -> Option<&String> {
        let ikey = key.to_string().to_ascii_lower();
        for &(ref k, ref v) in self.list.iter() {
            if k.to_ascii_lower() == ikey {
                return Some(v)
            }
        }
        return None
    }

    /// Return a list of all the references to the values for a given key.
    /// If that key is not in the headers, the return value will be an empty vector.
    pub fn get_all(&self, key: &str) -> Vec<&String> {
        let ikey = key.to_string().to_ascii_lower();
        let mut result = Vec::new();
        for &(ref k, ref v) in self.list.iter() {
            if k.to_ascii_lower() == ikey {
                result.push(v);
            }
        }
        return result
    }

    /// An iterator visiting all key-value pairs in sorted order.
    /// Iterator element type is `(&'a String, &'a String)`.
    pub fn iter(&self) -> HeaderEntries {
        self.list.iter().map(|kvpair| (kvpair.ref0(), kvpair.ref1()))
    }

    /// An iterator visiting all keys in sorted order.
    /// Iterator element type is `&'a String`.
    pub fn keys(&self) -> HeaderKeys {
        self.iter().map(|(k, _v)| k)
    }

    /// An iterator visiting all values in sorted order.
    /// Iterator element type is `&'a String`.
    pub fn values(&self) -> HeaderValues {
        self.iter().map(|(_k, v)| v)
    }

    /// Add a new header key-value pair to headers.
    /// TODO: _options_header_vkw
    pub fn add(&mut self, key: &str, value: &str) {
        self.list.push((key.to_string(), value.to_string()));
    }

    /// Removes a key from the headers, returning the first value at the key
    /// if the key was previously in the headers.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let ikey = key.to_string().to_ascii_lower();
        let mut rv: Option<String> = None;
        let mut newlist = Vec::new();
        for &(ref k, ref v) in self.list.iter() {
            if k.to_ascii_lower() != ikey {
                newlist.push((k.clone(), v.clone()));
            } else if rv != None {
                rv = Some(v.clone());
            }
        }
        self.list = newlist;
        return rv;
    }

    /// Removes all headers for `key` and add a new one.  The newly added key either
    /// appears at the end of the list if there was no entry or replaces the old one.
    /// TODO: _option_header_vkw
    pub fn set(&mut self, key: &str, value: &str) {
        let ikey = key.to_string().to_ascii_lower();
        let mut key_existed = false;
        let mut newlist = Vec::new();
        for &(ref k, ref v) in self.list.iter() {
            if k.to_ascii_lower() != ikey {
                newlist.push((k.clone(), v.clone()));
            } else if !key_existed {
                newlist.push((key.to_string(), value.to_string()));
                key_existed = true;
            }
        }
        if !key_existed {
            newlist.push((key.to_string(), value.to_string()));
        }
        self.list = newlist;
    }

    /// Return ths number of elements in the headers.
    pub fn len(&self) -> uint {
        return self.list.len();
    }

    /// Clears all headers.
    pub fn clear(&mut self) {
        self.list.clear();
    }
}


/// Response type.  It is just one container with a couple of parameters
/// (headers, body, status code etc).
#[deriving(Clone)]
pub struct Response {
    status: status::Status,
    pub headers: Headers,
    pub body: String,
}

impl Response {
    /// Create a `Response`.
    pub fn new(body: String) -> Response {
        let mut response = Response {
            status: status::Ok,
            headers: Headers::new(None),
            body: body,
        };
        let content_length = response.body.len().to_string();
        response.headers.set("Content-Type", "text/html; charset=utf-8");
        response.headers.set("Content-Length", content_length.as_slice());
        return response;
    }

    pub fn get_status_code(&self) -> u16 {
        self.status.code()
    }

    pub fn set_status_code(&mut self, code: u16) {
        match FromPrimitive::from_u64(code as u64) {
            Some(status) => { self.status = status; },
            None => { self.status = status::UnregisteredStatus(code as u16, String::from_str("UNKNOWN")); },
        }
    }

    pub fn get_status(&self) -> status::Status {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: status::Status) {
        self.status = status;
    }

    /// Sets a new string as response body.  The content length is set
    /// automatically.
    pub fn set_data(&mut self, value: String) {
        self.body = value;
        let content_length = self.body.len().to_string();
        self.headers.set("Content-Length", content_length.as_slice());
    }

    /// Returns the response content type if available.
    pub fn get_content_type(&self) -> Option<String> {
        let rv = self.headers.get("Content-Type");
        rv.map(|content_type| content_type.clone())
    }

    /// Set response content type.
    pub fn set_content_type(&mut self, value: &str) {
        self.headers.set("Content-Type", value.to_string().as_slice());
    }

    /// Returns the response content length if available.
    pub fn get_content_length(&self) -> Option<uint> {
        let rv = self.headers.get("Content-Length");
        match rv {
            Some(content_length) => from_str(content_length.as_slice()),
            None => None,
        }
    }

    /// Set content length.
    pub fn set_content_length(&mut self, value: uint) {
        self.headers.set("Content-Length", value.to_string().as_slice());
    }
}
