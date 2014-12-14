extern crate pencil;

use pencil::Pencil;
use pencil::{Request, Params, PencilResult, PenString};


fn user(_: Request, params: Params) -> PencilResult {
    Ok(PenString(format!("user {}", params[0])))
}


fn main() {
    let mut app = Pencil::new("/web/example");
    app.route(r"/user/(\d+)", &["GET"], "user", user);
    app.run();
}
