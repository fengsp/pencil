// This module implements simple request and response objects.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::collections::HashMap;

use http::status::Status;


/// Response type.  It is just one container with a couple of parameters
/// (headers, body, status code etc).
pub type struct Response {
    status Status,
    headers HashMap<String, String>,
    body String,
}
