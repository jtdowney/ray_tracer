use crate::{intersection, intersection::Intersection, vector, Shape, EPSILON};

use super::Geometry;

pub fn cube() -> Shape {
    Cube.into()
}

#[derive(Clone, Copy, Debug)]
pub struct Cube;

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let (tmin, tmax) = if direction.abs() >= EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * f64::INFINITY,
            tmax_numerator * f64::INFINITY,
        )
    };

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl Geometry for Cube {
    fn local_intersection<'a>(
        &'a self,
        shape: &'a Shape,
        ray: crate::Ray,
    ) -> Vec<Intersection<'a>> {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin < tmax {
            vec![intersection(tmin, shape), intersection(tmax, shape)]
        } else {
            vec![]
        }
    }

    fn local_normal_at(&self, point: crate::Point) -> crate::Vector {
        let maxc = point.x.abs().max(point.y.abs()).max(point.z.abs());

        if maxc == point.x.abs() {
            vector(point.x, 0.0, 0.0)
        } else if maxc == point.y.abs() {
            vector(0.0, point.y, 0.0)
        } else {
            vector(0.0, 0.0, point.z)
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{point, ray, vector};

    use super::*;

    #[test]
    fn ray_intersects_cube() {
        let tests = [
            (point(5.0, 0.5, 0.0), vector(-1, 0, 0), 4, 6),
            (point(-5.0, 0.5, 0.0), vector(1, 0, 0), 4, 6),
            (point(0.5, 5.0, 0.0), vector(0, -1, 0), 4, 6),
            (point(0.5, -5.0, 0.0), vector(0, 1, 0), 4, 6),
            (point(0.5, 0.0, 5.0), vector(0, 0, -1), 4, 6),
            (point(0.5, 0.0, -5.0), vector(0, 0, 1), 4, 6),
            (point(0.0, 0.5, 0.0), vector(0, 0, 1), -1, 1),
        ];

        let c = cube();
        for (origin, direction, t1, t2) in tests {
            let r = ray(origin, direction);
            let xs = c.intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].time, t1.into());
            assert_eq!(xs[1].time, t2.into());
        }
    }

    #[test]
    fn ray_missing_cube() {
        let tests = [
            (point(-2, 0, 0), vector(0.2673, 0.5345, 0.8018)),
            (point(0, -2, 0), vector(0.8018, 0.2673, 0.5345)),
            (point(0, 0, -2), vector(0.5345, 0.8018, 0.2673)),
            (point(2, 0, 2), vector(0, 0, -1)),
            (point(0, 2, 2), vector(0, -1, 0)),
            (point(2, 2, 0), vector(-1, 0, 0)),
        ];

        let c = cube();
        for (origin, direction) in tests {
            let r = ray(origin, direction);
            let xs = c.intersect(r);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn normal_on_surface_of_cube() {
        let tests = [
            (point(1.0, 0.5, -0.8), vector(1, 0, 0)),
            (point(-1.0, -0.2, 0.9), vector(-1, 0, 0)),
            (point(-0.4, 1.0, -0.1), vector(0, 1, 0)),
            (point(0.3, -1.0, -0.7), vector(0, -1, 0)),
            (point(-0.6, 0.3, 1.0), vector(0, 0, 1)),
            (point(0.4, 0.4, -1.0), vector(0, 0, -1)),
            (point(1, 1, 1), vector(1, 0, 0)),
            (point(-1, -1, -1), vector(-1, 0, 0)),
        ];

        let c = cube();
        for (point, normal) in tests {
            assert_eq!(normal, c.normal_at(point));
        }
    }
}
