use crate::{Color, Matrix4, Pattern, Point};
use derive_builder::Builder;

#[derive(Builder, Copy, Clone, Debug, PartialEq)]
pub struct GradientPattern {
    first: Color,
    second: Color,
    #[builder(default = "Matrix4::identity()")]
    transform: Matrix4,
}

impl Pattern for GradientPattern {
    fn box_clone(&self) -> Box<Pattern + Sync + Send> {
        Box::new(*self)
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn pattern_at(&self, Point { x, .. }: Point) -> Color {
        let distance = self.second - self.first;
        let fraction = x - x.floor();
        self.first + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPatternBuilder::default()
            .first(color::WHITE)
            .second(color::BLACK)
            .build()
            .unwrap();
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
