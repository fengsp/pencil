extern crate pencil;

use pencil::Pencil;
use pencil::{Request, PencilResult, PenString};


fn user(r: Request) -> PencilResult {
    Ok(PenString(format!("user {}", r.view_args[0])))
}


fn main() {
    let mut app = Pencil::new("/web/example");
    app.route(r"/user/(\d+)", &["GET"], "user", user);
    app.run();
}
