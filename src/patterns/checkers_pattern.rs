use crate::{Color, Matrix4, Pattern, Point};

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

impl Pattern for CheckersPattern {
    fn box_clone(&self) -> Box<Pattern + Sync + Send> {
        Box::new(*self)
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
    fn checkers_repeats_in_x() {
        let pattern = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.99, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(1.01, 0.0, 0.0)));
    }

    #[test]
    fn checkers_repeats_in_y() {
        let pattern = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.99, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(0.0, 1.01, 0.0)));
    }

    #[test]
    fn checkers_repeats_in_z() {
        let pattern = CheckersPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.99)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(0.0, 0.0, 1.01)));
    }
}
