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

    pub fn forward(&mut self, dist: f64) {
        self.x += dist * (self.angle.to_radians()).cos();
        self.y += dist * (self.angle.to_radians()).sin()
    }

    pub fn right(&mut self, angle: f64) {
        self.angle -= angle
    }

    pub fn left(&mut self, angle: f64) {
        self.angle += angle
    }
}
