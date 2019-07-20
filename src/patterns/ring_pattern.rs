use crate::{Color, Matrix4, Pattern, Point};
use derive_builder::Builder;

#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct RingPattern {
    first: Color,
    second: Color,
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
}

impl Pattern for RingPattern {
    fn box_clone(&self) -> Box<dyn Pattern + Sync + Send> {
        Box::new(*self)
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn pattern_at(&self, Point { x, z, .. }: Point) -> Color {
        if (z.powi(2) + x.powi(2)).sqrt().floor() as u32 % 2 == 0 {
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
    fn ring_extends_in_both_x_and_z() {
        let pattern = RingPatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(1.0, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(0.0, 0.0, 1.0)));
        assert_eq!(
            color::BLACK,
            pattern.pattern_at(Point::new(0.708, 0.0, 0.708))
        );
    }
}
