mod checkers;
mod gradiant;
mod ring;
mod stripe;

use crate::{Color, Matrix4, Point, Shape};
pub use checkers::*;
pub use gradiant::*;
pub use ring::*;
pub use stripe::*;

pub trait Pattern {
    fn transform(&self) -> &Matrix4;
    fn set_transform(&mut self, transform: Matrix4);
    fn pattern_at(&self, point: Point) -> Color;

    fn pattern_at_shape(&self, shape: &dyn Shape, world_point: Point) -> Color {
        let inverse_shape_transform = shape.transform().inverse();
        let inverse_pattern_transform = self.transform().inverse();
        let object_point = inverse_shape_transform * world_point;
        let pattern_point = inverse_pattern_transform * object_point;
        self.pattern_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::{color, point, sphere, transformations};

    use super::*;
    use derive_builder::Builder;

    fn test_pattern() -> TestPattern {
        TestPatternBuilder::default().build().unwrap()
    }

    #[derive(Builder)]
    struct TestPattern {
        #[builder(default = "Matrix4::identity()")]
        transform: Matrix4,
    }

    impl Pattern for TestPattern {
        fn transform(&self) -> &Matrix4 {
            &self.transform
        }

        fn set_transform(&mut self, transform: Matrix4) {
            self.transform = transform
        }

        fn pattern_at(&self, point: Point) -> Color {
            color(point.x, point.y, point.z)
        }
    }

    #[test]
    fn default_transform() {
        let pattern = test_pattern();
        assert_eq!(pattern.transform(), &Matrix4::identity());
    }

    #[test]
    fn pattern_with_object_transformation() {
        let mut object = sphere();
        object.set_transform(transformations::scaling(2.0, 2.0, 2.0));
        let pattern = test_pattern();
        let c = pattern.pattern_at_shape(&object, point(2.0, 3.0, 4.0));
        assert_eq!(c, color(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let object = sphere();
        let mut pattern = test_pattern();
        pattern.set_transform(transformations::scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_shape(&object, point(2.0, 3.0, 4.0));
        assert_eq!(c, color(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_both_object_and_pattern_transformation() {
        let mut object = sphere();
        object.set_transform(transformations::scaling(2.0, 2.0, 2.0));
        let mut pattern = test_pattern();
        pattern.set_transform(transformations::translation(0.5, 1.0, 1.5));
        let c = pattern.pattern_at_shape(&object, point(2.5, 3.0, 3.5));
        assert_eq!(c, color(0.75, 0.5, 0.25));
    }
}
