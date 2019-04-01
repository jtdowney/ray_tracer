use crate::{Color, Matrix4, Pattern, Point};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SolidPattern {
    color: Color,
    transform: Matrix4,
}

impl From<Color> for SolidPattern {
    fn from(color: Color) -> Self {
        SolidPattern::new(color)
    }
}

impl SolidPattern {
    pub fn new(color: Color) -> Self {
        SolidPattern {
            color,
            transform: Matrix4::identity(),
        }
    }
}

impl Pattern for SolidPattern {
    fn box_clone(&self) -> Box<Pattern> {
        Box::new((*self).clone())
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn pattern_at(&self, _: Point) -> Color {
        self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    #[test]
    fn test_solid_pattern_returns_constant_color() {
        let pattern = SolidPattern::new(color::WHITE);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(1.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(1.0, 1.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(1.0, 1.0, 1.0)));
    }
}
