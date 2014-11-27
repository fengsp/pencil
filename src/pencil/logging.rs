// This module implements the logging support for Pencil.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::os;
use serialize::json::Json;

use app::Pencil;


/// Set global log level based on the application's debug flag.
pub fn set_log_level(app: &Pencil) {
    match app.config.get(&String::from_str("DEBUG")) {
        Some(value) => {
            match value {
                &Json::Boolean(value) => {
                    if value {
                        os::setenv("RUST_LOG", "debug");
                    }
                },
                _ => ()
            }
        }
        None => (),
    }
}
