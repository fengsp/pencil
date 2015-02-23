// This module implements the http server support for our application.
// Copyright (c) 2014 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::old_io::net::ip::Ipv4Addr;

use hyper::server::Server;

use app::Pencil;


/// Run the `Pencil` application.
pub fn run_server(application: Pencil) {
    let server = Server::http(Ipv4Addr(127, 0, 0, 1), 5000);
    let _guard = server.listen(application).unwrap();
    println!("Listening on http://127.0.0.1:5000");
}
