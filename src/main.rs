mod turtle;
use turtle::{Color, Point, Turtle};

fn main() {
    let mut tortue: Turtle = Turtle::new();
    tortue.goto(500.0, 500.0);
    let mut width: f64 = 2.0;
    for line in 0..=3 {
        tortue
            .set_pen_width(width)
            .forward(50.0)
            .left(90.0)
            .set_color(Color::Purple)
            .right(90.0);

        width += 4.0;
    }
    tortue.save_svg("dessin.svg");
}
