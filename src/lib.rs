mod canvas;
pub mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod point;
mod ray;
mod sphere;
pub mod transformations;
mod vector;

pub use canvas::*;
pub use color::*;
pub use intersection::*;
pub use light::*;
pub use material::*;
pub use matrix::*;
pub use point::*;
pub use ray::*;
pub use sphere::*;
pub use vector::*;

pub const EPSILON: f64 = 0.0001;

pub trait ByteScale {
    fn byte_scale(self) -> u8;
}

impl ByteScale for f32 {
    fn byte_scale(self) -> u8 {
        let value = num::clamp(self * 255.0, 0.0, 255.0).round();
        value as u8
    }
}

impl ByteScale for f64 {
    fn byte_scale(self) -> u8 {
        let value = num::clamp(self * 255.0, 0.0, 255.0).round();
        value as u8
    }
}
