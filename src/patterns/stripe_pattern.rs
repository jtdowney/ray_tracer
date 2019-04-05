use crate::{Color, Matrix4, Pattern, Point};
use derive_builder::Builder;

#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct StripePattern {
    first: Color,
    second: Color,
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
}

impl Pattern for StripePattern {
    fn box_clone(&self) -> Box<Pattern + Sync + Send> {
        Box::new(*self)
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn pattern_at(&self, Point { x, .. }: Point) -> Color {
        if x.floor() as u32 % 2 == 0 {
            self.first
        } else {
            self.second
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    #[test]
    fn creating_stripe_pattern() {
        let pattern = StripePatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.first);
        assert_eq!(color::BLACK, pattern.second);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = StripePatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 1.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 2.0, 0.0)));
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = StripePatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 1.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 2.0)));
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.9, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(1.0, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(-0.1, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(-1.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(-1.1, 0.0, 0.0)));
    }
}
