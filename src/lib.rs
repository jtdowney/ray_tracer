mod canvas;
mod color;
mod matrix;
mod point;
mod vector;

pub use canvas::Canvas;
pub use color::{color, Color};
pub use matrix::{identity_matrix, matrix, Matrix, Matrix2, Matrix3, Matrix4};
pub use point::{point, Point};
pub use vector::{vector, Vector};

pub const EPSILON: f64 = 0.00001;

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
