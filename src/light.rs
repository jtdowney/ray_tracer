use crate::{Color, Point};

#[must_use]
pub fn point_light(position: Point, intensity: Color) -> PointLight {
    PointLight {
        position,
        intensity,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, point};

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = color(1, 1, 1);
        let position = point(0, 0, 0);
        let light = point_light(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
