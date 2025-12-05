mod canvas;
pub mod color;
mod point;
mod vector;

pub use canvas::{Canvas, canvas};
pub use color::{Color, color};
pub use point::{Point, point};
pub use vector::{Vector, vector};

#[must_use]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
