use crate::{Color, Point};

pub fn point_light<T>(position: Point<T>, intensity: Color<T>) -> PointLight<T>
where
    T: Copy,
{
    PointLight {
        position,
        intensity,
    }
}

pub struct PointLight<T>
where
    T: Copy,
{
    pub position: Point<T>,
    pub intensity: Color<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, point};

    #[test]
    fn creating_point_light() {
        let intensity = color(1, 1, 1);
        let position = point(0, 0, 0);
        let light = point_light(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
