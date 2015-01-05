// This module implements the http server support for our application.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use core::num::FromPrimitive;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

use http;
use http::status;
use http::server::{Server, ResponseWriter};

use app::Pencil;
use wrappers::Request;


/// The pencil server.
#[derive(Clone)]
struct PencilServer {
    app: Pencil,
}

impl PencilServer {
    /// Create a `PencilServer`.
    pub fn new(application: Pencil) -> PencilServer {
        PencilServer { app: application }
    }
}

impl Server for PencilServer {
    fn get_config(&self) -> http::server::Config {
        http::server::Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8000 } }
    }

    fn handle_request(&self, r: http::server::Request, w: &mut ResponseWriter) {
        let request = Request::new(&self.app, r);
        let request_method = request.method();
        let response = self.app.handle_request(request);
        response.write(request_method, w);
    }
}


/// Run the `Pencil` application.
pub fn run_server(application: Pencil) {
    let pencil_server = PencilServer::new(application);
    pencil_server.serve_forever();
}


pub fn get_status_from_code(code: int) -> status::Status {
    match FromPrimitive::from_u64(code as u64) {
            Some(status) => { status },
            None => { status::UnregisteredStatus(code as u16, String::from_str("UNKNOWN")) },
    }
}
