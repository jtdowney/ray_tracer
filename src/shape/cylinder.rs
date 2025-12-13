use bon::builder;

use crate::{
    EPSILON, Intersection, Material, Vector, identity_matrix, intersection, material,
    matrix::Matrix4,
    point::Point,
    ray::Ray,
    shape::{Geometry, Shape},
    vector,
};

#[builder(finish_fn = build, derive(Into))]
#[must_use]
pub fn cylinder(
    #[builder(default = identity_matrix())] transform: Matrix4,
    #[builder(default = material(), into)] material: Material,
    #[builder(default = f64::NEG_INFINITY)] minimum: f64,
    #[builder(default = f64::INFINITY)] maximum: f64,
    #[builder(default = false)] closed: bool,
) -> Shape {
    let mut shape: Shape = Cylinder {
        minimum,
        maximum,
        closed,
    }
    .into();
    shape.transform = transform;
    shape.material = material;
    shape
}

pub struct Cylinder {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Cylinder {
    fn check_cap(ray: Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        (x.powi(2) + z.powi(2)) <= 1.0
    }

    fn intersect_caps<'shape>(
        &self,
        shape: &'shape Shape,
        ray: Ray,
        xs: &mut Vec<Intersection<'shape>>,
    ) {
        if !self.closed || ray.direction.y.abs() < EPSILON {
            return;
        }

        let t = (self.minimum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(ray, t) {
            xs.push(intersection(t, shape));
        }

        let t = (self.maximum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(ray, t) {
            xs.push(intersection(t, shape));
        }
    }
}

impl Geometry for Cylinder {
    fn local_intersection<'shape>(
        &self,
        shape: &'shape Shape,
        ray: Ray,
    ) -> Vec<Intersection<'shape>> {
        let mut xs = vec![];

        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        if a.abs() < EPSILON {
            self.intersect_caps(shape, ray, &mut xs);
            return xs;
        }

        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return xs;
        }

