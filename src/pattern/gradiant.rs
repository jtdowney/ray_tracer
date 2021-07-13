use crate::{Color, Matrix4, Pattern};
use derive_builder::Builder;

pub fn gradiant_pattern(a: Color, b: Color) -> GradiantPattern {
    GradiantPatternBuilder::default().a(a).b(b).build().unwrap()
}

#[derive(Builder)]
pub struct GradiantPattern {
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
    a: Color,
    b: Color,
}

impl Pattern for GradiantPattern {
    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform
    }

    fn pattern_at(&self, point: crate::Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color, point};

    #[test]
    fn gradiant_linearly_interpolates_colors() {
        let pattern = gradiant_pattern(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(
            pattern.pattern_at(point(0.25, 0.0, 0.0)),
            color(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(point(0.5, 0.0, 0.0)),
            color(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(point(0.75, 0.0, 0.0)),
            color(0.25, 0.25, 0.25)
        );
    }
}
