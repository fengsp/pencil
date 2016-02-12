// This module implements test support helpers.

use app::Pencil;
use wrappers::{Request, Response};


/// This type allows to send requests to a wrapped application.
pub struct PencilClient<'c> {
    application: &'c Pencil,
}

impl<'c> PencilClient<'c> {
    /// Create a new `PencilClient`.
    pub fn new(application: &Pencil) -> PencilClient {
        PencilClient { application: application }
    }

    /// Get wrapped application.
    pub fn get_application(&self) -> &Pencil {
        return self.application;
    }

    /// Runs the wrapped pencil app with the given request.
    fn run_pencil_app(&self, request: &mut Request) -> Response {
        self.application.handle_request(request)
    }

    fn open(&self, mut request: Request) -> Response {
        self.run_pencil_app(&mut request)
    }

    pub fn get(&self, request: Request) -> Response {
        self.open(request)
    }
}
