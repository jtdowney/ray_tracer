use crate::{Color, Matrix4, Pattern, Point};
use std::any::Any;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GradientPattern {
    a: Color,
    b: Color,
    transform: Matrix4,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        GradientPattern {
            a,
            b,
            transform: Matrix4::identity(),
        }
    }
}

impl PartialEq<Pattern> for GradientPattern {
    fn eq(&self, other: &Pattern) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |x| self == x)
    }
}

impl Pattern for GradientPattern {
    fn as_any(&self) -> &Any {
        self
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn pattern_at(&self, Point { x, .. }: Point) -> Color {
        let distance = self.b - self.a;
        let fraction = x - x.floor();
        self.a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    #[test]
    fn test_gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(
            Color::new(0.75, 0.75, 0.75),
            pattern.pattern_at(Point::new(0.25, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            pattern.pattern_at(Point::new(0.5, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.25, 0.25, 0.25),
            pattern.pattern_at(Point::new(0.75, 0.0, 0.0))
        );
    }
}
