// This module implements simple request and response objects.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use core;
use std::iter;
use std::ascii::OwnedAsciiExt;

use http::status;


/// Headers iterator.
pub type HeaderEntries<'a> = iter::Map<'static, &'a(String, String), (&'a String, &'a String), core::slice::Items<'a, (String, String)>>;
/// Header keys iterator.
pub type HeaderKeys<'a> = iter::Map<'static, (&'a String, &'a String), &'a String, HeaderEntries<'a>>;
/// Header values iterator.
pub type HeaderValues<'a> = iter::Map<'static, (&'a String, &'a String), &'a String, HeaderEntries<'a>>;


/// Headers type that stores some headers.  It has a HashMap like interface
/// but is ordered and can store the same keys multiple times.
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
    pub fn get(&self, key: String) -> Option<&String> {
        let ikey = key.into_ascii_lower();
        for ref kvpairs in self.list.iter() {
            let k = kvpairs.ref0();
            let v = kvpairs.ref1();
            if k.clone().into_ascii_lower() == ikey {
                return Some(v)
            }
        }
        return None
    }

    /// Return a list of all the references to the values for a given key.
    /// If that key is not in the headers, the return value will be an empty vector.
    pub fn get_all(&self, key: String) -> Vec<&String> {
        let ikey = key.into_ascii_lower();
        let mut result = Vec::new();
        for ref kvpairs in self.list.iter() {
            let k = kvpairs.ref0();
            let v = kvpairs.ref1();
            if k.clone().into_ascii_lower() == ikey {
                result.push(v);
            }
        }
        return result
    }

    /// An iterator visiting all key-value pairs in sorted order.
    /// Iterator element type is `(&'a String, &'a String)`.
    pub fn iter(&self) -> HeaderEntries {
        self.list.iter().map(|ref kvpairs| (kvpairs.ref0(), kvpairs.ref1()))
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
    pub fn add(&mut self, key: String, value: String) {
        self.list.push((key, value));
    }

    /// Removes a key from the headers, returning the first value at the key
    /// if the key was previously in the headers.
    pub fn remove(&mut self, key: String) -> Option<String> {
        let ikey = key.into_ascii_lower();
        let mut rv: Option<String> = None;
        let mut newlist = Vec::new();
        for kvpairs in self.list.iter() {
            let k = kvpairs.ref0();
            let v = kvpairs.ref1();
            if k.clone().into_ascii_lower() != ikey {
                newlist.push((k.clone(), v.clone()));
            } else {
                rv = Some(v.clone());
            }
        }
        return rv;
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
pub struct Response {
    pub status: status::Status,
    pub headers: Headers,
    pub body: String,
}

impl Response {
    /// Create a `Response`.
    pub fn new(body: String) -> Response {
        Response {
            status: status::Ok,
            headers: Headers::new(None),
            body: body,
        }
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
}
