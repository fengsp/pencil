// This module implements some useful objects.

use std::iter;
use std::collections::HashMap;
use std::collections::hash_map;


/// MultiDict keys iterator.
pub type MultiDictKeys<'a, T> = hash_map::Keys<'a, String, Vec<T>>;
/// MultiDict list entries iterator.
pub type MultiDictListIter<'a, T> = hash_map::Iter<'a, String, Vec<T>>;
/// MultiDict list values iterator.
pub type MultiDictListValues<'a, T> = hash_map::Values<'a, String, Vec<T>>;
/// MultiDict entries iterator.
pub type MultiDictIter<'a, T> = iter::Map<MultiDictListIter<'a, T>, for<'b, 'c> fn((&'b String, &'c Vec<T>)) -> (&'b String, &'c T)>;
/// MultiDict values iterator.
pub type MultiDictValues<'a, T> = iter::Map<MultiDictListValues<'a, T>, for<'b> fn(&'b Vec<T>) -> &'b T>;


/// This is used to deal with multiple values for the same key.
#[derive(Clone)]
pub struct MultiDict<T> {
    map: HashMap<String, Vec<T>>,
}

impl<T> MultiDict<T> {
    pub fn new() -> MultiDict<T> {
        MultiDict {
            map: HashMap::new(),
        }
    }

    /// Return the first value for this key.
    pub fn get(&self, key: &str) -> Option<&T> {
        match self.map.get(&key.to_string()) {
            Some(value) => Some(&value[0]),
            None => None
        }
    }

    /// Removes an existing key first and add the value.
    pub fn set(&mut self, key: &str, value: T) {
        self.map.insert(key.to_string(), vec![value]);
    }

    /// Adds a new value for the key.
    pub fn add(&mut self, key: String, value: T) {
        match self.map.remove(&key) {
            Some(mut v) => {
                v.push(value);
                self.map.insert(key, v);
            },
            None => {
                self.map.insert(key, vec![value]);
            },
        }
    }

    /// Return the list of items for a given key.
    pub fn getlist(&self, key: &str) -> Option<&Vec<T>> {
        self.map.get(&key.to_string())
    }
    
    /// An iterator of `(key, value)` pairs.
    /// The value will be first value of each key.
    pub fn iter(&self) -> MultiDictIter<T> {
        fn first<'a, 'b, A, B>(kvpair: (&'a A, &'b Vec<B>)) -> (&'a A, &'b B) { (kvpair.0, &kvpair.1[0]) }
        let first: for<'b, 'c> fn((&'b String, &'c Vec<T>)) -> (&'b String, &'c T) = first;
        self.listiter().map(first)
    }

    /// An iterator of `(key, values)` pairs.
    pub fn listiter(&self) -> MultiDictListIter<T> {
        self.map.iter()
    }

    /// An iterator visiting all keys in arbitrary order.
    pub fn keys(&self) -> MultiDictKeys<T> {
        self.map.keys()
    }

    /// An iterator of the first value on every key's value list.
    pub fn values(&self) -> MultiDictValues<T> {
        fn first<'a, A>(list: &'a Vec<A>) -> &'a A { &list[0] }
        let first: for<'b> fn(&'b Vec<T>) -> &'b T = first;
        self.listvalues().map(first)
    }

    /// An iterator of all values corresponding to a key.
    pub fn listvalues(&self) -> MultiDictListValues<T> {
        self.map.values()
    }
}
