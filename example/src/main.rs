extern crate pencil;

use pencil::Pencil;
use pencil::{Request, Params, PencilResult, PenValue};


fn user(_: Request, params: Params) -> PencilResult {
    PenValue(format!("user {}", params[0]))
}


fn main() {
    let mut app: Pencil = Pencil::new("/web/example");
    app.route(r"/user/(\d+)", &["GET"], "user", user);
    app.run();
}
