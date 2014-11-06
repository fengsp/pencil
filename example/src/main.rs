extern crate pencil;
use pencil::Pencil;

fn main() {
    let mut app = Pencil::new();
    app.add_url_rule("/user".to_string(), "get_user".to_string(), "fengsp".to_string());
    app.run("/user".to_string());
}
