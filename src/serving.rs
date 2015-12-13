// This module implements the http server support for our application.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use hyper::server::Server;

use app::Pencil;


/// Run the `Pencil` application.
pub fn run_server(application: Pencil) {
    let server = Server::http("127.0.0.1:5000").unwrap();
    let _guard = server.handle(application).unwrap();
    println!("Listening on http://127.0.0.1:5000");
}
