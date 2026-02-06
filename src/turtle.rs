use std::f64;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Blue,
    Red,
    Purple,
    Pink,
    Orange,
    Yellow,
    Green,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    point: (f64, f64),
}

pub struct Turtle {
    point: Point,
    angle: f64,
    writing: bool,
    lines: Vec<(Point, Point, Color)>,
    color: Color,
}

impl Point {
    pub fn new() -> Point {
        Point { point: (0.0, 0.0) }
    }

    pub fn new_point(x: f64, y: f64) -> Point {
        Point { point: (x, y) }
    }
}

impl Turtle {
    pub fn new() -> Turtle {
        Turtle {
            point: Point::new(),
            angle: 0.0,
            writing: true,
            lines: Vec::new(),
            color: Color::Black,
        }
    }

    pub fn forward(&mut self, dist: f64) {
        let (x_start, y_start) = self.point.point;
        self.point.point.0 += dist * (self.angle.to_radians()).cos();
        self.point.point.1 += dist * (self.angle.to_radians()).sin();
        if self.writing {
            self.lines.push((
                Point::new_point(x_start, y_start),
                Point::new_point(self.point.point.0, self.point.point.1),
                self.color,
            ));
        }
    }

    pub fn goto(&mut self, x: f64, y: f64) {
        self.point.point.0 = x;
        self.point.point.1 = y;
    }

    pub fn right(&mut self, angle: f64) {
        self.angle -= angle
    }

    pub fn left(&mut self, angle: f64) {
        self.angle += angle
    }

    pub fn pen_up(&mut self) {
        self.writing = false
    }

    pub fn pen_down(&mut self) {
        self.writing = true
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn save_svg(&self, name: &str) -> std::io::Result<()> {
        let mut f = File::create(name)?;

        writeln!(
            f,
            r#"<svg width="1000" height="1000" xmlns="http://www.w3.org/2000/svg">"#
        )?;

        for (start, end, color) in &self.lines {
            writeln!(
                f,
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{:?}" stroke-width="2" />"#,
                start.point.0, start.point.1, end.point.0, end.point.1, color
            )?;
        }

        writeln!(f, r#"</svg>"#)?;
        println!("succes");
        Ok(())
    }
}
