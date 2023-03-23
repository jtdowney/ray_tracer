use std::mem;

use approx::relative_eq;

use crate::{intersection, intersection::Intersection, vector, Point, Ray, Shape, EPSILON};

use super::Geometry;

pub fn cylinder() -> Shape {
    Cylinder::default().into()
}

#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }
}

fn check_cap(ray: Ray, time: f64) -> bool {
    let x = ray.origin.x + time * ray.direction.x;
    let z = ray.origin.z + time * ray.direction.z;
    x.powi(2) + z.powi(2) <= 1.0
}

fn intersect_caps(cylinder: &Cylinder, ray: Ray) -> Vec<f64> {
    let mut xs = vec![];
    if !cylinder.closed || relative_eq!(ray.direction.y, 0.0, epsilon = EPSILON) {
        return xs;
    }

    // Check for an intersection with the lower end cap by intersecting
    // the ray with the plane at y=cylinder.minimum
    let t = (cylinder.minimum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t) {
        xs.push(t);
    }

    // Check for an intersection with the upper end cap by intersecting
    // the ray with the plane at y=cylinder.maximum
    let t = (cylinder.maximum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t) {
        xs.push(t);
    }

    xs
}

impl Geometry for Cylinder {
    fn local_intersection<'a>(&'a self, shape: &'a crate::Shape, ray: Ray) -> Vec<Intersection> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return vec![];
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
        if t0 > t1 {
            mem::swap(&mut t0, &mut t1);
        }

        let mut xs = vec![];
        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(intersection(t0, shape));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(intersection(t1, shape));
        }

        let cap_xs = intersect_caps(self, ray)
            .into_iter()
            .map(|t| intersection(t, shape));

        xs.extend(cap_xs);
        xs
    }

    fn local_normal_at(&self, Point { x, y, z }: Point) -> crate::Vector {
        let dist = x.powi(2) + z.powi(2);
        if dist < 1.0 && y > self.maximum - EPSILON {
            vector(0, 1, 0)
        } else if dist < 1.0 && y < self.minimum + EPSILON {
            vector(0, -1, 0)
        } else {
            vector(x, 0.0, z)
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{point, ray, EPSILON};

    use super::*;

    #[test]
    fn ray_misses_cylinder() {
        let tests = [
            (point(1, 0, 0), vector(0, 1, 0)),
            (point(0, 0, 0), vector(0, 1, 0)),
            (point(0, 0, -5), vector(1, 1, 1)),
        ];

        let cyl = cylinder();
        for (origin, direction) in tests {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = cyl.intersect(r);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn ray_hits_cylinder() {
        let tests = [
            (point(1, 0, -5), vector(0, 0, 1), 5.0, 5.0),
            (point(0, 0, -5), vector(0, 0, 1), 4.0, 6.0),
            (
                point(0.5, 0.0, -5.0),
                vector(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];

        let cyl = cylinder();
        for (origin, direction, t0, t1) in tests {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = cyl.intersect(r);
            assert_eq!(2, xs.len());
            assert_abs_diff_eq!(t0, xs[0].time, epsilon = EPSILON);
            assert_abs_diff_eq!(t1, xs[1].time, epsilon = EPSILON);
        }
    }

    #[test]
    fn normal_vector_on_cylinder() {
        let tests = [
            (point(1, 0, 0), vector(1, 0, 0)),
            (point(0, 5, -1), vector(0, 0, -1)),
            (point(0, -2, 1), vector(0, 0, 1)),
            (point(-1, 1, 0), vector(-1, 0, 0)),
        ];

        let cyl = cylinder();
        for (point, normal) in tests {
            assert_eq!(normal, cyl.normal_at(point));
        }
    }

    #[test]
    fn intersecting_constrained_cylinders() {
        let tests = [
            (point(0.0, 1.5, 0.0), vector(0.1, 1.0, 0.0), 0),
            (point(0, 3, -5), vector(0, 0, 1), 0),
            (point(0, 0, -5), vector(0, 0, 1), 0),
            (point(0, 2, -5), vector(0, 0, 1), 0),
            (point(0, 1, -5), vector(0, 0, 1), 0),
            (point(0.0, 1.5, -2.0), vector(0, 0, 1), 2),
        ];

        let cyl: Shape = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        }
        .into();
        for (origin, direction, count) in tests {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = cyl.intersect(r);
            assert_eq!(count, xs.len());
        }
    }

    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        let tests = [
            (point(0, 3, 0), vector(0, -1, 0), 2),
            (point(0, 3, -2), vector(0, -1, 2), 2),
            (point(0, 4, -2), vector(0, -1, 1), 2),
            (point(0, 0, -2), vector(0, 1, 2), 2),
            (point(0, -1, -2), vector(0, 1, 1), 2),
        ];

        let cyl: Shape = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
        }
        .into();
        for (origin, direction, count) in tests {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = cyl.intersect(r);
            assert_eq!(count, xs.len());
        }
    }

    #[test]
    fn normal_vector_on_cylinders_end_caps() {
        let tests = [
            (point(0, 1, 0), vector(0, -1, 0)),
            (point(0.5, 1.0, 0.0), vector(0, -1, 0)),
            (point(0.0, 1.0, 0.5), vector(0, -1, 0)),
            (point(0, 2, 0), vector(0, 1, 0)),
            (point(0.5, 2.0, 0.0), vector(0, 1, 0)),
            (point(0.0, 2.0, 0.5), vector(0, 1, 0)),
        ];

        let cyl: Shape = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
        }
        .into();

        for (point, normal) in tests {
            assert_eq!(normal, cyl.normal_at(point));
        }
    }
}
