// This module implements the logging support for Pencil.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::env;
use serialize::json::Json;

use app::Pencil;


/// Set global log level based on the application's debug flag.
pub fn set_log_level(app: &Pencil) {
    match app.config.get("DEBUG") {
        Some(value) => {
            match value {
                &Json::Boolean(value) => {
                    if value {
                        env::set_var("RUST_LOG", "debug");
                    }
                },
                _ => ()
            }
        }
        None => (),
    }
}
