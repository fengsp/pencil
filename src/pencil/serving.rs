// This module implements the http server support for our application.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::io::net::ip::{SocketAddr, Ipv4Addr};

use http;
use http::server::{Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

use app::Pencil;


/// The pencil server.
#[deriving(Clone)]
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

    fn handle_request(&self, r: Request, w: &mut ResponseWriter) {
        // let request = r;
        let response = self.app.handle_request(r);

        w.headers.content_type = Some(MediaType {
            type_ : String::from_str("text"),
            subtype: String::from_str("html"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });
        w.headers.server = Some(String::from_str("Pencil"));
        w.write(response.body.as_bytes()).unwrap();

        self.app.do_teardown_request();
    }
}


/// Run the `Pencil` application.
pub fn run_server(application: Pencil) {
    let pencil_server = PencilServer::new(application);
    pencil_server.serve_forever();
}
