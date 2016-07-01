//! This module implements the http server support for our application.

use std::env;
use std::str::FromStr;

use std::net::ToSocketAddrs;

use hyper::server::Server;

use app::Pencil;


/// Run the `Pencil` application.
pub fn run_server<A: ToSocketAddrs>(application: Pencil, addr: A) {
    let threads_str = env::var("THREADS").unwrap_or(String::new());
    let threads = FromStr::from_str(&threads_str).unwrap_or(10) as usize;

    let server = Server::http(addr).unwrap();
    let _guard = server.handle_threads(application,threads).unwrap();
}
