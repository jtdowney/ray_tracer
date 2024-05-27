use std::{any::Any, mem};

use approx::relative_eq;

use crate::{intersection, intersection::Intersection, vector, Point, Ray, Shape, Vector, EPSILON};

use super::Geometry;

pub fn cone() -> Shape {
    Cone::default().into()
}

#[derive(Clone, Copy, Debug)]
pub struct Cone {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }
}

fn check_cap(ray: Ray, time: f64, radius: f64) -> bool {
    let x = ray.origin.x + time * ray.direction.x;
    let z = ray.origin.z + time * ray.direction.z;
    x.powi(2) + z.powi(2) <= radius
}

fn intersect_caps(cone: &Cone, ray: Ray) -> Vec<f64> {
    let mut xs = vec![];
    if !cone.closed || relative_eq!(ray.direction.y, 0.0, epsilon = EPSILON) {
        return xs;
    }

    // Check for an intersection with the lower end cap by intersecting
    // the ray with the plane at y=cylinder.minimum
    let t = (cone.minimum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, cone.minimum.abs()) {
        xs.push(t);
    }

    // Check for an intersection with the upper end cap by intersecting
    // the ray with the plane at y=cylinder.maximum
    let t = (cone.maximum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, cone.maximum.abs()) {
        xs.push(t);
    }

    xs
}

impl Geometry for Cone {
    fn local_intersection<'a>(&'a self, shape: &'a Shape, ray: Ray) -> Vec<Intersection> {
        let Point {
            x: ox,
            y: oy,
            z: oz,
        } = ray.origin;
        let Vector {
            x: dx,
            y: dy,
            z: dz,
        } = ray.direction;

        let a = dx.powi(2) - dy.powi(2) + dz.powi(2);
        let b = 2.0 * ox * dx - 2.0 * oy * dy + 2.0 * oz * dz;
        let c = ox.powi(2) - oy.powi(2) + oz.powi(2);

        let a_zero = relative_eq!(a, 0.0, epsilon = EPSILON);
        let b_zero = relative_eq!(b, 0.0, epsilon = EPSILON);

        if a_zero && b_zero {
            return vec![];
        }

        let mut xs = vec![];
        if a_zero {
            let t = -c / (2.0 * b);
            xs.push(intersection(t, shape));
        } else {
            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                return vec![];
            }

            let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
            let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }

            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(intersection(t0, shape));
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(intersection(t1, shape));
            }
        }

        let caps_xs = intersect_caps(self, ray)
            .into_iter()
            .map(|t| intersection(t, shape));
        xs.extend(caps_xs);

        xs
    }

    fn local_normal_at(&self, point @ Point { x, y, z }: Point) -> crate::Vector {
        let dist = x.powi(2) + z.powi(2);
        if dist < 1.0 && y > self.maximum - EPSILON {
            vector(0, 1, 0)
        } else if dist < 1.0 && y < self.minimum + EPSILON {
            vector(0, -1, 0)
        } else {
            let y = (x.powi(2) + z.powi(2)).sqrt();
            let y = if point.y > 0.0 { -y } else { y };
            vector(x, y, z)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{point, ray, vector, EPSILON};

    use super::*;

    #[test]
    fn intersecting_cone_with_ray() {
        let tests = [
            (point(0, 0, -5), vector(0, 0, 1), 5.0, 5.0),
            (point(0, 0, -5), vector(1, 1, 1), 8.66025, 8.66025),
            (point(1, 1, -5), vector(-0.5, -1.0, 1.0), 4.55006, 49.44994),
        ];

        let shape = cone();
        for (origin, direction, t0, t1) in tests {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = shape.intersect(r);
            assert_eq!(2, xs.len());
            assert_abs_diff_eq!(t0, xs[0].time, epsilon = EPSILON);
            assert_abs_diff_eq!(t1, xs[1].time, epsilon = EPSILON);
        }
    }

    #[test]
    fn intersecting_code_with_ray_parallel_to_one_of_its_halve() {
        let shape = cone();
        let direction = vector(0, 1, 1).normalize();
        let r = ray(point(0, 0, -1), direction);
        let xs = shape.intersect(r);
        assert_eq!(1, xs.len());
        assert_abs_diff_eq!(0.35355, xs[0].time, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_cone_end_caps() {
        let tests = [
            (point(0, 0, -5), vector(0, 1, 0), 0),
            (point(0.0, 0.0, -0.25), vector(0, 1, 1), 2),
            (point(0.0, 0.0, -0.25), vector(0, 1, 0), 4),
        ];

        let shape: Shape = Cone {
            minimum: -0.5,
            maximum: 0.5,
            closed: true,
        }
        .into();
        for (origin, direction, count) in tests {
            let direction = direction.normalize();
            let r = ray(origin, direction);
            let xs = shape.intersect(r);
            assert_eq!(count, xs.len());
        }
    }

    #[test]
    fn normal_vector_on_cone() {
        let tests = [
            (point(0, 0, 0), vector(0, 0, 0)),
            (point(1, 1, 1), vector(1.0, -(2_f64.sqrt()), 1.0)),
            (point(-1, -1, 0), vector(-1, 1, 0)),
        ];

        let shape = Cone::default();
        for (point, normal) in tests {
            assert_eq!(normal, shape.local_normal_at(point));
        }
    }
}
