mod turtle;
use turtle::{Point, Turtle};

fn main() {
    let mut tortue: Turtle = Turtle::new();
    tortue.goto(500.0, 500.0);
    for line in 0..=3 {
        tortue.forward(50.0);
        tortue.pen_up();
        tortue.forward(50.0);
        tortue.pen_down();
    }
    tortue.save_svg("dessin.svg");
}
