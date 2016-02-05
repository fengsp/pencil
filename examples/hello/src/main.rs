extern crate pencil;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

use pencil::Pencil;
use pencil::{Request, PencilResult, PenString};


fn user(r: Request) -> PencilResult {
    Ok(PenString(format!("user {}", r.view_args[0])))
}


fn main() {
    let mut app = Pencil::new("/web/example");
    env::set_var("RUST_LOG", "debug");
    env_logger::init().unwrap();
    app.route(r"/user/(\d+)", &["GET"], "user", user);
    info!("starting up");
    app.run();
}
