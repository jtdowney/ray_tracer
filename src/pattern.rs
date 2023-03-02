use std::{
    fmt::{self, Debug},
    sync::Arc,
};

use crate::{identity_matrix, Color, Matrix4, Point, Shape};

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern::new(move |Point { x, .. }| {
        let value = x.floor() as i32;
        if value % 2 == 0 {
            a
        } else {
            b
        }
    })
}

pub fn gradiant_pattern(a: Color, b: Color) -> Pattern {
    Pattern::new(move |Point { x, .. }| {
        let distance = b - a;
        let fraction = x - x.floor();
        a + distance * fraction
    })
}

pub fn ring_pattern(a: Color, b: Color) -> Pattern {
    Pattern::new(move |Point { x, z, .. }| {
        let value = (x.powi(2) + z.powi(2)).sqrt().floor() as i32;
        if value % 2 == 0 {
            a
        } else {
            b
        }
    })
}

pub fn checkers_pattern(a: Color, b: Color) -> Pattern {
    Pattern::new(move |Point { x, y, z }| {
        let value = (x.floor() + y.floor() + z.floor()) as i32;
        if value % 2 == 0 {
            a
        } else {
            b
        }
    })
}

#[derive(Clone)]
pub struct Pattern {
    pub transform: Matrix4,
    point_to_color: Arc<dyn Send + Sync + Fn(Point) -> Color>,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pattern")
            .field("transform", &self.transform)
            .finish()
    }
}

impl Pattern {
    fn new<F: Fn(Point) -> Color + Send + Sync + 'static>(point_to_color: F) -> Self {
        Self {
            transform: identity_matrix(),
            point_to_color: Arc::new(point_to_color),
        }
    }

    pub fn pattern_at_shape(&self, shape: &Shape, world_point: Point) -> Color {
        let object_point = shape.transform.inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.pattern_at(pattern_point)
    }

    fn pattern_at(&self, point: Point) -> Color {
        (self.point_to_color)(point)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color, point, sphere,
        transform::{scaling, translation},
        BLACK, ORIGIN, WHITE,
    };

    use super::*;

    fn test_pattern() -> Pattern {
        Pattern::new(|Point { x, y, z }| color(x, y, z))
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_eq!(WHITE, pattern.pattern_at(point(0, 0, 0)));
        assert_eq!(WHITE, pattern.pattern_at(point(0, 1, 0)));
        assert_eq!(WHITE, pattern.pattern_at(point(0, 2, 0)));
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_eq!(WHITE, pattern.pattern_at(point(0, 0, 0)));
        assert_eq!(WHITE, pattern.pattern_at(point(0, 0, 1)));
        assert_eq!(WHITE, pattern.pattern_at(point(0, 0, 2)));
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_eq!(WHITE, pattern.pattern_at(point(0, 0, 0)));
        assert_eq!(WHITE, pattern.pattern_at(point(0.9, 0.0, 0.0)));
        assert_eq!(BLACK, pattern.pattern_at(point(1, 0, 0)));
        assert_eq!(BLACK, pattern.pattern_at(point(-0.1, 0.0, 0.0)));
        assert_eq!(BLACK, pattern.pattern_at(point(-1, 0, 0)));
        assert_eq!(WHITE, pattern.pattern_at(point(-1.1, 0.0, 0.0)));
    }

    #[test]
    fn pattern_with_object_transformation() {
        let mut shape = sphere();
        shape.transform = scaling(2, 2, 2);
        let pattern = test_pattern();
        assert_eq!(
            color(1.0, 1.5, 2.0),
            pattern.pattern_at_shape(&shape, point(2, 3, 4))
        );
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let shape = sphere();
        let mut pattern = test_pattern();
        pattern.transform = scaling(2, 2, 2);
        assert_eq!(
            color(1.0, 1.5, 2.0),
            pattern.pattern_at_shape(&shape, point(2, 3, 4))
        );
    }

    #[test]
    fn pattern_with_object_and_pattern_transformation() {
        let mut object = sphere();
        object.transform = scaling(2, 2, 2);
        let mut pattern = test_pattern();
        pattern.transform = translation(0.5, 1.0, 1.5);
        assert_eq!(
            color(0.75, 0.5, 0.25),
            pattern.pattern_at_shape(&object, point(2.5, 3.0, 3.5))
        );
    }

    #[test]
    fn gradiant_pattern_interpolates_between_colors() {
        let pattern = gradiant_pattern(WHITE, BLACK);
        assert_eq!(WHITE, pattern.pattern_at(ORIGIN));
        assert_eq!(
            color(0.75, 0.75, 0.75),
            pattern.pattern_at(point(0.25, 0.0, 0.0))
        );
        assert_eq!(
            color(0.5, 0.5, 0.5),
            pattern.pattern_at(point(0.5, 0.0, 0.0))
        );
        assert_eq!(
            color(0.25, 0.25, 0.25),
            pattern.pattern_at(point(0.75, 0.0, 0.0))
        );
    }

    #[test]
    fn ring_pattern_should_extend_in_x_and_z() {
        let pattern = ring_pattern(WHITE, BLACK);
        assert_eq!(WHITE, pattern.pattern_at(ORIGIN));
        assert_eq!(BLACK, pattern.pattern_at(point(1, 0, 0)));
        assert_eq!(BLACK, pattern.pattern_at(point(0, 0, 1)));
        assert_eq!(BLACK, pattern.pattern_at(point(0.708, 0.0, 0.708)));
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = checkers_pattern(WHITE, BLACK);
        assert_eq!(WHITE, pattern.pattern_at(ORIGIN));
        assert_eq!(WHITE, pattern.pattern_at(point(0.99, 0.0, 0.0)));
        assert_eq!(BLACK, pattern.pattern_at(point(1.01, 0.0, 0.0)));
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = checkers_pattern(WHITE, BLACK);
        assert_eq!(WHITE, pattern.pattern_at(ORIGIN));
        assert_eq!(WHITE, pattern.pattern_at(point(0.0, 0.99, 0.0)));
        assert_eq!(BLACK, pattern.pattern_at(point(0.0, 1.01, 0.0)));
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = checkers_pattern(WHITE, BLACK);
        assert_eq!(WHITE, pattern.pattern_at(ORIGIN));
        assert_eq!(WHITE, pattern.pattern_at(point(0.0, 0.0, 0.99)));
        assert_eq!(BLACK, pattern.pattern_at(point(0.0, 0.0, 1.01)));
    }
}
