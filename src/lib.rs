mod canvas;
pub mod color;
mod matrix;
mod point;
mod vector;

pub use canvas::{Canvas, canvas};
pub use color::{Color, color};
pub use matrix::{Matrix, Matrix2, Matrix3, Matrix4, identity_matrix, matrix};
pub use point::{Point, point};
pub use vector::{Vector, vector};

pub const EPSILON: f64 = 0.0001;

#[must_use]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
