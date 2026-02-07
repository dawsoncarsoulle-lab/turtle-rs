# Turtle RS

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight and simple Rust library for creating vector graphics (SVG) using "Turtle Graphics" logic, inspired by Python's famous turtle module.

Perfect for learning Rust, teaching algorithmics, or generating geometric patterns programmatically.

## Features

- **Intuitive Movement**: Forward, turn left/right, goto coordinates.
- **Pen Control**: Pen up/down, change color, adjust line width.
- **Built-in Shapes**: Square, Triangle, Circle, Star, Polygons.
- **SVG Export**: Saves your art to clean, scalable SVG files.
- **Fluent Interface**: Chain methods for cleaner code (e.g., `t.forward(10).right(90)`).
- **Zero dependencies**

## Installation

Add the dependency to your `Cargo.toml`:

```bash
cargo add turtle_rs
```

## Quick Start

Here is how to draw a red triangle and save it:

```rust
use turtle_rs::turtle::{Turtle, Color};

fn main() {
    // 1. Create a new turtle
    let mut t = Turtle::new(); // needs binding

    // 2. Draw (you can chain commands!)
    t.set_color(Color::Red)
     .set_pen_width(2.0);

    // Draw a triangle
    t.forward(100.0);
    t.right(120.0);
    t.forward(100.0);
    t.right(120.0);
    t.forward(100.0);

    // 3. Save the result
    t.save_svg("drawing.svg");

    // or

    let mut t = Turtle::new();

    t.set_color(Color::Red)
     .set_pen_width(2.0)
     .forward(100.0)
     .right(120.0)
     .forward(100.0)
     .right(120.0)
     .forward(100.0)
     .save_svg("drawing.svg");
}
```

## Shapes API

### Circles and Polygons

```rust
t.circle(100.0);
t.square(80.0);
t.triangle(60.0);
```

> ℹ️ Note
> Shapes such as squares and triangles are regular polygons **inscribed in a
> circle**, generated from a polar representation. This approach is ideal for
> generative art and visualization, but does not aim to replace exact
> Cartesian geometry.

## Parametric Radial Shapes

The core primitive of this crate is the `shape` method:

```rust
t.shape(radius, segments, |i| {
    // deformation factor per segment
    (i as f64 * 0.1).sin().abs() + 1.0
});
```

This allows the creation of:

- stars
- flowers
- spirals
- organic blobs
- custom generative shapes

## Coordinate System

The turtle starts at the top left corner of the canvas `(0, 0)`.

- **X** increases to the right.
- **Y** increases downwards (SVG standard).
- Angle **0°** points to the East (Right).

## Contribution

Contributions are welcome! Feel free to open an Issue or a Pull Request on the repository.

## License

This project is licensed under the MIT License.
