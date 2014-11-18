extern crate pencil;

use pencil::Pencil;
use pencil::PenValue;

fn main() {
    let mut app = Pencil::new();
    app.add_url_rule("/user".to_string(), "get_user".to_string(), PenValue("fengsp".to_string()));
    app.run();
}
