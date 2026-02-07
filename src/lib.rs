//! turtle_rs (SVG)
//!
//! A fluent turtle-graphics library for generating SVG drawings.
//!
//! This crate provides an ergonomic API inspired by classic turtle graphics,
//! using a **polar coordinate model** to generate geometric and generative shapes.
//!
//! ## Features
//! - Chainable turtle commands
//! - SVG output
//! - Colors and stroke width
//! - Pen up / pen down
//! - Parametric radial shapes
//!
//! ## Example
//! ```rust
//! use turtle_rs::{Turtle, Color};
//!
//! let mut t = Turtle::new();
//!
//! t.pen_down()
//!  .set_color(Color::Purple)
//!  .circle(120.0)
//!  .right(45.0)
//!  .set_color(Color::Red)
//!  .star(80.0, 5)
//!  .save_svg("example.svg");
//! ```

mod turtle;

pub use turtle::Color;
pub use turtle::Turtle;
