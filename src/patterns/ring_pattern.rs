use crate::{Color, Matrix4, Pattern, Point};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RingPattern {
    a: Color,
    b: Color,
    transform: Matrix4,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        RingPattern {
            a,
            b,
            transform: Matrix4::identity(),
        }
    }
}

impl Pattern for RingPattern {
    fn box_clone(&self) -> Box<Pattern> {
        Box::new((*self).clone())
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn pattern_at(&self, Point { x, z, .. }: Point) -> Color {
        if (z.powi(2) + x.powi(2)).sqrt().floor() as u32 % 2 == 0 {
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
    fn test_ring_extends_in_both_x_and_z() {
        let pattern = RingPattern::new(color::WHITE, color::BLACK);
        assert_eq!(color::WHITE, pattern.pattern_at(Point::new(0.0, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(1.0, 0.0, 0.0)));
        assert_eq!(color::BLACK, pattern.pattern_at(Point::new(0.0, 0.0, 1.0)));
        assert_eq!(
            color::BLACK,
            pattern.pattern_at(Point::new(0.708, 0.0, 0.708))
        );
    }
}
