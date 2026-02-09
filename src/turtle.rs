use std::f64;
use std::fs::File;
use std::io::Write;

/// Represents the available pen colors.
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
    Custom(u8, u8, u8),
}

#[derive(Clone, Copy, Debug)]
struct Point {
    point: (f64, f64),
}

struct Line {
    start: Point,
    end: Point,
    color: Color,
    width: f64,
}

/// The main Turtle structure.
///
/// It keeps track of its position, angle, pen state (up/down),
/// and the list of lines drawn so far.
pub struct Turtle {
    point: Point,
    angle: f64,
    writing: bool,
    lines: Vec<Line>,
    color: Color,
    width: f64,
}

impl Color {
    fn to_svg(&self) -> String {
        match self {
            Color::Custom(r, g, b) => {
                format!("rgb({}, {}, {})", r, g, b)
            }
            Color::Yellow => "Yellow".to_string(),
            Color::Blue => "Blue".to_string(),
            Color::Green => "Green".to_string(),
            Color::Orange => "Orange".to_string(),
            Color::Pink => "Pink".to_string(),
            Color::Purple => "Purple".to_string(),
            Color::Red => "Red".to_string(),
            _ => "Black".to_string(),
        }
    }
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
    /// Creates a new turtle at the origin, facing right (0°),
    /// with 2.0 width and a black stroke. The pen cannot writing.
    pub fn new() -> Turtle {
        Turtle {
            point: Point::new(),
            angle: 0.0,
            writing: false,
            lines: Vec::new(),
            color: Color::Black,
            width: 2.0,
        }
    }

    /// Moves the turtle forward by a given distance.
    ///
    /// If the pen is down, a line segment is drawn.
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

    /// Moves the turtle backward by a given distance.
    ///
    /// If the pen is down, a line segment is drawn.
    pub fn backward(&mut self, dist: f64) -> &mut Self {
        self.forward(-dist);
        self
    }

    /// Moves the turtle to an absolute position.
    ///
    /// If the pen is down, a line is drawn between the current
    /// position and the target point.
    pub fn goto(&mut self, x: f64, y: f64) -> &mut Self {
        if self.writing {
            self.lines.push(Line {
                start: self.point,
                end: Point::new_point(x, y),
                color: self.color,
                width: self.width,
            });
        }
        self.point.point.0 = x;
        self.point.point.1 = y;
        self
    }

    /// Moves the turtle to an absolute position.
    ///
    /// If the pen is down, a line is not drawn between the current
    /// position and the target point.
    pub fn teleport(&mut self, x: f64, y: f64) -> &mut Self {
        self.point.point = (x, y);
        self
    }

    /// Rotates the turtle clockwise by the given angle (in degrees).
    pub fn right(&mut self, angle: f64) -> &mut Self {
        self.angle -= angle;
        self
    }

    /// Rotates the turtle counter-clockwise by the given angle (in degrees).
    pub fn left(&mut self, angle: f64) -> &mut Self {
        self.angle += angle;
        self
    }

    /// Lifts the pen, preventing drawing while moving.
    pub fn pen_up(&mut self) -> &mut Self {
        self.writing = false;
        self
    }

    /// Lowers the pen, enabling drawing.
    pub fn pen_down(&mut self) -> &mut Self {
        self.writing = true;
        self
    }

    /// Sets the stroke color for subsequent lines.
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    /// Sets a custom RGB color for the pen.
    /// # Example
    /// ```
    /// t.set_custom_color((159, 159, 159));
    /// ```
    pub fn set_custom_color(&mut self, rgb: (u8, u8, u8)) -> &mut Self {
        self.color = Color::Custom(rgb.0, rgb.1, rgb.2);
        self
    }

    /// Sets the stroke width for subsequent lines.
    pub fn set_pen_width(&mut self, width: f64) -> &mut Self {
        self.width = width;
        self
    }

    /// Sets the x-coordinate of the turtle's position.
    pub fn set_x(&mut self, x: f64) -> &mut Self {
        self.point.point.0 = x;
        self
    }

    /// Sets the y-coordinate of the turtle's position.
    pub fn set_y(&mut self, y: f64) -> &mut Self {
        self.point.point.1 = y;
        self
    }

    /// Returns whether the pen is currently down.
    pub fn pen_is_down(&self) -> bool {
        self.writing
    }

    /// Returns the current position of the turtle.
    pub fn position(&self) -> (f64, f64) {
        (self.point.point.0, self.point.point.1)
    }

    /// Returns the current x-coordinate of the turtle.
    pub fn x_cor(&self) -> f64 {
        self.point.point.0
    }

    /// Returns the current y-coordinate of the turtle.
    pub fn y_cor(&self) -> f64 {
        self.point.point.1
    }

    /// Returns the current heading (orientation) of the turtle in degrees.
    pub fn heading(&self) -> f64 {
        self.angle
    }

