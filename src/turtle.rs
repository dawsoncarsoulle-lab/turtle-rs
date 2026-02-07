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

pub struct Line {
    start: Point,
    end: Point,
    color: Color,
    width: f64,
}

pub struct Turtle {
    point: Point,
    angle: f64,
    writing: bool,
    lines: Vec<Line>,
    color: Color,
    width: f64,
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
            width: 2.0,
        }
    }

    pub fn forward(&mut self, dist: f64) -> &mut Self {
        let start = self.point;
        self.point.point.0 += dist * (self.angle.to_radians()).cos();
        self.point.point.1 += dist * (self.angle.to_radians()).sin();
        if self.writing {
            self.lines.push(Line {
                start: start,
                end: self.point,
                color: self.color,
                width: self.width,
            });
        }
        self
    }

    pub fn goto(&mut self, x: f64, y: f64) -> &mut Self {
        self.point.point.0 = x;
        self.point.point.1 = y;
        self
    }

    pub fn right(&mut self, angle: f64) -> &mut Self {
        self.angle -= angle;
        self
    }

    pub fn left(&mut self, angle: f64) -> &mut Self {
        self.angle += angle;
        self
    }

    pub fn pen_up(&mut self) -> &mut Self {
        self.writing = false;
        self
    }

    pub fn pen_down(&mut self) -> &mut Self {
        self.writing = true;
        self
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn set_pen_width(&mut self, width: f64) -> &mut Self {
        self.width = width;
        self
    }

    pub fn save_svg(&self, name: &str) -> std::io::Result<()> {
        let mut f = File::create(name)?;

        writeln!(
            f,
            r#"<svg width="1000" height="1000" xmlns="http://www.w3.org/2000/svg">"#
        )?;

        for line in &self.lines {
            writeln!(
                f,
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{:?}" stroke-width="{}" />"#,
                line.start.point.0,
                line.start.point.1,
                line.end.point.0,
                line.end.point.1,
                line.color,
                line.width
            )?;
        }

        writeln!(f, r#"</svg>"#)?;
        println!("succes");
        Ok(())
    }
}
