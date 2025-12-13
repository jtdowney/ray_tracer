use crate::{Intersection, Material, Matrix4, Point, Ray, Vector, identity_matrix, material};

mod cone;
mod cube;
mod cylinder;
mod plane;
mod sphere;

pub use cone::cone;
pub use cube::cube;
pub use cylinder::cylinder;
pub use plane::plane;
pub use sphere::{glass_sphere, sphere};

pub trait Geometry {
    fn local_intersection<'shape>(
        &self,
        shape: &'shape Shape,
        ray: Ray,
    ) -> Vec<Intersection<'shape>>;
    fn local_normal_at(&self, point: Point) -> Vector;
}

pub struct Shape {
    pub transform: Matrix4,
    pub material: Material,
    geometry: Box<dyn Geometry>,
}

impl Shape {
    /// # Panics
    /// Panics if the shape's transform matrix is not invertible.
    #[must_use]
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection<'_>> {
        let local_ray = ray.transform(self.transform.inverse().expect("invertable"));
        self.geometry.local_intersection(self, local_ray)
    }

    /// # Panics
    /// Panics if the shape's transform matrix is not invertible.
    #[must_use]
    pub fn normal_at(&self, world_point: Point) -> Vector {
        let inverse = self.transform.inverse().expect("invertible");
        let object_point = inverse * world_point;
        let object_normal = self.geometry.local_normal_at(object_point);
        let world_normal = inverse.transpose() * object_normal;
        world_normal.normalize()
    }
}

impl<G: Geometry + 'static> From<G> for Shape {
    fn from(geometry: G) -> Self {
        Shape {
            transform: identity_matrix(),
            material: material(),
            geometry: Box::new(geometry),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, f64::consts::FRAC_1_SQRT_2, rc::Rc};

    use approx::assert_relative_eq;

    use super::{Geometry, Shape};
    use crate::{
        EPSILON, Intersection, Material, Point, Ray, Vector, identity_matrix, material, point, ray,
        transform, vector,
    };

    struct TestShape {
        saved_ray: Rc<RefCell<Option<Ray>>>,
    }

    impl TestShape {
        fn new(saved_ray: Rc<RefCell<Option<Ray>>>) -> Self {
            Self { saved_ray }
        }
    }

    impl Geometry for TestShape {
        fn local_intersection<'shape>(
            &self,
            _shape: &'shape Shape,
            ray: Ray,
        ) -> Vec<Intersection<'shape>> {
            *self.saved_ray.borrow_mut() = Some(ray);
            vec![]
        }

        fn local_normal_at(&self, point: Point) -> Vector {
            vector(point.x, point.y, point.z)
        }
    }

    fn test_shape() -> (Shape, Rc<RefCell<Option<Ray>>>) {
        let saved_ray = Rc::new(RefCell::new(None));
        let shape: Shape = TestShape::new(Rc::clone(&saved_ray)).into();
        (shape, saved_ray)
    }

    #[test]
    fn default_transformation() {
        let (s, _) = test_shape();
        assert_eq!(s.transform, identity_matrix());
    }

    #[test]
    fn assigning_transformation() {
        let (mut s, _) = test_shape();
        s.transform = transform::translation(2, 3, 4);
        assert_eq!(s.transform, transform::translation(2, 3, 4));
    }

    #[test]
    fn default_material() {
        let (s, _) = test_shape();
        assert_eq!(s.material, material());
    }

    #[test]
    fn assigning_material() {
        let (mut s, _) = test_shape();
        let m = Material::builder().ambient(1.0).build();
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let (mut s, saved_ray) = test_shape();
        s.transform = transform::scaling(2, 2, 2);
        let _ = s.intersect(r);
        let saved = saved_ray.borrow();
        let saved = saved.as_ref().expect("saved ray");
        assert_relative_eq!(saved.origin.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.origin.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.origin.z, -2.5, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.z, 0.5, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let (mut s, saved_ray) = test_shape();
        s.transform = transform::translation(5, 0, 0);
        let _ = s.intersect(r);
        let saved = saved_ray.borrow();
        let saved = saved.as_ref().expect("saved ray");
        assert_relative_eq!(saved.origin.x, -5.0, epsilon = EPSILON);
        assert_relative_eq!(saved.origin.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.origin.z, -5.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.z, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let (mut s, _) = test_shape();
        s.transform = transform::translation(0, 1, 0);
        let n = s.normal_at(point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_relative_eq!(n.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(n.y, FRAC_1_SQRT_2, epsilon = EPSILON);
        assert_relative_eq!(n.z, -FRAC_1_SQRT_2, epsilon = EPSILON);
    }

    #[test]
    fn computing_normal_on_transformed_shape() {
        let (mut s, _) = test_shape();
        let m =
            transform::scaling(1.0, 0.5, 1.0) * transform::rotation_z(std::f64::consts::PI / 5.0);
        s.transform = m;
        let sqrt2_over_2 = FRAC_1_SQRT_2;
        let n = s.normal_at(point(0.0, sqrt2_over_2, -sqrt2_over_2));
        assert_relative_eq!(n.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(n.y, 0.97014, epsilon = EPSILON);
        assert_relative_eq!(n.z, -0.24254, epsilon = EPSILON);
    }
}
