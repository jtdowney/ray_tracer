use std::any::Any;

use bon::builder;

use crate::{
    Intersection, Material, ORIGIN, identity_matrix, material,
    matrix::Matrix4,
    point::Point,
    ray::Ray,
    shape::{Geometry, Shape},
    vector::Vector,
};

#[builder(finish_fn = build)]
#[must_use]
pub fn sphere(
    #[builder(default = identity_matrix())] transform: Matrix4,
    #[builder(default = material(), into)] material: Material,
) -> Shape {
    let shape = Shape::new(Sphere);
    shape.set_transform(transform);
    shape.set_material(material);
    shape
}

#[must_use]
pub fn glass_sphere() -> Shape {
    sphere()
        .material(Material::builder().transparency(1.0).refractive_index(1.5))
        .build()
}

pub struct Sphere;

impl Geometry for Sphere {
    fn local_intersection(&self, shape: &Shape, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - ORIGIN;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - (4.0 * a * c);

        let mut intersections = vec![];
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            intersections.push(Intersection {
                time: t1,
                object: shape.clone(),
            });
            intersections.push(Intersection {
                time: t2,
                object: shape.clone(),
            });
        }

        intersections
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        point - ORIGIN
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
    use std::f32::consts::FRAC_1_SQRT_2;

    use approx::assert_relative_eq;

    use crate::{
        EPSILON, Material, identity_matrix, material, point, ray,
        shape::{glass_sphere, sphere},
        transform, vector,
    };

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere().build();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0, 1, -5), vector(0, 0, 1));
        let s = sphere().build();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 5.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 5.0, epsilon = EPSILON);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0, 2, -5), vector(0, 0, 1));
        let s = sphere().build();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let s = sphere().build();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, -1.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let s = sphere().build();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, -6.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, -4.0, epsilon = EPSILON);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere().build();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = sphere().build();
        assert_eq!(s.inner().transform, identity_matrix());
    }

    #[test]
    fn changing_sphere_transformation() {
        let s = sphere().build();
        let t = transform::translation(2, 3, 4);
        s.set_transform(t);
        assert_eq!(s.inner().transform, t);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere().transform(transform::scaling(2, 2, 2)).build();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 3.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 7.0, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere().transform(transform::translation(5, 0, 0)).build();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = sphere().build();
        let n = s.normal_at(point(1, 0, 0));
        assert_eq!(n, vector(1, 0, 0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = sphere().build();
        let n = s.normal_at(point(0, 1, 0));
        assert_eq!(n, vector(0, 1, 0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = sphere().build();
        let n = s.normal_at(point(0, 0, 1));
        assert_eq!(n, vector(0, 0, 1));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = sphere().build();
        let sqrt3_over_3 = 3.0_f32.sqrt() / 3.0;
        let n = s.normal_at(point(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3));
        assert_relative_eq!(n.x, sqrt3_over_3, epsilon = EPSILON);
        assert_relative_eq!(n.y, sqrt3_over_3, epsilon = EPSILON);
        assert_relative_eq!(n.z, sqrt3_over_3, epsilon = EPSILON);
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = sphere().build();
        let sqrt3_over_3 = 3.0_f32.sqrt() / 3.0;
        let n = s.normal_at(point(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3));
        assert_relative_eq!(n.x, n.normalize().x, epsilon = EPSILON);
        assert_relative_eq!(n.y, n.normalize().y, epsilon = EPSILON);
        assert_relative_eq!(n.z, n.normalize().z, epsilon = EPSILON);
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let s = sphere().transform(transform::translation(0, 1, 0)).build();
        let n = s.normal_at(point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_relative_eq!(n.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(n.y, FRAC_1_SQRT_2, epsilon = EPSILON);
        assert_relative_eq!(n.z, -FRAC_1_SQRT_2, epsilon = EPSILON);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let t =
            transform::scaling(1.0, 0.5, 1.0) * transform::rotation_z(std::f32::consts::PI / 5.0);
        let s = sphere().transform(t).build();
        let sqrt2_over_2 = 2.0_f32.sqrt() / 2.0;
        let n = s.normal_at(point(0.0, sqrt2_over_2, -sqrt2_over_2));
        assert_relative_eq!(n.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(n.y, 0.97014, epsilon = EPSILON);
        assert_relative_eq!(n.z, -0.24254, epsilon = EPSILON);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = sphere().build();
        assert_eq!(s.inner().material, material());
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let m = Material::builder().ambient(1.0).build();
        let s = sphere().material(m.clone()).build();
        assert_eq!(s.inner().material, m);
    }

    #[test]
    fn glass_sphere_helper() {
        let s = glass_sphere();
        assert_eq!(s.inner().transform, identity_matrix());
        assert_relative_eq!(s.inner().material.transparency, 1.0, epsilon = EPSILON);
        assert_relative_eq!(s.inner().material.refractive_index, 1.5, epsilon = EPSILON);
    }
}
