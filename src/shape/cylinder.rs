use crate::{
    intersection, material, vector, Intersection, Intersections, Material, Matrix4, Point, Ray,
    Shape, Vector, EPSILON,
};
use approx::abs_diff_eq;
use derive_builder::Builder;

pub fn cylinder() -> Cylinder {
    CylinderBuilder::default().build().unwrap()
}

#[derive(Builder, Clone)]
pub struct Cylinder {
    #[builder(default)]
    id: usize,
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default = "material()")]
    pub material: Material,
    #[builder(default = "-f64::INFINITY")]
    minimum: f64,
    #[builder(default = "f64::INFINITY")]
    maximum: f64,
    #[builder(default = "false")]
    closed: bool,
}

impl Shape for Cylinder {
    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: Ray) -> Intersections {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return Intersections::empty();
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

        let mut intersections = vec![];
        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            intersections.push(intersection(t0, self));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            intersections.push(intersection(t1, self));
        }

        if self.closed {
            let cap_intersections = self.intersect_caps(ray);
            intersections.extend_from_slice(&cap_intersections);
        }

        intersections.into()
    }

    fn local_normal_at(&self, Point { x, y, z }: Point) -> Vector {
        let dist = x.powi(2) + z.powi(2);
        if dist < 1.0 && y >= self.maximum - EPSILON {
            vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && y <= self.minimum + EPSILON {
            vector(0.0, -1.0, 0.0)
        } else {
            vector(x, 0.0, z)
        }
    }
}

impl Cylinder {
    fn check_cap(&self, ray: Ray, time: f64) -> bool {
        let x = ray.origin.x + time * ray.direction.x;
        let z = ray.origin.z + time * ray.direction.z;

        (x.powi(2) + z.powi(2)) <= 1.0
    }

    fn intersect_caps(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = vec![];

        if abs_diff_eq!(ray.direction.y, 0.0, epsilon = EPSILON) {
            return intersections;
        }

        let time = (self.minimum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, time) {
            intersections.push(intersection(time, self));
        }

        let time = (self.maximum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, time) {
            intersections.push(intersection(time, self));
        }

        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, ray, vector};
    use approx::assert_abs_diff_eq;

    #[test]
    fn ray_misses_cylinder() {
        let cyl = cylinder();

        let direction = vector(0.0, 1.0, 0.0).normalize();
        let r = ray(point(1.0, 0.0, 0.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());

        let direction = vector(0.0, 1.0, 0.0).normalize();
        let r = ray(point(0.0, 0.0, 0.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());

        let direction = vector(1.0, 1.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_strikes_cylinder() {
        let cyl = cylinder();

        let direction = vector(0.0, 0.0, 1.0).normalize();
        let r = ray(point(1.0, 0.0, -5.0), direction);
        let mut xs = cyl.local_intersect(r).into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let direction = vector(0.0, 0.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -5.0), direction);
        let mut xs = cyl.local_intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let direction = vector(0.1, 1.0, 1.0).normalize();
        let r = ray(point(0.5, 0.0, -5.0), direction);
        let mut xs = cyl.local_intersect(r).into_iter();
        assert_abs_diff_eq!(6.80798, xs.next().unwrap().time, epsilon = EPSILON);
        assert_abs_diff_eq!(7.08872, xs.next().unwrap().time, epsilon = EPSILON);
        assert_eq!(None, xs.next());
    }

    #[test]
    fn normal_vector_on_cylinder() {
        let cyl = cylinder();

        assert_eq!(
            cyl.local_normal_at(point(1.0, 0.0, 0.0)),
            vector(1.0, 0.0, 0.0)
        );
        assert_eq!(
            cyl.local_normal_at(point(0.0, 5.0, -1.0)),
            vector(0.0, 0.0, -1.0)
        );
        assert_eq!(
            cyl.local_normal_at(point(0.0, -2.0, 1.0)),
            vector(0.0, 0.0, 1.0)
        );
        assert_eq!(
            cyl.local_normal_at(point(-1.0, 1.0, 0.0)),
            vector(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn default_minimum_and_maximum() {
        let cyl = cylinder();
        assert_eq!(cyl.minimum, -f64::INFINITY);
        assert_eq!(cyl.maximum, f64::INFINITY);
    }

    #[test]
    fn intersecting_constrained_cylinder() {
        let cyl = CylinderBuilder::default()
            .minimum(1.0)
            .maximum(2.0)
            .build()
            .unwrap();

        let direction = vector(0.1, 1.0, 0.0).normalize();
        let r = ray(point(0.0, 1.5, -5.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());

        let direction = vector(0.0, 0.0, 1.0).normalize();
        let r = ray(point(0.0, 3.0, -5.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());

        let direction = vector(0.0, 0.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());

        let direction = vector(0.0, 0.0, 1.0).normalize();
        let r = ray(point(0.0, 2.0, -5.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());

        let direction = vector(0.0, 0.0, 1.0).normalize();
        let r = ray(point(0.0, 1.0, -5.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());

        let direction = vector(0.0, 0.0, 1.0).normalize();
        let r = ray(point(0.0, 1.5, -2.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(2, xs.count());
    }

    #[test]
    fn default_closed_value() {
        let cyl = cylinder();
        assert!(!cyl.closed);
    }

    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        let cyl = CylinderBuilder::default()
            .minimum(1.0)
            .maximum(2.0)
            .closed(true)
            .build()
            .unwrap();

        let direction = vector(0.0, -1.0, 0.0).normalize();
        let r = ray(point(0.0, 3.0, 0.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(2, xs.count());

        let direction = vector(0.0, -1.0, 2.0).normalize();
        let r = ray(point(0.0, 3.0, -2.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(2, xs.count());

        let direction = vector(0.0, -1.0, 1.0).normalize();
        let r = ray(point(0.0, 4.0, -2.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(2, xs.count());

        let direction = vector(0.0, 1.0, 2.0).normalize();
        let r = ray(point(0.0, 0.0, -2.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(2, xs.count());

        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = ray(point(0.0, -1.0, -2.0), direction);
        let xs = cyl.local_intersect(r).into_iter();
        assert_eq!(2, xs.count());
    }

    #[test]
    fn normal_vector_on_end_caps_of_closed_cylinder() {
        let cyl = CylinderBuilder::default()
            .minimum(1.0)
            .maximum(2.0)
            .closed(true)
            .build()
            .unwrap();

        assert_eq!(
            cyl.local_normal_at(point(0.0, 1.0, 0.0)),
            vector(0.0, -1.0, 0.0),
        );
        assert_eq!(
            cyl.local_normal_at(point(0.5, 1.0, 0.0)),
            vector(0.0, -1.0, 0.0),
        );
        assert_eq!(
            cyl.local_normal_at(point(0.0, 1.0, 0.5)),
            vector(0.0, -1.0, 0.0),
        );
        assert_eq!(
            cyl.local_normal_at(point(0.0, 2.0, 0.0)),
            vector(0.0, 1.0, 0.0),
        );
        assert_eq!(
            cyl.local_normal_at(point(0.5, 2.0, 0.0)),
            vector(0.0, 1.0, 0.0),
        );
        assert_eq!(
            cyl.local_normal_at(point(0.0, 2.0, 0.5)),
            vector(0.0, 1.0, 0.0),
        );
    }
}
