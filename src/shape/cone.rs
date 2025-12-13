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
pub fn cone(
    #[builder(default = identity_matrix())] transform: Matrix4,
    #[builder(default = material(), into)] material: Material,
    #[builder(default = f32::NEG_INFINITY)] minimum: f32,
    #[builder(default = f32::INFINITY)] maximum: f32,
    #[builder(default = false)] closed: bool,
) -> Shape {
    let shape = Shape::new(Cone {
        minimum,
        maximum,
        closed,
    });
    shape.set_transform(transform);
    shape.set_material(material);
    shape
}

pub struct Cone {
    pub minimum: f32,
    pub maximum: f32,
    pub closed: bool,
}

impl Cone {
    fn check_cap(ray: Ray, t: f32, y: f32) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();
        let y_abs = y.abs();
        (x * x + z * z) <= y_abs * y_abs
    }

    fn intersect_caps(&self, shape: &Shape, ray: Ray, xs: &mut Vec<Intersection>) {
        if !self.closed || ray.direction.y().abs() < EPSILON {
            return;
        }

        let t = (self.minimum - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(ray, t, self.minimum) {
            xs.push(Intersection {
                time: t,
                object: shape.clone(),
                u: None,
                v: None,
            });
        }

        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(ray, t, self.maximum) {
            xs.push(Intersection {
                time: t,
                object: shape.clone(),
                u: None,
                v: None,
            });
        }
    }
}

impl Geometry for Cone {
    #[allow(clippy::many_single_char_names)]
    fn local_intersection(&self, shape: &Shape, ray: Ray) -> Vec<Intersection> {
        let mut xs = vec![];

        let (dx, dy, dz) = (ray.direction.x(), ray.direction.y(), ray.direction.z());
        let (ox, oy, oz) = (ray.origin.x(), ray.origin.y(), ray.origin.z());

        let a = dx * dx - dy * dy + dz * dz;
        let b = 2.0 * ox * dx - 2.0 * oy * dy + 2.0 * oz * dz;
        let c = ox * ox - oy * oy + oz * oz;

        if a.abs() < EPSILON {
            if b.abs() >= EPSILON {
                let t = -c / (2.0 * b);
                let y = oy + t * dy;
                if self.minimum < y && y < self.maximum {
                    xs.push(Intersection {
                        time: t,
                        object: shape.clone(),
                        u: None,
                        v: None,
                    });
                }
            }
            self.intersect_caps(shape, ray, &mut xs);
            return xs;
        }

        let discriminant = match b * b - 4.0 * a * c {
            d if d < -EPSILON => return xs,
            d => d.max(0.0),
        };

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

        let y0 = ray.origin.y() + t0 * ray.direction.y();
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection {
                time: t0,
                object: shape.clone(),
                u: None,
                v: None,
            });
        }

        let y1 = ray.origin.y() + t1 * ray.direction.y();
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection {
                time: t1,
                object: shape.clone(),
                u: None,
                v: None,
            });
        }

        self.intersect_caps(shape, ray, &mut xs);

        xs
    }

    fn local_normal_at(&self, point: Point, _hit: Option<&Intersection>) -> Vector {
        let dist = point.x() * point.x() + point.z() * point.z();
        let max_abs = self.maximum.abs();
        let min_abs = self.minimum.abs();

        if dist < max_abs * max_abs && point.y() >= self.maximum - EPSILON {
            vector(0, 1, 0)
        } else if dist < min_abs * min_abs && point.y() <= self.minimum + EPSILON {
            vector(0, -1, 0)
        } else {
            let mut y = dist.sqrt();
            if point.y() > 0.0 {
                y = -y;
            }
            vector(point.x(), y, point.z())
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

    use super::Cone;
    use crate::{
        EPSILON, point, ray,
        shape::{Geometry, cone::cone},
        vector,
    };

    #[test]
    fn intersecting_cone_along_z_axis() {
        let shape = cone().build();
        let direction = vector(0, 0, 1).normalize();
        let r = ray(point(0, 0, -5), direction);
        let xs = shape.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 5.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 5.0, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_cone_at_angle() {
        let shape = cone().build();
        let direction = vector(1, 1, 1).normalize();
        let r = ray(point(0, 0, -5), direction);
        let xs = shape.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 8.66025, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 8.66025, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_cone_skewed() {
        let shape = cone().build();
        let direction = vector(-0.5, -1.0, 1.0).normalize();
        let r = ray(point(1.0, 1.0, -5.0), direction);
        let xs = shape.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.55006, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 49.44994, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_cone_parallel_to_half() {
        let shape = cone().build();
        let direction = vector(0, 1, 1).normalize();
        let r = ray(point(0, 0, -1), direction);
        let xs = shape.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_relative_eq!(xs[0].time, 0.35355, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_cone_end_caps_miss() {
        let shape = cone().minimum(-0.5).maximum(0.5).closed(true).build();
        let direction = vector(0, 1, 0).normalize();
        let r = ray(point(0, 0, -5), direction);
        let xs = shape.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_cone_end_caps_diagonal() {
        let shape = cone().minimum(-0.5).maximum(0.5).closed(true).build();
        let direction = vector(0, 1, 1).normalize();
        let r = ray(point(0.0, 0.0, -0.25), direction);
        let xs = shape.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_cone_end_caps_through_both() {
        let shape = cone().minimum(-0.5).maximum(0.5).closed(true).build();
        let direction = vector(0, 1, 0).normalize();
        let r = ray(point(0.0, 0.0, -0.25), direction);
        let xs = shape.intersect(r);
        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn normal_on_cone_at_origin() {
        let cone_geom = Cone {
            minimum: f32::NEG_INFINITY,
            maximum: f32::INFINITY,
            closed: false,
        };
        let n = cone_geom.local_normal_at(point(0, 0, 0), None);
        assert_eq!(n, vector(0, 0, 0));
    }

    #[test]
    fn normal_on_cone_positive_y() {
        let shape = cone().build();
        let n = shape.normal_at(point(1, 1, 1));
        let sqrt2 = 2.0_f32.sqrt();
        assert_relative_eq!(n.x(), 0.5, epsilon = EPSILON);
        assert_relative_eq!(n.y(), -sqrt2 / 2.0, epsilon = EPSILON);
        assert_relative_eq!(n.z(), 0.5, epsilon = EPSILON);
    }

    #[test]
    fn normal_on_cone_negative_y() {
        let shape = cone().build();
        let n = shape.normal_at(point(-1, -1, 0));
        let sqrt2 = 2.0_f32.sqrt();
        assert_relative_eq!(n.x(), -1.0 / sqrt2, epsilon = EPSILON);
        assert_relative_eq!(n.y(), 1.0 / sqrt2, epsilon = EPSILON);
        assert_relative_eq!(n.z(), 0.0, epsilon = EPSILON);
    }
}
