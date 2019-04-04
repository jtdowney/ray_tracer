use crate::{Color, Matrix4, Pattern, Point};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform: Matrix4,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern {
            a,
            b,
            transform: Matrix4::identity(),
        }
    }
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
    fn creating_stripe_pattern() {
        let pattern = StripePattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.a);
        assert_eq!(color::BLACK, pattern.b);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = StripePattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 1.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 2.0, 0.0)));
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = StripePattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 1.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 2.0)));
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.9, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(1.0, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(-0.1, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(-1.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(-1.1, 0.0, 0.0)));
    }
}
