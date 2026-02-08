mod turtle;
use turtle::{Color, Turtle};

fn main() {
    let mut turtle: Turtle = Turtle::new();

    turtle
        .set_custom_color((159, 159, 159))
        .goto(500.0, 500.0)
        .pen_down()
        .circle(50.0)
        .pen_up()
        .goto(300.0, 300.0)
        .pen_down()
        .set_color(Color::Orange)
        .triangle(50.0)
        .save_svg("dessin.svg");
}
