use turtle::Turtle;

fn main() {
    let mut t = Turtle::new();
    t.set_speed("instant");
    turtle_text::write_text(&mut t, "Hallo!", 50.0);
}
