use crate::{Color, Point};

pub fn point_light(position: Point, intensity: Color) -> PointLight {
    PointLight {
        intensity,
        position,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}
