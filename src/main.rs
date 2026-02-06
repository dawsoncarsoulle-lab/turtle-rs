mod turtle;
use turtle::{Color, Point, Turtle};

fn main() {
    let mut tortue: Turtle = Turtle::new();
    tortue.goto(500.0, 500.0);
    for line in 0..=3 {
        tortue.set_color(Color::Green);
        tortue.forward(50.0);
        tortue.set_color(Color::Blue);
        tortue.forward(50.0);
    }
    tortue.save_svg("dessin.svg");
}
