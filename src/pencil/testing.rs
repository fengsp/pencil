// This module implements test support helpers.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use app::Pencil;


/// This type allows to send requests to a wrapped application.
pub struct Client<'c> {
    application: &'c Pencil,
}

impl<'c> Client<'c> {
    /// Create a new `Client`.
    pub fn new(application: &Pencil) -> Client {
        Client { application: application }
    }

    /// Get wrapped application.
    pub fn get_application(&self) -> &Pencil {
        return self.application;
    }
}
