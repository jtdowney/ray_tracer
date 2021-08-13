use crate::{
    intersection, material, vector, Intersection, Intersections, Material, Matrix4, Point, Ray,
    Shape, Vector, EPSILON,
};
use approx::abs_diff_eq;
use derive_builder::Builder;

pub fn cone() -> Cone {
    ConeBuilder::default().build().unwrap()
}

#[derive(Builder, Clone)]
pub struct Cone {
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

impl Shape for Cone {
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
        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * ray.origin.x * ray.direction.x - 2.0 * ray.origin.y * ray.direction.y
            + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        let mut intersections = vec![];
        let a_zero = abs_diff_eq!(a, 0.0, epsilon = EPSILON);
        let b_zero = abs_diff_eq!(b, 0.0, epsilon = EPSILON);
        if a_zero && b_zero {
            return Intersections::empty();
        }

        if a_zero {
            let time = -c / (2.0 * b);
            intersections.push(intersection(time, self));
        } else {
            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                return intersections.into();
            }

            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);
            let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                intersections.push(intersection(t0, self));
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                intersections.push(intersection(t1, self));
            }
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
            let ny = dist.sqrt();
            let ny = if y > 0.0 { -ny } else { ny };
            vector(x, ny, z)
        }
    }
}

impl Cone {
    fn check_cap(&self, ray: Ray, time: f64, radius: f64) -> bool {
        let x = ray.origin.x + time * ray.direction.x;
        let z = ray.origin.z + time * ray.direction.z;

        (x.powi(2) + z.powi(2)) <= radius
    }

    fn intersect_caps(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = vec![];

        if !self.closed || abs_diff_eq!(ray.direction.y, 0.0, epsilon = EPSILON) {
            return intersections;
        }

        let time = (self.minimum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, time, self.minimum.abs()) {
            intersections.push(intersection(time, self));
        }

        let time = (self.maximum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, time, self.maximum.abs()) {
            intersections.push(intersection(time, self));
        }

        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, ray};
    use approx::assert_abs_diff_eq;

    #[test]
    fn ray_strikes_cone() {
        let cone = cone();

        let direction = vector(0.0, 0.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -5.0), direction);
        let mut xs = cone.local_intersect(r).into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let direction = vector(1.0, 1.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -5.0), direction);
        let mut xs = cone.local_intersect(r).into_iter();
        assert_abs_diff_eq!(8.66025, xs.next().unwrap().time, epsilon = EPSILON);
        assert_abs_diff_eq!(8.66025, xs.next().unwrap().time, epsilon = EPSILON);
        assert_eq!(None, xs.next());

        let direction = vector(-0.5, -1.0, 1.0).normalize();
        let r = ray(point(1.0, 1.0, -5.0), direction);
        let mut xs = cone.local_intersect(r).into_iter();
        assert_abs_diff_eq!(4.55006, xs.next().unwrap().time, epsilon = EPSILON);
        assert_abs_diff_eq!(49.44994, xs.next().unwrap().time, epsilon = EPSILON);
        assert_eq!(None, xs.next());
    }

    #[test]
    fn intersecting_a_cone_parallel_to_one_of_its_halves() {
        let cone = cone();

        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -1.0), direction);
        let mut xs = cone.local_intersect(r).into_iter();
        assert_abs_diff_eq!(0.35355, xs.next().unwrap().time, epsilon = EPSILON);
        assert_eq!(None, xs.next());
    }

    #[test]
    fn intersecting_codes_end_caps() {
        let cone = ConeBuilder::default()
            .minimum(-0.5)
            .maximum(0.5)
            .closed(true)
            .build()
            .unwrap();

        let direction = vector(0.0, 1.0, 0.0).normalize();
        let r = ray(point(0.0, 0.0, -5.0), direction);
        let xs = cone.local_intersect(r).into_iter();
        assert_eq!(0, xs.count());

        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -0.25), direction);
        let xs = cone.local_intersect(r).into_iter();
        assert_eq!(2, xs.count());

        let direction = vector(0.0, 1.0, 0.0).normalize();
        let r = ray(point(0.0, 0.0, -0.25), direction);
        let xs = cone.local_intersect(r).into_iter();
        assert_eq!(4, xs.count());
    }

    #[test]
    fn computing_normal_vector_on_cone() {
        let cone = cone();

        assert_eq!(
            vector(0.0, 0.0, 0.0),
            cone.local_normal_at(point(0.0, 0.0, 0.0))
        );
        assert_eq!(
            vector(1.0, -f64::sqrt(2.0), 1.0),
            cone.local_normal_at(point(1.0, 1.0, 1.0))
        );
        assert_eq!(
            vector(-1.0, 1.0, 0.0),
            cone.local_normal_at(point(-1.0, -1.0, 0.0))
        );
    }
}
