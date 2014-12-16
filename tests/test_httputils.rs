// Test http utilities.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

extern crate pencil;

use pencil::{get_name_by_http_code};


#[test]
fn test_get_name_by_http_code() {
    let status_name = get_name_by_http_code(200).unwrap();
    assert!(status_name == "OK");
}
