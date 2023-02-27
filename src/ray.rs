use crate::{Matrix4, Point, Vector};

pub fn ray(origin: Point, direction: Vector) -> Ray {
    Ray { origin, direction }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn position<T: Into<f64>>(self, t: T) -> Point {
        self.origin + self.direction * t.into()
    }

    pub fn transform(&self, transform: Matrix4) -> Ray {
        let origin = transform * self.origin;
        let direction = transform * self.direction;
        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        point,
        transform::{scaling, translation},
        vector,
    };

    use super::*;

    #[test]
    fn computing_point_from_distance() {
        let r = ray(point(2, 3, 4), vector(1, 0, 0));
        assert_eq!(point(2, 3, 4), r.position(0));
        assert_eq!(point(3, 3, 4), r.position(1));
        assert_eq!(point(1, 3, 4), r.position(-1));
        assert_eq!(point(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn translating_ray() {
        let r = ray(point(1, 2, 3), vector(0, 1, 0));
        let m = translation(3, 4, 5);
        let r2 = r.transform(m);
        assert_eq!(point(4, 6, 8), r2.origin);
        assert_eq!(vector(0, 1, 0), r2.direction);
    }

    #[test]
    fn scaling_ray() {
        let r = ray(point(1, 2, 3), vector(0, 1, 0));
        let m = scaling(2, 3, 4);
        let r2 = r.transform(m);
        assert_eq!(point(2, 6, 12), r2.origin);
        assert_eq!(vector(0, 3, 0), r2.direction);
    }
}
