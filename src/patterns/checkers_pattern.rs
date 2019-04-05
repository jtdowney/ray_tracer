use crate::{Color, Matrix4, Pattern, Point};
use derive_builder::Builder;

#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct CheckersPattern {
    first: Color,
    second: Color,
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
}

impl Pattern for CheckersPattern {
    fn box_clone(&self) -> Box<Pattern + Sync + Send> {
        Box::new(*self)
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn pattern_at(&self, Point { x, y, z }: Point) -> Color {
        if (x.floor() + y.floor() + z.floor()) as u32 % 2 == 0 {
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
    fn checkers_repeats_in_x() {
        let pattern = CheckersPatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.99, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(1.01, 0.0, 0.0)));
    }

    #[test]
    fn checkers_repeats_in_y() {
        let pattern = CheckersPatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.99, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(0.0, 1.01, 0.0)));
    }

    #[test]
    fn checkers_repeats_in_z() {
        let pattern = CheckersPatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.99)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(0.0, 0.0, 1.01)));
    }
}
