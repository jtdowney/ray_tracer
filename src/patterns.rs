use crate::{Color, Matrix4, Point, Shape};
use std::any::Any;
use std::fmt::Debug;

mod checkers_pattern;
mod gradient_pattern;
mod ring_pattern;
mod solid_pattern;
mod stripe_pattern;

pub use self::checkers_pattern::CheckersPattern;
pub use self::gradient_pattern::GradientPattern;
pub use self::ring_pattern::RingPattern;
pub use self::solid_pattern::SolidPattern;
pub use self::stripe_pattern::StripePattern;

pub trait Pattern: Any + Debug {
    fn as_any(&self) -> &Any;
    fn pattern_at(&self, point: Point) -> Color;
    fn transform(&self) -> Matrix4;

    fn pattern_at_object(&self, object: &Shape, world_point: Point) -> Color {
        let object_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform().inverse() * object_point;
        self.pattern_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transforms, Sphere};

    #[derive(Debug)]
    struct TestPattern {
        transform: Matrix4,
    }

    impl Default for TestPattern {
        fn default() -> Self {
            TestPattern {
                transform: Matrix4::identity(),
            }
        }
    }

    impl Pattern for TestPattern {
        fn as_any(&self) -> &Any {
            self
        }
        fn pattern_at(&self, Point { x, y, z }: Point) -> Color {
            Color::new(x, y, z)
        }
        fn transform(&self) -> Matrix4 {
            self.transform
        }
    }

    #[test]
    fn test_pattern_with_object_transformation() {
        let mut object = Sphere::default();
        object.transform = transforms::scaling(2.0, 2.0, 2.0);
        let pattern = TestPattern::default();

        assert_eq!(
            Color::new(1.0, 1.5, 2.0),
            pattern.pattern_at_object(&object, Point::new(2.0, 3.0, 4.0))
        );
    }

    #[test]
    fn test_pattern_with_pattern_transformation() {
        let object = Sphere::default();
        let mut pattern = TestPattern::default();
        pattern.transform = transforms::scaling(2.0, 2.0, 2.0);

        assert_eq!(
            Color::new(1.0, 1.5, 2.0),
            pattern.pattern_at_object(&object, Point::new(2.0, 3.0, 4.0))
        );
    }

    #[test]
    fn test_pattern_with_object_and_pattern_transformation() {
        let mut object = Sphere::default();
        object.transform = transforms::scaling(2.0, 2.0, 2.0);
        let mut pattern = TestPattern::default();
        pattern.transform = transforms::translation(0.5, 1.0, 1.5);

        assert_eq!(
            Color::new(0.75, 0.5, 0.25),
            pattern.pattern_at_object(&object, Point::new(2.5, 3.0, 3.5))
        );
    }
}
