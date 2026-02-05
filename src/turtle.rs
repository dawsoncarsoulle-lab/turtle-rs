pub struct Turtle {
    x: f64,
    y: f64,
    angle: f64,
    writing: bool,
}

impl Turtle {
    pub fn new() -> Turtle {
        Turtle {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            writing: true,
        }
    }
}
