extern crate pencil;

use pencil::Pencil;
use pencil::{Request, PencilResult, PenString};


fn user(r: Request) -> PencilResult {
    Ok(PenString(format!("user {}", r.view_args.get("user_id").unwrap())))
}


fn main() {
    let mut app = Pencil::new("/web/example");
    app.route("/user/<int:user_id>", &["GET"], "user", user);
    app.run();
}
