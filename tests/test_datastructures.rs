// Test data structures.

extern crate pencil;

use pencil::MultiDict;


#[test]
fn test_multi_dict_basic_interface() {
    let mut multi_dict = MultiDict::new();
    multi_dict.add("Content-Type", "text/plain");
    multi_dict.add("X-Foo", "bar");
    assert!(multi_dict.get("X-Foo") != None);
    assert!(multi_dict.get("Content-Type") != None);

    multi_dict.set("Content-Type", "foo/bar");
    assert!(multi_dict.get("Content-Type").unwrap() == "foo/bar");

    multi_dict.add("X-Foo", "bar2");

    let mut all_keys = vec![];
    for key in multi_dict.keys() {
        all_keys.push(key);
    }
    assert!(all_keys.len() == 2);
    let mut all_values = vec![];
    for value in multi_dict.values() {
        all_values.push(value);
    }
    assert!(all_values.len() == 2);
}
