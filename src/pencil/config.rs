// This module implements configuration related stuff.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use serialize::json::JsonObject;


pub trait ConfigTrait {
    fn from_jsonfile(&mut self, filepath: Path) -> Path {
        filepath
    }
}


/// Currently we are using `JsonObject` as config type.  Works like
/// a JSON Object but provides ways to fill it from JSON files:
///
/// ```rust
/// app.config.from_jsonfile("yourconfig.json")
/// ```
///
/// You can also load configurations from an environment variable
/// pointing to a file:
///
/// ```rust
/// app.config.from_envvar("YOURAPPLICATION_SETTINGS")
/// ```
///
/// In this case, you have to set this environment variable to the file
/// you want to use.  On Linux and OS X you can use the export statement:
///
/// ```bash
/// export YOURAPPLICATION_SETTINGS="/path/to/config/file"
/// ```
/// 
pub type Config = JsonObject;

impl ConfigTrait for Config {
}
