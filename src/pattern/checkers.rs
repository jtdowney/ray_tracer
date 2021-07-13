use crate::{Color, Matrix4, Pattern};
use derive_builder::Builder;
use num::Integer;

pub fn checkers_pattern(a: Color, b: Color) -> CheckersPattern {
    CheckersPatternBuilder::default().a(a).b(b).build().unwrap()
}

#[derive(Builder)]
pub struct CheckersPattern {
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
    a: Color,
    b: Color,
}

impl Pattern for CheckersPattern {
    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform
    }

    fn pattern_at(&self, point: crate::Point) -> Color {
        let value = (point.x.floor() + point.y.floor() + point.z.floor()) as i32;
        if value.is_even() {
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
    fn checkers_repeats_in_x() {
        let pattern = checkers_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.99, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(1.01, 0.0, 0.0)), color::BLACK);
    }

    #[test]
    fn checkers_repeats_in_y() {
        let pattern = checkers_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.0, 0.99, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.0, 1.01, 0.0)), color::BLACK);
    }

    #[test]
    fn checkers_repeats_in_z() {
        let pattern = checkers_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.99)), color::WHITE);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.01)), color::BLACK);
    }
}
