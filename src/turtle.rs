use std::f64;
use std::fs::File;
use std::io::Write;

use crate::util::*;

/// The main Turtle structure.
///
/// It keeps track of its position, angle, pen state (up/down),
/// and the list of lines drawn so far.
pub struct Turtle {
    pub position: Point,
    pub angle: f64,
    pub writing: bool,
    lines: Vec<Line>,
    pub color: Color,
    pub width: f64,
}

impl Turtle {
    const DEFAULT_ANGLE: f64 = 0.0;
    const DEFAULT_WRITING_STATE: bool = false;
    const DEFAULT_COLOR: Color = Color::Black;
    const DEFAULT_LINE_WIDTH: f64 = 2.0;
    const DEFAULT_INITIAL_POINT: Point = Point { x: 0.0, y: 0.0 };

    /// Creates a new turtle at the origin, facing right (0°),
    /// with 2.0 width and a black stroke. The pen cannot writing.
    pub fn new() -> Turtle {
        Turtle {
            position: Turtle::DEFAULT_INITIAL_POINT,
            angle: Turtle::DEFAULT_ANGLE,
            writing: Turtle::DEFAULT_WRITING_STATE,
            lines: Vec::new(),
            color: Turtle::DEFAULT_COLOR,
            width: Turtle::DEFAULT_LINE_WIDTH,
        }
    }

    /// Moves the turtle forward by a given distance.
    ///
    /// If the pen is down, a line segment is drawn.
    pub fn forward(&mut self, dist: f64) -> &mut Self {
        let start = self.position;

        let (sin, cos) = self.angle.to_radians().sin_cos();
        self.position.x += dist * cos;
        self.position.y += dist * sin;

        if self.writing {
            self.lines.push(Line {
                start: start,
                end: self.position,
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
                start: self.position,
                end: Point { x, y },
                color: self.color,
                width: self.width,
            });
        }
        self.position = Point { x, y };
        self
    }

    /// Moves the turtle to an absolute position.
    ///
    /// If the pen is down, a line is not drawn between the current
    /// position and the target point.
    pub fn teleport_to(&mut self, x: f64, y: f64) -> &mut Self {
        self.position = Point { x, y };
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

    /// Sets a custom RGB color for the pen.
    /// # Example
    /// ```
    /// t.set_custom_color((159, 159, 159));
    /// ```
    pub fn with_color_custom(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.color = Color::Custom(red, green, blue);
        self
    }

    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    /// Sets the stroke width for subsequent lines.
    pub fn with_pen_width(&mut self, width: f64) -> &mut Self {
        self.width = width;
        self
    }

    /// Returns whether the pen is currently down.
    pub fn pen_is_down(&self) -> bool {
        self.writing
    }

    /// Calculates the Euclidean distance between the turtle and a target point.
    pub fn distance_to(&self, x: f64, y: f64) -> f64 {
        (self.position.x - x).hypot(self.position.y - y)
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
    pub fn towards(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.position.x;
        let dy = y - self.position.y;
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
        self.angle = self.towards(x, y);
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
        assert!(segments > 0, "Segment count cannot be 0");

        let initial_point = self.position;
        let initial_angle = self.angle;

        let mut coordinates = (0..=segments).map(|i| {
            let angle = i as f64 * (360.0 / segments as f64) + initial_angle;
            let step_radius = radius * deformation(i % segments); // implicit deformation(0), assert needed

            let (sin, cos) = angle.to_radians().sin_cos();
            let current_x = initial_point.x + step_radius * cos;
            let current_y = initial_point.y + step_radius * sin;

            (current_x, current_y)
        });

        if let Some((initial_x, initial_y)) = coordinates.next() {
            self.teleport_to(initial_x, initial_y);
        } else {
            unsafe { std::hint::unreachable_unchecked() } // will have one element
        }

        for (x, y) in coordinates.into_iter() {
            let _ = self.goto(x, y);
        }

        self
    }

    /// Saves the drawing as an SVG file.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file cannot be written.
    pub fn save_svg(&self, name: &str) -> std::io::Result<File> {
        let mut f = File::create(name)?;

        writeln!(
            f,
            r#"<svg width="1000" height="1000" xmlns="http://www.w3.org/2000/svg">"# // hardcoded dimensions...
        )?;

        for line in &self.lines {
            writeln!(
                f,
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" />"#,
                line.start.x,
                line.start.y,
                line.end.x,
                line.end.y,
                line.color.to_svg(),
                line.width
            )?;
        }

        writeln!(f, r#"</svg>"#)?;
        Ok(f)
    }
}