    /// Calculates the Euclidean distance between the turtle and a target point.
    pub fn distance(&self, x: f64, y: f64) -> f64 {
        ((self.y_cor() - y).powf(2.0) + (self.x_cor() - x).powf(2.0)).sqrt()
    }

    /// Calculates the angle from the turtle's current position to a target point.
    ///
    /// Returns the heading (in degrees) that the turtle would need to face
    /// in order to point directly at the given coordinates.
    ///
    /// The angle is measured clockwise from the positive x-axis (East):
    /// - 0° = East
    /// - 90° = North
    /// - 180° or -180° = West
    /// - -90° = South
    pub fn toward(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x_cor();
        let dy = y - self.y_cor();
        dy.atan2(dx).to_degrees()
    }

    /// Sets the turtle's heading to an absolute angle.
    ///
    /// Unlike [`left`](Self::left) and [`right`](Self::right), which rotate
    /// relative to the current heading, this method sets the heading directly.
    pub fn absolute_orientation(&mut self, angle: f64) -> &mut Self {
        self.angle = angle;
        self
    }

    /// Orients the turtle to face toward a target point.
    ///
    /// This is a convenience method that combines [`toward`](Self::toward)
    /// and [`absolute_orientation`](Self::absolute_orientation).
    /// The turtle's position does not change.
    pub fn face_toward(&mut self, x: f64, y: f64) -> &mut Self {
        self.angle = self.toward(x, y);
        self
    }

    /// Draws a parametric radial shape.
    ///
    /// The shape is generated in polar coordinates around the
    /// turtle's current position.
    ///
    /// - `radius` defines the base radius
    /// - `segments` controls angular resolution
    /// - `deformation` modulates the radius per segment
    ///
    /// This method enables the creation of circles, stars,
    /// flowers, spirals, and organic generative shapes.
    ///
    /// # Example
    /// ```rust
    /// t.shape(100.0, 360, |i| {
    ///     (i as f64 * 0.1).sin().abs() + 1.0
    /// });
    /// ```
    pub fn shape<F>(&mut self, radius: f64, segments: usize, deformation: F) -> &mut Self
    where
        F: Fn(usize) -> f64,
    {
        let (cx, cy) = self.point.point;
        let angle_0: f64 = self.angle;
        let radius_0 = radius * deformation(0);
        let x_0 = cx + radius_0 * angle_0.to_radians().cos();
        let y_0 = cy + radius_0 * angle_0.to_radians().sin();

        self.pen_up();
        self.goto(x_0, y_0);
        self.pen_down();
        for i in 1..=segments {
            let angle = i as f64 * (360.0 / segments as f64) + self.angle;
            let actual_radius = radius * deformation(i % segments);
            let target_x = cx + actual_radius * angle.to_radians().cos();
            let target_y = cy + actual_radius * angle.to_radians().sin();
            self.goto(target_x, target_y);
        }
        self.pen_up();
        self
    }

    /// Draws a circle using a radial approximation.
    pub fn circle(&mut self, radius: f64) -> &mut Self {
        self.shape(radius, 360, |_| 1.0);
        self
    }

    /// Draws a square as a regular polygon inscribed in a circle.
    pub fn square(&mut self, size: f64) -> &mut Self {
        self.right(45.0);
        self.shape(1.0, 4, |_| 1.0 * size);
        self.left(45.0);
        self
    }

    /// Draws a triangle as a regular polygon inscribed in a circle.
    pub fn triangle(&mut self, size: f64) -> &mut Self {
        self.right(90.0);
        self.shape(1.0, 3, |_| 1.0 * size);
        self.left(90.0);
        self
    }

    /// Draws a star shape.
    ///
    /// `branch` must be greater than or equal to 3.
    pub fn star(&mut self, size: f64, branch: usize) -> &mut Self {
        if branch * 2 < 6 {
            println!("error : the number of branch must be superior or equal to 3");
            return self;
        }
        self.right(90.0);
        self.shape(1.0, branch * 2, |i| {
            if i % 2 == 0 {
                1.0 * size
            } else {
                0.4 * size
            }
        });
        self.left(90.0);
        self
    }

    /// Saves the drawing as an SVG file.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file cannot be written.
    pub fn save_svg(&self, name: &str) -> std::io::Result<()> {
        let mut f = File::create(name)?;

        writeln!(
            f,
            r#"<svg width="1000" height="1000" xmlns="http://www.w3.org/2000/svg">"#
        )?;

        for line in &self.lines {
            writeln!(
                f,
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
                line.start.point.0,
                line.start.point.1,
                line.end.point.0,
                line.end.point.1,
                line.color.to_svg(),
                line.width
            )?;
        }

        writeln!(f, r#"</svg>"#)?;
        Ok(())
    }
}
