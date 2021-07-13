use crate::{Color, Matrix4, Pattern};
use derive_builder::Builder;
use num::Integer;

pub fn stripe_pattern(a: Color, b: Color) -> StripePattern {
    StripePatternBuilder::default().a(a).b(b).build().unwrap()
}

#[derive(Builder)]
pub struct StripePattern {
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
    a: Color,
    b: Color,
}

impl Pattern for StripePattern {
    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: crate::Point) -> Color {
        let x = point.x.floor() as i32;
        if x.is_even() {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, point};

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = stripe_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.a, color::WHITE);
        assert_eq!(pattern.b, color::BLACK);
    }

    #[test]
    fn pattern_is_constant_in_y() {
        let pattern = stripe_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.0, 1.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.0, 2.0, 0.0)), color::WHITE);
    }

    #[test]
    fn pattern_is_constant_in_z() {
        let pattern = stripe_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 2.0)), color::WHITE);
    }

    #[test]
    fn pattern_alternates_in_x() {
        let pattern = stripe_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.9, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(point(-0.1, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(point(-1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(point(-1.1, 0.0, 0.0)), color::WHITE);
    }
}
