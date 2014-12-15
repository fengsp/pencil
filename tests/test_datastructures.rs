// Test data structures.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

extern crate pencil;

use pencil::Headers;


#[test]
fn test_headers_basic_interface() {
    let mut headers = Headers::new(None);
    headers.add("Content-Type", "text/plain");
    headers.add("X-Foo", "bar");
    assert!(headers.get("X-foo") != None);
    assert!(headers.get("Content-type") != None);

    headers.set("Content-Type", "foo/bar");
    assert!(*headers.get("Content-Type").unwrap() == String::from_str("foo/bar"));
    assert!(headers.get_all("Content-Type").len() == 1);

    headers.add("X-Foo", "bar2");
    let mut values = vec![];
    for &value in headers.get_all("X-Foo").iter() {
        values.push(value.clone());
    }
    assert!(values == vec![String::from_str("bar"), String::from_str("bar2")]);

    let mut all_keys = vec![];
    let expected_keys = vec![String::from_str("Content-Type"), String::from_str("X-Foo"),
                             String::from_str("X-Foo")];
    for key in headers.keys() {
        all_keys.push(key.clone());
    }
    assert!(all_keys == expected_keys);
    let mut all_values = vec![];
    let expected_values = vec![String::from_str("foo/bar"), String::from_str("bar"),
                               String::from_str("bar2")];
    for value in headers.values() {
        all_values.push(value.clone());
    }
    assert!(all_values == expected_values);

    all_keys.clear();
    all_values.clear();
    for (key, value) in headers.iter() {
        all_keys.push(key.clone());
        all_values.push(value.clone());
    }
    assert!(all_keys == expected_keys);
    assert!(all_values == expected_values);

    headers.remove("X-Foo");
    assert!(headers.len() == 1);
    assert!(headers.get_all("Content-Type").len() == 1);
 
    headers.clear();
    assert!(headers.len() == 0);
}
