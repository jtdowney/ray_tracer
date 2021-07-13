use crate::{Color, Matrix4, Pattern};
use derive_builder::Builder;
use num::Integer;

pub fn ring_pattern(a: Color, b: Color) -> RingPattern {
    RingPatternBuilder::default().a(a).b(b).build().unwrap()
}

#[derive(Builder)]
pub struct RingPattern {
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
    a: Color,
    b: Color,
}

impl Pattern for RingPattern {
    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform
    }

    fn pattern_at(&self, point: crate::Point) -> Color {
        let floor = (point.x.powi(2) + point.z.powi(2)).sqrt().floor() as i32;
        if floor.is_even() {
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
    fn ring_extends_in_both_x_and_z() {
        let pattern = ring_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.708, 0.0, 0.708)), color::BLACK);
    }
}
