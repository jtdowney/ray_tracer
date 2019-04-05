use crate::{Color, Matrix4, Pattern, Point};
use derive_builder::Builder;

#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct SolidPattern {
    color: Color,
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
}

impl Pattern for SolidPattern {
    fn box_clone(&self) -> Box<Pattern + Sync + Send> {
        Box::new(*self)
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
    fn solid_pattern_returns_constant_color() {
        let pattern = SolidPatternBuilder::default()
            .color(color::WHITE)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(1.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(1.0, 1.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(1.0, 1.0, 1.0)));
    }
}
