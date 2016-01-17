// This module implements some useful objects.

use std::iter;
use std::collections::HashMap;
use std::collections::hash_map;


/// MultiDict keys iterator.
pub type MultiDictKeys<'a> = hash_map::Keys<'a, String, Vec<String>>;
/// MultiDict list entries iterator.
pub type MultiDictListIter<'a> = hash_map::Iter<'a, String, Vec<String>>;
/// MultiDict list values iterator.
pub type MultiDictListValues<'a> = hash_map::Values<'a, String, Vec<String>>;
/// MultiDict entries iterator.
pub type MultiDictIter<'a> = iter::Map<MultiDictListIter<'a>, for<'b, 'c> fn((&'b String, &'c Vec<String>)) -> (&'b String, &'c String)>;
/// MultiDict values iterator.
pub type MultiDictValues<'a> = iter::Map<MultiDictListValues<'a>, for<'b> fn(&'b Vec<String>) -> &'b String>;


/// This is used to deal with multiple values for the same key.
#[derive(Clone)]
pub struct MultiDict {
    map: HashMap<String, Vec<String>>,
}

impl MultiDict {
    pub fn new() -> MultiDict {
        MultiDict {
            map: HashMap::new(),
        }
    }

    /// Return the first value for this key.
    pub fn get(&self, key: &str) -> Option<&String> {
        match self.map.get(&key.to_string()) {
            Some(value) => Some(&value[0]),
            None => None
        }
    }

    /// Removes an existing key first and add the value.
    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), vec![value.to_string()]);
    }

    /// Adds a new value for the key.
    pub fn add(&mut self, key: &str, value: &str) {
        match self.map.remove(&key.to_string()) {
            Some(mut v) => {
                v.push(value.to_string());
                self.map.insert(key.to_string(), v);
            },
            None => {
                self.map.insert(key.to_string(), vec![value.to_string()]);
            },
        }
    }

    /// Return the list of items for a given key.
    pub fn getlist(&self, key: &str) -> Option<&Vec<String>> {
        self.map.get(&key.to_string())
    }
    
    /// An iterator of `(key, value)` pairs.
    pub fn iter(&self) -> MultiDictIter {
        fn first<'a, 'b, A>(kvpair: (&'a A, &'b Vec<A>)) -> (&'a A, &'b A) { (kvpair.0, &kvpair.1[0]) }
        let first: for<'b, 'c> fn((&'b String, &'c Vec<String>)) -> (&'b String, &'c String) = first;
        self.listiter().map(first)
    }

    /// An iterator of `(key, values)` pairs.
    pub fn listiter(&self) -> MultiDictListIter {
        self.map.iter()
    }

    /// An iterator visiting all keys in arbitrary order.
    pub fn keys(&self) -> MultiDictKeys {
        self.map.keys()
    }

    /// An iterator of the first value on every key's value list.
    pub fn values(&self) -> MultiDictValues {
        fn first<'a, A>(list: &'a Vec<A>) -> &'a A { &list[0] }
        let first: for<'b> fn(&    'b Vec<String>) -> &'b String = first;
        self.listvalues().map(first)
    }

    /// An iterator of all values corresponding to a key.
    pub fn listvalues(&self) -> MultiDictListValues {
        self.map.values()
    }
}
