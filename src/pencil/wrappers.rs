// This module implements simple request and response objects.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::ascii::OwnedAsciiExt;

use http::status;


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
    pub fn get(&self, key:String) -> Option<&String> {
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
