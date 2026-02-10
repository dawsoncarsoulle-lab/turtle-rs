use turtle_rs::{turtle::Turtle, util::Color};

fn main() {
    let mut turtle: Turtle = Turtle::new();

    let _ = turtle
        .with_color(Color::Black)
        .goto(500.0, 500.0)
        .pen_down()
        .shape(80.0, 360, |_| 1.0)
        .save_svg("apples.svg");
}
