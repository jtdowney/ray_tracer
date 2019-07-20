use crate::{Color, Matrix4, Point, Shape};
use std::fmt::Debug;

mod checkers_pattern;
mod gradient_pattern;
mod ring_pattern;
mod solid_pattern;
mod stripe_pattern;

pub use self::checkers_pattern::{CheckersPattern, CheckersPatternBuilder};
pub use self::gradient_pattern::{GradientPattern, GradientPatternBuilder};
pub use self::ring_pattern::{RingPattern, RingPatternBuilder};
pub use self::solid_pattern::{SolidPattern, SolidPatternBuilder};
pub use self::stripe_pattern::{StripePattern, StripePatternBuilder};

pub trait Pattern: Debug {
    fn box_clone(&self) -> Box<dyn Pattern + Sync + Send>;
    fn pattern_at(&self, point: Point) -> Color;
    fn transform(&self) -> Matrix4;

    fn pattern_at_object(&self, object: &dyn Shape, world_point: Point) -> Color {
        let object_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform().inverse() * object_point;
        self.pattern_at(pattern_point)
    }
}

impl Clone for Box<dyn Pattern + Sync + Send> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{transforms, Sphere, SphereBuilder};

    #[derive(Copy, Clone, Debug)]
    pub struct TestPattern {
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
        fn box_clone(&self) -> Box<dyn Pattern + Sync + Send> {
            Box::new(*self)
        }

        fn pattern_at(&self, Point { x, y, z }: Point) -> Color {
            Color::new(x as f32, y as f32, z as f32)
        }

        fn transform(&self) -> Matrix4 {
            self.transform
        }
    }

    #[test]
    fn pattern_with_object_transformation() {
        let object = SphereBuilder::default()
            .transform(transforms::scaling(2.0, 2.0, 2.0))
            .build()
            .unwrap();
        let pattern = TestPattern::default();

        assert_eq!(
            Color::new(1.0, 1.5, 2.0),
            pattern.pattern_at_object(&object, Point::new(2.0, 3.0, 4.0))
        );
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let object = Sphere::default();
        let mut pattern = TestPattern::default();
        pattern.transform = transforms::scaling(2.0, 2.0, 2.0);

        assert_eq!(
            Color::new(1.0, 1.5, 2.0),
            pattern.pattern_at_object(&object, Point::new(2.0, 3.0, 4.0))
        );
    }

    #[test]
    fn pattern_with_object_and_pattern_transformation() {
        let object = SphereBuilder::default()
            .transform(transforms::scaling(2.0, 2.0, 2.0))
            .build()
            .unwrap();
        let mut pattern = TestPattern::default();
        pattern.transform = transforms::translation(0.5, 1.0, 1.5);

        assert_eq!(
            Color::new(0.75, 0.5, 0.25),
            pattern.pattern_at_object(&object, Point::new(2.5, 3.0, 3.5))
        );
    }
}
