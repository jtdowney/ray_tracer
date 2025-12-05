use crate::{Matrix4, Point, Vector};

#[must_use]
pub fn ray(origin: Point, direction: Vector) -> Ray {
    Ray { origin, direction }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn position(self, t: impl Into<f64>) -> Point {
        self.origin + self.direction * t.into()
    }

    #[must_use]
    pub fn transform(&self, transform: Matrix4) -> Ray {
        let origin = transform * self.origin;
        let direction = transform * self.direction;
        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, transform, vector};

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1, 2, 3);
        let direction = vector(4, 5, 6);
        let r = ray(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = ray(point(2, 3, 4), vector(1, 0, 0));
        assert_eq!(r.position(0), point(2, 3, 4));
        assert_eq!(r.position(1), point(3, 3, 4));
        assert_eq!(r.position(-1), point(1, 3, 4));
        assert_eq!(r.position(2.5), point(4.5, 3, 4));
    }

    #[test]
    fn translating_a_ray() {
        let r = ray(point(1, 2, 3), vector(0, 1, 0));
        let m = transform::translation(3, 4, 5);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(4, 6, 8));
        assert_eq!(r2.direction, vector(0, 1, 0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = ray(point(1, 2, 3), vector(0, 1, 0));
        let m = transform::scaling(2, 3, 4);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(2, 6, 12));
        assert_eq!(r2.direction, vector(0, 3, 0));
    }
}
