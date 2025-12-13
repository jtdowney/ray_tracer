use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    fmt,
    rc::{Rc, Weak},
};

use crate::{Intersection, Material, Matrix4, Point, Ray, Vector, identity_matrix, material};

mod cone;
mod cube;
mod cylinder;
mod group;
mod plane;
mod smooth_triangle;
mod sphere;
mod triangle;

pub use cone::cone;
pub use cube::cube;
pub use cylinder::cylinder;
pub use group::{Group, group};
pub use plane::plane;
pub use smooth_triangle::{SmoothTriangle, smooth_triangle};
pub use sphere::{glass_sphere, sphere};
pub use triangle::{Triangle, triangle};

pub type ShapeRef = Rc<RefCell<ShapeInner>>;
pub type WeakShapeRef = Weak<RefCell<ShapeInner>>;

pub trait Geometry {
    fn local_intersection(&self, shape: &Shape, ray: Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, point: Point, hit: Option<&Intersection>) -> Vector;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ShapeInner {
    pub transform: Matrix4,
    pub inverse_transform: Matrix4,
    pub material: Material,
    pub parent: Option<WeakShapeRef>,
    pub(crate) geometry: Box<dyn Geometry>,
}

#[derive(Clone)]
pub struct Shape {
    inner_ref: ShapeRef,
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner();
        f.debug_struct("Shape")
            .field("transform", &inner.transform)
            .field("material", &inner.material)
            .finish_non_exhaustive()
    }
}

impl Shape {
    pub fn new<G: Geometry + 'static>(geometry: G) -> Self {
        Shape {
            inner_ref: Rc::new(RefCell::new(ShapeInner {
                transform: identity_matrix(),
                inverse_transform: identity_matrix(),
                material: material(),
                parent: None,
                geometry: Box::new(geometry),
            })),
        }
    }

    pub(crate) fn inner(&self) -> Ref<'_, ShapeInner> {
        self.inner_ref.borrow()
    }

    pub(crate) fn inner_mut(&self) -> RefMut<'_, ShapeInner> {
        self.inner_ref.borrow_mut()
    }

    /// Returns this shape's transformation matrix.
    #[must_use]
    pub fn transform(&self) -> Matrix4 {
        self.inner_ref.borrow().transform
    }

    /// Returns a clone of this shape's material.
    #[must_use]
    pub fn material(&self) -> Material {
        self.inner_ref.borrow().material.clone()
    }

    #[must_use]
    pub fn downgrade(&self) -> WeakShapeRef {
        Rc::downgrade(&self.inner_ref)
    }

    /// Sets the transformation matrix for this shape.
    ///
    /// # Panics
    /// Panics if the matrix is not invertible.
    pub fn set_transform(&self, transform: Matrix4) {
        let mut inner = self.inner_ref.borrow_mut();
        inner.transform = transform;
        inner.inverse_transform = transform.inverse().expect("invertible");
    }

    /// Sets the material for this shape.
    /// If this shape is a Group, the material is recursively applied to all children.
    pub fn set_material(&self, material: Material) {
        if let Some(group) = self
            .inner_ref
            .borrow()
            .geometry
            .as_any()
            .downcast_ref::<Group>()
        {
            for child in group.children() {
                child.set_material(material.clone());
            }
        }

        self.inner_ref.borrow_mut().material = material;
    }

    /// Sets the parent reference for this shape.
    pub fn set_parent(&self, parent: WeakShapeRef) {
        self.inner_ref.borrow_mut().parent = Some(parent);
    }

    /// Returns the parent shape if it exists and is still alive.
    #[must_use]
    pub fn parent(&self) -> Option<Shape> {
        self.inner_ref
            .borrow()
            .parent
            .as_ref()
            .and_then(|weak| weak.upgrade().map(|inner_ref| Shape { inner_ref }))
    }

    /// Converts a point from world space to object space, recursively
    /// taking into consideration any parent objects between the two spaces.
    #[must_use]
    pub fn world_to_object(&self, point: Point) -> Point {
        let point = if let Some(parent) = self.parent() {
            parent.world_to_object(point)
        } else {
            point
        };

        let inner = self.inner_ref.borrow();
        inner.inverse_transform * point
    }

    /// Converts a normal vector from object space to world space, recursively
    /// taking into consideration any parent objects between the two spaces.
    #[must_use]
    pub fn normal_to_world(&self, normal: Vector) -> Vector {
        let inner = self.inner_ref.borrow();
        let normal = inner.inverse_transform.transpose() * normal;
        let normal = normal.normalize();
        drop(inner);

        if let Some(parent) = self.parent() {
            parent.normal_to_world(normal)
        } else {
            normal
        }
    }

    /// Computes the intersections between a ray and this shape.
    #[must_use]
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let inner = self.inner_ref.borrow();
        let local_ray = ray.transform(inner.inverse_transform);
        inner.geometry.local_intersection(self, local_ray)
    }

    /// Computes the normal vector at a point on this shape's surface.
    #[must_use]
    pub fn normal_at(&self, world_point: Point) -> Vector {
        self.normal_at_with_hit(world_point, None)
    }

    /// Computes the normal vector at a point, with optional intersection data for smooth triangles.
    #[must_use]
    pub fn normal_at_with_hit(&self, world_point: Point, hit: Option<&Intersection>) -> Vector {
        let local_point = self.world_to_object(world_point);
        let inner = self.inner_ref.borrow();
        let local_normal = inner.geometry.local_normal_at(local_point, hit);
        drop(inner);
        self.normal_to_world(local_normal)
    }

    /// Adds a child shape to this group. Sets the child's parent to this shape.
    ///
    /// # Panics
    /// Panics if this shape is not a Group.
    pub fn add_child(&self, child: Shape) {
        let mut inner = self.inner_mut();
        let group = inner
            .geometry
            .as_any_mut()
            .downcast_mut::<Group>()
            .expect("Shape must be a Group");

        child.set_parent(self.downgrade());
        group.children.push(child);
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner_ref, &other.inner_ref)
    }
}

