use crate::{Color, Matrix4, Pattern, Point};
use std::any::Any;

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

impl PartialEq<Pattern> for SolidPattern {
    fn eq(&self, other: &Pattern) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |x| self == x)
    }
}

impl Pattern for SolidPattern {
    fn as_any(&self) -> &Any {
        self
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
