use crate::{Color, Point};

pub fn point_light(position: Point, intensity: Color) -> PointLight {
    PointLight {
        position,
        intensity,
    }
}

#[derive(Clone, Copy)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, point};

    #[test]
    fn creating_point_light() {
        let intensity = color::WHITE;
        let position = point::ORIGIN;
        let light = point_light(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
