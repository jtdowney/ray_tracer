mod canvas;
mod color;
mod matrix;
mod point;
pub mod transformations;
mod vector;

pub use canvas::*;
pub use color::*;
pub use matrix::*;
pub use point::*;
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