        let mut t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let mut t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(intersection(t0, shape));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(intersection(t1, shape));
        }

        self.intersect_caps(shape, ray, &mut xs);

        xs
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let dist = point.x.powi(2) + point.z.powi(2);

        if dist < 1.0 && point.y >= self.maximum - EPSILON {
            vector(0, 1, 0)
        } else if dist < 1.0 && point.y <= self.minimum + EPSILON {
            vector(0, -1, 0)
        } else {
            vector(point.x, 0, point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::Cylinder;
    use crate::{EPSILON, point, ray, shape::cylinder::cylinder, vector};

    // Test #1: A ray misses a cylinder
    #[test]
    fn ray_misses_cylinder_on_surface_parallel_to_y() {
        let cyl = cylinder().build();
        let direction = vector(0, 1, 0).normalize();
        let r = ray(point(1, 0, 0), direction);
        let xs = cyl.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_cylinder_inside_parallel_to_y() {
        let cyl = cylinder().build();
        let direction = vector(0, 1, 0).normalize();
        let r = ray(point(0, 0, 0), direction);
        let xs = cyl.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_cylinder_outside_skewed() {
        let cyl = cylinder().build();
        let direction = vector(1, 1, 1).normalize();
        let r = ray(point(0, 0, -5), direction);
        let xs = cyl.intersect(r);
        assert!(xs.is_empty());
    }

    // Test #2: A ray strikes a cylinder
    #[test]
    fn ray_strikes_cylinder_tangent() {
        let cyl = cylinder().build();
        let direction = vector(0, 0, 1).normalize();
        let r = ray(point(1, 0, -5), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 5.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 5.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_strikes_cylinder_through_middle() {
        let cyl = cylinder().build();
        let direction = vector(0, 0, 1).normalize();
        let r = ray(point(0, 0, -5), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_strikes_cylinder_at_angle() {
        let cyl = cylinder().build();
        let direction = vector(0.1, 1.0, 1.0).normalize();
        let r = ray(point(0.5, 0.0, -5.0), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 6.80798, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 7.08872, epsilon = EPSILON);
    }

    // Test #3: Normal vector on a cylinder
    #[test]
    fn normal_on_cylinder_positive_x() {
        let cyl = cylinder().build();
        let n = cyl.normal_at(point(1, 0, 0));
        assert_eq!(n, vector(1, 0, 0));
    }

    #[test]
    fn normal_on_cylinder_negative_z() {
        let cyl = cylinder().build();
        let n = cyl.normal_at(point(0, 5, -1));
        assert_eq!(n, vector(0, 0, -1));
    }

    #[test]
    fn normal_on_cylinder_positive_z() {
        let cyl = cylinder().build();
        let n = cyl.normal_at(point(0, -2, 1));
        assert_eq!(n, vector(0, 0, 1));
    }

    #[test]
    fn normal_on_cylinder_negative_x() {
        let cyl = cylinder().build();
        let n = cyl.normal_at(point(-1, 1, 0));
        assert_eq!(n, vector(-1, 0, 0));
    }

    // Test #4: The default minimum and maximum for a cylinder
    #[test]
    fn default_minimum_and_maximum() {
        let cyl = Cylinder {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        };
        assert!(cyl.minimum.is_infinite() && cyl.minimum.is_sign_negative());
        assert!(cyl.maximum.is_infinite() && cyl.maximum.is_sign_positive());
    }

    // Test #5: Intersecting a constrained cylinder
    #[test]
    fn intersecting_constrained_cylinder_diagonal_escape() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).build();
        let direction = vector(0.1, 1.0, 0.0).normalize();
        let r = ray(point(0.0, 1.5, 0.0), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_constrained_cylinder_above() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).build();
        let direction = vector(0, 0, 1).normalize();
        let r = ray(point(0, 3, -5), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_constrained_cylinder_below() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).build();
        let direction = vector(0, 0, 1).normalize();
        let r = ray(point(0, 0, -5), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_constrained_cylinder_at_maximum() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).build();
        let direction = vector(0, 0, 1).normalize();
        let r = ray(point(0, 2, -5), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_constrained_cylinder_at_minimum() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).build();
        let direction = vector(0, 0, 1).normalize();
        let r = ray(point(0, 1, -5), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_constrained_cylinder_through_middle() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).build();
        let direction = vector(0, 0, 1).normalize();
        let r = ray(point(0.0, 1.5, -2.0), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    // Test #6: The default closed value for a cylinder
    #[test]
    fn default_closed_value() {
        let cyl = Cylinder {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        };
        assert!(!cyl.closed);
    }

    // Test #7: Intersecting the caps of a closed cylinder
    #[test]
    fn intersecting_closed_cylinder_caps_from_above() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let direction = vector(0, -1, 0).normalize();
        let r = ray(point(0, 3, 0), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_closed_cylinder_caps_diagonal_from_above() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let direction = vector(0, -1, 2).normalize();
        let r = ray(point(0, 3, -2), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_closed_cylinder_caps_corner_above() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let direction = vector(0, -1, 1).normalize();
        let r = ray(point(0, 4, -2), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_closed_cylinder_caps_diagonal_from_below() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let direction = vector(0, 1, 2).normalize();
        let r = ray(point(0, 0, -2), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_closed_cylinder_caps_corner_below() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let direction = vector(0, 1, 1).normalize();
        let r = ray(point(0, -1, -2), direction);
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    // Test #8: The normal vector on a cylinder's end caps
    #[test]
    fn normal_on_cylinder_end_cap_lower_center() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let n = cyl.normal_at(point(0, 1, 0));
        assert_eq!(n, vector(0, -1, 0));
    }

    #[test]
    fn normal_on_cylinder_end_cap_lower_x_offset() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let n = cyl.normal_at(point(0.5, 1.0, 0.0));
        assert_eq!(n, vector(0, -1, 0));
    }

    #[test]
    fn normal_on_cylinder_end_cap_lower_z_offset() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let n = cyl.normal_at(point(0.0, 1.0, 0.5));
        assert_eq!(n, vector(0, -1, 0));
    }

    #[test]
    fn normal_on_cylinder_end_cap_upper_center() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let n = cyl.normal_at(point(0, 2, 0));
        assert_eq!(n, vector(0, 1, 0));
    }

    #[test]
    fn normal_on_cylinder_end_cap_upper_x_offset() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let n = cyl.normal_at(point(0.5, 2.0, 0.0));
        assert_eq!(n, vector(0, 1, 0));
    }

    #[test]
    fn normal_on_cylinder_end_cap_upper_z_offset() {
        let cyl = cylinder().minimum(1.0).maximum(2.0).closed(true).build();
        let n = cyl.normal_at(point(0.0, 2.0, 0.5));
        assert_eq!(n, vector(0, 1, 0));
    }
}