impl<G: Geometry + 'static> From<G> for Shape {
    fn from(geometry: G) -> Self {
        Shape::new(geometry)
    }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, cell::RefCell, f32::consts::FRAC_1_SQRT_2, rc::Rc};

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
        fn local_intersection(&self, _shape: &Shape, ray: Ray) -> Vec<Intersection> {
            *self.saved_ray.borrow_mut() = Some(ray);
            vec![]
        }

        fn local_normal_at(&self, point: Point, _hit: Option<&Intersection>) -> Vector {
            vector(point.x(), point.y(), point.z())
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
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
        assert_eq!(s.inner().transform, identity_matrix());
    }

    #[test]
    fn assigning_transformation() {
        let (s, _) = test_shape();
        s.set_transform(transform::translation(2, 3, 4));
        assert_eq!(s.inner().transform, transform::translation(2, 3, 4));
    }

    #[test]
    fn default_material() {
        let (s, _) = test_shape();
        assert_eq!(s.inner().material, material());
    }

    #[test]
    fn assigning_material() {
        let (s, _) = test_shape();
        let m = Material::builder().ambient(1.0).build();
        s.set_material(m.clone());
        assert_eq!(s.inner().material, m);
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let (s, saved_ray) = test_shape();
        s.set_transform(transform::scaling(2, 2, 2));
        let _ = s.intersect(r);
        let saved = saved_ray.borrow();
        let saved = saved.as_ref().expect("saved ray");
        assert_relative_eq!(saved.origin.x(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.origin.y(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.origin.z(), -2.5, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.x(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.y(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.z(), 0.5, epsilon = EPSILON);
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let (s, saved_ray) = test_shape();
        s.set_transform(transform::translation(5, 0, 0));
        let _ = s.intersect(r);
        let saved = saved_ray.borrow();
        let saved = saved.as_ref().expect("saved ray");
        assert_relative_eq!(saved.origin.x(), -5.0, epsilon = EPSILON);
        assert_relative_eq!(saved.origin.y(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.origin.z(), -5.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.x(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.y(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(saved.direction.z(), 1.0, epsilon = EPSILON);
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let (s, _) = test_shape();
        s.set_transform(transform::translation(0, 1, 0));
        let n = s.normal_at(point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_relative_eq!(n.x(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(n.y(), FRAC_1_SQRT_2, epsilon = EPSILON);
        assert_relative_eq!(n.z(), -FRAC_1_SQRT_2, epsilon = EPSILON);
    }

    #[test]
    fn computing_normal_on_transformed_shape() {
        let (s, _) = test_shape();
        let m =
            transform::scaling(1.0, 0.5, 1.0) * transform::rotation_z(std::f32::consts::PI / 5.0);
        s.set_transform(m);
        let sqrt2_over_2 = FRAC_1_SQRT_2;
        let n = s.normal_at(point(0.0, sqrt2_over_2, -sqrt2_over_2));
        assert_relative_eq!(n.x(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(n.y(), 0.97014, epsilon = EPSILON);
        assert_relative_eq!(n.z(), -0.24254, epsilon = EPSILON);
    }

    #[test]
    fn shape_has_parent_attribute() {
        let (s, _) = test_shape();
        assert!(s.parent().is_none());
    }
}
