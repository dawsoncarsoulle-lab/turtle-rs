use std::borrow::Cow;

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

impl Color {
    pub fn to_svg(&self) -> Cow<'static, str> {
        match self {
            Color::Custom(r, g, b) => Cow::Owned(format!("rgb({}, {}, {})", r, g, b)),
            Color::Yellow => Cow::Borrowed("yellow"),
            Color::Blue => Cow::Borrowed("blue"),
            Color::Green => Cow::Borrowed("green"),
            Color::Orange => Cow::Borrowed("orange"),
            Color::Pink => Cow::Borrowed("pink"),
            Color::Purple => Cow::Borrowed("purple"),
            Color::Red => Cow::Borrowed("red"),
            Color::Black => Cow::Borrowed("black"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: Color,
    pub width: f64,
}
