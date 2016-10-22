//! This module implements the logging support for Pencil.

use std::env;
use serde_json::Value;

use app::Pencil;


/// Set global log level based on the application's debug flag.
/// This is only useful for `env_logger` crate.
pub fn set_log_level(app: &Pencil) {
    if let Some(value) = app.config.get("DEBUG") {
        if let Value::Bool(value) = *value {
            if value {
                env::set_var("RUST_LOG", "debug");
            }
        }
    }
}
