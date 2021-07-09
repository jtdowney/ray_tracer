use crate::{intersection, material, point, Intersections, Material, Matrix4, Ray, Shape, Vector};
use derive_builder::Builder;
use point::Point;

pub fn sphere() -> Sphere {
    SphereBuilder::default().build().unwrap()
}

#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default = "material()")]
    pub material: Material,
}

impl Shape for Sphere {
    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn material(&self) -> Material {
        self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: Ray) -> Intersections {
        let sphere_to_ray = ray.origin - point::ORIGIN;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = (b * b) - 4.0 * a * c;

        if discriminant.is_sign_negative() {
            Intersections::empty()
        } else {
            let discriminant_root = discriminant.sqrt();
            let t1 = (-b - discriminant_root) / (2.0 * a);
            let t2 = (-b + discriminant_root) / (2.0 * a);

            let intersections = vec![intersection(t1, self), intersection(t2, self)];
            intersections.into()
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        point - point::ORIGIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, ray, transformations, vector, MaterialBuilder};
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let s = sphere();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let s = sphere();
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn ray_misses_sphere() {
        let s = sphere();
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = s.intersect(r).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let s = sphere();
        let r = ray(point::ORIGIN, vector(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(-1.0, xs.next().unwrap().time);
        assert_eq!(1.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn sphere_behind_ray() {
        let s = sphere();
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(-6.0, xs.next().unwrap().time);
        assert_eq!(-4.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn spheres_default_transformation() {
        let s = sphere();
        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn intersect_scaled_ray_with_sphere() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = SphereBuilder::default()
            .transform(transformations::scaling(2.0, 2.0, 2.0))
            .build()
            .unwrap();
        let mut xs = s.intersect(r).into_iter();
        assert_eq!(3.0, xs.next().unwrap().time);
        assert_eq!(7.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn intersect_translated_ray_with_sphere() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = SphereBuilder::default()
            .transform(transformations::translation(5.0, 0.0, 0.0))
            .build()
            .unwrap();
        let mut xs = s.intersect(r).into_iter();
        assert!(xs.next().is_none());
    }

    #[test]
    fn normal_on_sphere_at_x_axis() {
        let s = sphere();
        assert_eq!(s.normal_at(point(1.0, 0.0, 0.0)), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_y_axis() {
        let s = sphere();
        assert_eq!(s.normal_at(point(0.0, 1.0, 0.0)), vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_z_axis() {
        let s = sphere();
        assert_eq!(s.normal_at(point(0.0, 0.0, 1.0)), vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = sphere();
        assert_eq!(
            s.normal_at(point(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )),
            vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let s = sphere();
        let n = s.normal_at(point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = SphereBuilder::default()
            .transform(transformations::translation(0.0, 1.0, 0.0))
            .build()
            .unwrap();
        assert_abs_diff_eq!(
            s.normal_at(point(0.0, 1.70711, -0.70711)),
            vector(0.0, 0.70711, -0.70711)
        );
    }

    #[test]
    fn normal_on_transformed_shape() {
        let s = SphereBuilder::default()
            .transform(
                transformations::scaling(1.0, 0.5, 1.0) * transformations::rotation_z(PI / 5.0),
            )
            .build()
            .unwrap();
        assert_abs_diff_eq!(
            s.normal_at(point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0),),
            vector(0.0, 0.97014, -0.24254)
        );
    }

    #[test]
    fn sphere_gets_a_default_material() {
        let s = sphere();
        assert_eq!(s.material, material());
    }

    #[test]
    fn sphere_can_be_assigned_a_material() {
        let s = SphereBuilder::default()
            .material(MaterialBuilder::default().ambient(1.0).build().unwrap())
            .build()
            .unwrap();
        assert_eq!(s.material.ambient, 1.0);
    }
}
