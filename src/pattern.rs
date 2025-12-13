use std::rc::Rc;

use bon::builder;

use crate::{Color, Matrix4, Point, Shape, identity_matrix};

#[derive(Clone)]
pub struct Pattern {
    pub transform: Matrix4,
    point_to_color: Rc<dyn Fn(Point) -> Color>,
}

impl Pattern {
    #[must_use]
    pub fn pattern_at(&self, point: Point) -> Color {
        (self.point_to_color)(point)
    }

    /// # Panics
    /// Panics if the shape's or pattern's transform matrix is not invertible.
    #[must_use]
    pub fn pattern_at_shape(&self, shape: &Shape, world_point: Point) -> Color {
        let object_point = shape.world_to_object(world_point);
        let pattern_point = self.transform.inverse().expect("invertible") * object_point;
        self.pattern_at(pattern_point)
    }
}

#[must_use]
#[builder(finish_fn = build)]
pub fn stripe_pattern(
    #[builder(start_fn)] a: Color,
    #[builder(start_fn)] b: Color,
    #[builder(default = identity_matrix())] transform: Matrix4,
) -> Pattern {
    Pattern {
        transform,
        point_to_color: Rc::new(move |Point { x, .. }| {
            #[allow(clippy::cast_possible_truncation)]
            let value = x.floor() as i32;
            if value % 2 == 0 { a } else { b }
        }),
    }
}

#[must_use]
#[builder(finish_fn = build)]
pub fn gradient_pattern(
    #[builder(start_fn)] a: Color,
    #[builder(start_fn)] b: Color,
    #[builder(default = identity_matrix())] transform: Matrix4,
) -> Pattern {
    Pattern {
        transform,
        point_to_color: Rc::new(move |Point { x, .. }| {
            let distance = b - a;
            let fraction = x - x.floor();
            a + distance * fraction
        }),
    }
}

#[must_use]
#[builder(finish_fn = build)]
pub fn ring_pattern(
    #[builder(start_fn)] a: Color,
    #[builder(start_fn)] b: Color,
    #[builder(default = identity_matrix())] transform: Matrix4,
) -> Pattern {
    Pattern {
        transform,
        point_to_color: Rc::new(move |Point { x, z, .. }| {
            #[allow(clippy::cast_possible_truncation)]
            let value = (x.powi(2) + z.powi(2)).sqrt().floor() as i32;
            if value % 2 == 0 { a } else { b }
        }),
    }
}

#[must_use]
#[builder(finish_fn = build)]
pub fn checkers_pattern(
    #[builder(start_fn)] a: Color,
    #[builder(start_fn)] b: Color,
    #[builder(default = identity_matrix())] transform: Matrix4,
) -> Pattern {
    Pattern {
        transform,
        point_to_color: Rc::new(move |Point { x, y, z }| {
            #[allow(clippy::cast_possible_truncation)]
            let value = (x.floor() + y.floor() + z.floor()) as i32;
            if value % 2 == 0 { a } else { b }
        }),
    }
}

