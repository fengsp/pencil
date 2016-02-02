// This module implements the logging support for Pencil.

use std::env;
use rustc_serialize::json::Json;

use app::Pencil;


/// Set global log level based on the application's debug flag.
/// This is only useful for `env_logger` crate.
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
