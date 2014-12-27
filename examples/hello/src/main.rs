extern crate pencil;

use pencil::Pencil;
use pencil::{Request, ViewArgs, PencilResult, PenString};


fn user(_: Request, view_args: ViewArgs) -> PencilResult {
    Ok(PenString(format!("user {}", view_args[0])))
}


fn main() {
    let mut app = Pencil::new("/web/example");
    app.route(r"/user/(\d+)", &["GET"], "user", user);
    app.run();
}
