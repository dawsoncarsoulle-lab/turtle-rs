mod turtle;
use turtle::{Color, Point, Turtle};

fn main() {
    let mut turtle = Turtle::new();
    turtle.goto(500.0, 500.0).star(50.0, 5);
    turtle.goto(500.0, 800.0).square(50.0);
    turtle.goto(500.0, 300.0).circle(100.0);
    turtle.goto(300.0, 500.0).triangle(50.0);

    turtle.save_svg("dessin.svg");
}
