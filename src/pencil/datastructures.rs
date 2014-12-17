// This module implements some useful objects.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use core;
use std::iter;
use std::ascii::AsciiExt;


/// Headers iterator.
pub type HeaderEntries<'a> = iter::Map<&'a(String, String), (&'a String, &'a String), core::slice::Items<'a, (String, String)>, for<'b> fn(&'b(String, String)) -> (&'b String, &'b String)>;
/// Header keys iterator.
pub type HeaderKeys<'a> = iter::Map<(&'a String, &'a String), &'a String, HeaderEntries<'a>, fn((&'a String, &'a String)) -> &'a String>;
/// Header values iterator.
pub type HeaderValues<'a> = iter::Map<(&'a String, &'a String), &'a String, HeaderEntries<'a>, fn((&'a String, &'a String)) -> &'a String>;


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
        fn unpack<A, B>(kvpair: &(A, B)) -> (&A, &B) { (&kvpair.0, &kvpair.1) }
        self.list.iter().map(unpack)
    }

    /// An iterator visiting all keys in sorted order.
    /// Iterator element type is `&'a String`.
    pub fn keys(&self) -> HeaderKeys {
        fn first<A, B>((k, _): (A, B)) -> A { k }
        self.iter().map(first)
    }

    /// An iterator visiting all values in sorted order.
    /// Iterator element type is `&'a String`.
    pub fn values(&self) -> HeaderValues {
        fn second<A, B>((_, v): (A, B)) -> B { v }
        self.iter().map(second)
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
    /// TODO: _option_header_vkw and validate_value
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
