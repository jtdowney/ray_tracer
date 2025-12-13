use std::any::Any;

use bon::builder;

use crate::{
    EPSILON, Intersection, Material, Vector, identity_matrix, material,
    matrix::Matrix4,
    point::Point,
    ray::Ray,
    shape::{Geometry, Shape},
    vector,
};

#[builder(finish_fn = build)]
#[must_use]
pub fn cube(
    #[builder(default = identity_matrix())] transform: Matrix4,
    #[builder(default = material(), into)] material: Material,
) -> Shape {
    let shape = Shape::new(Cube);
    shape.set_transform(transform);
    shape.set_material(material);
    shape
}

pub struct Cube;

impl Cube {
    fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (
                tmin_numerator * f32::INFINITY,
                tmax_numerator * f32::INFINITY,
            )
        };

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl Geometry for Cube {
    fn local_intersection(&self, shape: &Shape, ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = Self::check_axis(ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = Self::check_axis(ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = Self::check_axis(ray.origin.z(), ray.direction.z());

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![
                Intersection {
                    time: tmin,
                    object: shape.clone(),
                },
                Intersection {
                    time: tmax,
                    object: shape.clone(),
                },
            ]
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let abs_x = point.x().abs();
        let abs_y = point.y().abs();
        let abs_z = point.z().abs();

        if abs_x >= abs_y && abs_x >= abs_z {
            vector(point.x(), 0.0, 0.0)
        } else if abs_y >= abs_z {
            vector(0.0, point.y(), 0.0)
        } else {
            vector(0.0, 0.0, point.z())
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::{EPSILON, point, ray, shape::cube::cube, vector};

    #[test]
    fn ray_intersects_cube_positive_x() {
        let c = cube().build();
        let r = ray(point(5.0, 0.5, 0.0), vector(-1, 0, 0));
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_intersects_cube_negative_x() {
        let c = cube().build();
        let r = ray(point(-5.0, 0.5, 0.0), vector(1, 0, 0));
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_intersects_cube_positive_y() {
        let c = cube().build();
        let r = ray(point(0.5, 5.0, 0.0), vector(0, -1, 0));
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_intersects_cube_negative_y() {
        let c = cube().build();
        let r = ray(point(0.5, -5.0, 0.0), vector(0, 1, 0));
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_intersects_cube_positive_z() {
        let c = cube().build();
        let r = ray(point(0.5, 0.0, 5.0), vector(0, 0, -1));
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_intersects_cube_negative_z() {
        let c = cube().build();
        let r = ray(point(0.5, 0.0, -5.0), vector(0, 0, 1));
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_intersects_cube_from_inside() {
        let c = cube().build();
        let r = ray(point(0.0, 0.5, 0.0), vector(0, 0, 1));
        let xs = c.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, -1.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_misses_cube_1() {
        let c = cube().build();
        let r = ray(point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018));
        let xs = c.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_cube_2() {
        let c = cube().build();
        let r = ray(point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345));
        let xs = c.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_cube_3() {
        let c = cube().build();
        let r = ray(point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673));
        let xs = c.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_cube_4() {
        let c = cube().build();
        let r = ray(point(2.0, 0.0, 2.0), vector(0, 0, -1));
        let xs = c.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_cube_5() {
        let c = cube().build();
        let r = ray(point(0.0, 2.0, 2.0), vector(0, -1, 0));
        let xs = c.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_cube_6() {
        let c = cube().build();
        let r = ray(point(2.0, 2.0, 0.0), vector(-1, 0, 0));
        let xs = c.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn normal_on_cube_positive_x() {
        let c = cube().build();
        let n = c.normal_at(point(1.0, 0.5, -0.8));
        assert_eq!(n, vector(1, 0, 0));
    }

    #[test]
    fn normal_on_cube_negative_x() {
        let c = cube().build();
        let n = c.normal_at(point(-1.0, -0.2, 0.9));
        assert_eq!(n, vector(-1, 0, 0));
    }

    #[test]
    fn normal_on_cube_positive_y() {
        let c = cube().build();
        let n = c.normal_at(point(-0.4, 1.0, -0.1));
        assert_eq!(n, vector(0, 1, 0));
    }

    #[test]
    fn normal_on_cube_negative_y() {
        let c = cube().build();
        let n = c.normal_at(point(0.3, -1.0, -0.7));
        assert_eq!(n, vector(0, -1, 0));
    }

    #[test]
    fn normal_on_cube_positive_z() {
        let c = cube().build();
        let n = c.normal_at(point(-0.6, 0.3, 1.0));
        assert_eq!(n, vector(0, 0, 1));
    }

    #[test]
    fn normal_on_cube_negative_z() {
        let c = cube().build();
        let n = c.normal_at(point(0.4, 0.4, -1.0));
        assert_eq!(n, vector(0, 0, -1));
    }

    #[test]
    fn normal_on_cube_corner_positive() {
        let c = cube().build();
        let n = c.normal_at(point(1.0, 1.0, 1.0));
        assert_eq!(n, vector(1, 0, 0));
    }

    #[test]
    fn normal_on_cube_corner_negative() {
        let c = cube().build();
        let n = c.normal_at(point(-1.0, -1.0, -1.0));
        assert_eq!(n, vector(-1, 0, 0));
    }
}
