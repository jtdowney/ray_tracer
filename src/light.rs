use crate::{Color, Point, Scalar};

#[derive(Copy, Clone, Debug)]
pub struct PointLight<T: Scalar> {
    pub position: Point<T>,
    pub intensity: Color<T>,
}

impl<T: Scalar> PointLight<T> {
    pub fn new(position: Point<T>, intensity: Color<T>) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}