/// A test pattern that returns a color based on the point's coordinates.
/// Used for testing pattern transformations.
#[must_use]
pub fn test_pattern() -> Pattern {
    Pattern {
        transform: identity_matrix(),
        point_to_color: Rc::new(|Point { x, y, z }| crate::color(x, y, z)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color,
        color::{BLACK, WHITE},
        point, sphere, transform,
    };

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = stripe_pattern(WHITE, BLACK).build();
        assert_eq!(pattern.pattern_at(point(0, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(0, 1, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(0, 2, 0)), WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = stripe_pattern(WHITE, BLACK).build();
        assert_eq!(pattern.pattern_at(point(0, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(0, 0, 1)), WHITE);
        assert_eq!(pattern.pattern_at(point(0, 0, 2)), WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(WHITE, BLACK).build();
        assert_eq!(pattern.pattern_at(point(0, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(0.9, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(1, 0, 0)), BLACK);
        assert_eq!(pattern.pattern_at(point(-0.1, 0, 0)), BLACK);
        assert_eq!(pattern.pattern_at(point(-1, 0, 0)), BLACK);
        assert_eq!(pattern.pattern_at(point(-1.1, 0, 0)), WHITE);
    }

    #[test]
    fn stripes_with_object_transformation() {
        let object = sphere().transform(transform::scaling(2, 2, 2)).build();
        let pattern = stripe_pattern(WHITE, BLACK).build();
        let c = pattern.pattern_at_shape(&object, point(1.5, 0, 0));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = sphere().build();
        let pattern = stripe_pattern(WHITE, BLACK)
            .transform(transform::scaling(2, 2, 2))
            .build();
        let c = pattern.pattern_at_shape(&object, point(1.5, 0, 0));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let object = sphere().transform(transform::scaling(2, 2, 2)).build();
        let pattern = stripe_pattern(WHITE, BLACK)
            .transform(transform::translation(0.5, 0, 0))
            .build();
        let c = pattern.pattern_at_shape(&object, point(2.5, 0, 0));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn default_pattern_transformation() {
        let pattern = test_pattern();
        assert_eq!(pattern.transform, identity_matrix());
    }

    #[test]
    fn assigning_pattern_transformation() {
        let pattern = stripe_pattern(WHITE, BLACK)
            .transform(transform::translation(1, 2, 3))
            .build();
        assert_eq!(pattern.transform, transform::translation(1, 2, 3));
    }

    #[test]
    fn pattern_with_object_transformation() {
        let shape = sphere().transform(transform::scaling(2, 2, 2)).build();
        let pattern = test_pattern();
        let c = pattern.pattern_at_shape(&shape, point(2, 3, 4));
        assert_eq!(c, color(1, 1.5, 2));
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let shape = sphere().build();
        let pattern = Pattern {
            transform: transform::scaling(2, 2, 2),
            point_to_color: Rc::new(|Point { x, y, z }| color(x, y, z)),
        };
        let c = pattern.pattern_at_shape(&shape, point(2, 3, 4));
        assert_eq!(c, color(1, 1.5, 2));
    }

    #[test]
    fn pattern_with_both_object_and_pattern_transformation() {
        let shape = sphere().transform(transform::scaling(2, 2, 2)).build();
        let pattern = Pattern {
            transform: transform::translation(0.5, 1, 1.5),
            point_to_color: Rc::new(|Point { x, y, z }| color(x, y, z)),
        };
        let c = pattern.pattern_at_shape(&shape, point(2.5, 3, 3.5));
        assert_eq!(c, color(0.75, 0.5, 0.25));
    }

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = gradient_pattern(WHITE, BLACK).build();
        assert_eq!(pattern.pattern_at(point(0, 0, 0)), WHITE);
        assert_eq!(
            pattern.pattern_at(point(0.25, 0, 0)),
            color(0.75, 0.75, 0.75)
        );
        assert_eq!(pattern.pattern_at(point(0.5, 0, 0)), color(0.5, 0.5, 0.5));
        assert_eq!(
            pattern.pattern_at(point(0.75, 0, 0)),
            color(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn ring_extends_in_both_x_and_z() {
        let pattern = ring_pattern(WHITE, BLACK).build();
        assert_eq!(pattern.pattern_at(point(0, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(1, 0, 0)), BLACK);
        assert_eq!(pattern.pattern_at(point(0, 0, 1)), BLACK);
        assert_eq!(pattern.pattern_at(point(0.708, 0, 0.708)), BLACK);
    }

    #[test]
    fn checkers_repeat_in_x() {
        let pattern = checkers_pattern(WHITE, BLACK).build();
        assert_eq!(pattern.pattern_at(point(0, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(0.99, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(1.01, 0, 0)), BLACK);
    }

    #[test]
    fn checkers_repeat_in_y() {
        let pattern = checkers_pattern(WHITE, BLACK).build();
        assert_eq!(pattern.pattern_at(point(0, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(0, 0.99, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(0, 1.01, 0)), BLACK);
    }

    #[test]
    fn checkers_repeat_in_z() {
        let pattern = checkers_pattern(WHITE, BLACK).build();
        assert_eq!(pattern.pattern_at(point(0, 0, 0)), WHITE);
        assert_eq!(pattern.pattern_at(point(0, 0, 0.99)), WHITE);
        assert_eq!(pattern.pattern_at(point(0, 0, 1.01)), BLACK);
    }
}
