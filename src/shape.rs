mod plane;
mod sphere;

use crate::{Intersections, Material, Matrix4, Point, Ray, Vector};
pub use plane::*;
pub use sphere::*;

pub trait Shape {
    fn transform(&self) -> Matrix4;
    fn set_transform(&mut self, transform: Matrix4);
    fn material(&self) -> Material;
    fn set_material(&mut self, material: Material);
    fn local_intersect(&self, ray: Ray) -> Intersections;
    fn local_normal_at(&self, point: Point) -> Vector;

    fn intersect(&self, ray: Ray) -> Intersections {
        let inverse_transform = self.transform().inverse();
        let local_ray = ray.transform(inverse_transform);
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, point: Point) -> Vector {
        let inverse_transform = self.transform().inverse();
        let local_point = inverse_transform * point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = inverse_transform.transpose() * local_normal;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{material, point, ray, transformations, vector};
    use approx::assert_abs_diff_eq;
    use derive_builder::Builder;
    use std::{cell::Cell, f64::consts::PI};

    fn test_shape() -> TestShape {
        TestShapeBuilder::default().build().unwrap()
    }

    #[derive(Debug, Clone, Builder)]
    struct TestShape {
        #[builder(default = "Matrix4::identity()")]
        pub transform: Matrix4,
        #[builder(default = "material()")]
        pub material: Material,
        #[builder(setter(skip))]
        saved_ray: Cell<Option<Ray>>,
    }

    impl Shape for TestShape {
        fn transform(&self) -> Matrix4 {
            self.transform
        }

        fn material(&self) -> Material {
            self.material
        }

        fn local_intersect(&self, ray: Ray) -> Intersections {
            self.saved_ray.set(Some(ray));
            Intersections::empty()
        }

        fn local_normal_at(&self, point: Point) -> Vector {
            vector(point.x, point.y, point.z)
        }

        fn set_transform(&mut self, _transform: Matrix4) {
            todo!()
        }

        fn set_material(&mut self, _material: Material) {
            todo!()
        }
    }

    #[test]
    fn default_transformation() {
        let s = test_shape();
        assert_eq!(s.transform(), Matrix4::identity());
        assert_eq!(s.material(), material());
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = TestShapeBuilder::default()
            .transform(transformations::scaling(2.0, 2.0, 2.0))
            .build()
            .unwrap();
        s.intersect(r);
        let saved_ray = s.saved_ray.get().unwrap();
        assert_abs_diff_eq!(saved_ray.origin, point(0.0, 0.0, -2.5));
        assert_abs_diff_eq!(saved_ray.direction, vector(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = TestShapeBuilder::default()
            .transform(transformations::translation(5.0, 0.0, 0.0))
            .build()
            .unwrap();
        s.intersect(r);
        let saved_ray = s.saved_ray.get().unwrap();
        assert_abs_diff_eq!(saved_ray.origin, point(-5.0, 0.0, -5.0));
        assert_abs_diff_eq!(saved_ray.direction, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let s = TestShapeBuilder::default()
            .transform(transformations::translation(0.0, 1.0, 0.0))
            .build()
            .unwrap();
        let n = s.normal_at(point(0.0, 1.70711, -0.70711));
        assert_abs_diff_eq!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_shape() {
        let s = TestShapeBuilder::default()
            .transform(
                transformations::scaling(1.0, 0.5, 1.0) * transformations::rotation_z(PI / 5.0),
            )
            .build()
            .unwrap();
        let n = s.normal_at(point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));
        assert_abs_diff_eq!(n, vector(0.0, 0.97014, -0.24254));
    }
}
