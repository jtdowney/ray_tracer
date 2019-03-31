use crate::{Color, Matrix4, Pattern, Point};
use std::any::Any;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CheckersPattern {
    a: Color,
    b: Color,
    transform: Matrix4,
}

impl CheckersPattern {
    pub fn new(a: Color, b: Color) -> Self {
        CheckersPattern {
            a,
            b,
            transform: Matrix4::identity(),
        }
    }
}

impl PartialEq<Pattern> for CheckersPattern {
    fn eq(&self, other: &Pattern) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |x| self == x)
    }
}

impl Pattern for CheckersPattern {
    fn as_any(&self) -> &Any {
        self
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn pattern_at(&self, Point { x, y, z }: Point) -> Color {
        if (x.floor() + y.floor() + z.floor()) as u32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    #[test]
    fn test_checkers_repeats_in_x() {
        let pattern = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.99, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(1.01, 0.0, 0.0)));
    }

    #[test]
    fn test_checkers_repeats_in_y() {
        let pattern = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.99, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(0.0, 1.01, 0.0)));
    }

    #[test]
    fn test_checkers_repeats_in_z() {
        let pattern = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.99)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(0.0, 0.0, 1.01)));
    }
}
