mod turtle;
use turtle::{Color, Turtle};

fn main() {
    let mut turtle: Turtle = Turtle::new();

    turtle
        .set_custom_color((159, 159, 159))
        .goto(500.0, 500.0)
        .pen_down()
        .circle(75.0)
        .save_svg("dessin.svg");
}
