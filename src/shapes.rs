use crate::{Intersections, Material, Matrix4, Point, Ray, Vector3, World};
use indextree::NodeId;
use std::any::Any;
use std::fmt::Debug;

mod cone;
mod cube;
mod cylinder;
mod group;
mod plane;
mod sphere;

pub use self::cone::{Cone, ConeBuilder};
pub use self::cube::{Cube, CubeBuilder};
pub use self::cylinder::{Cylinder, CylinderBuilder};
pub use self::group::{Group, GroupBuilder};
pub use self::plane::{Plane, PlaneBuilder};
pub use self::sphere::{Sphere, SphereBuilder};

pub trait Shape: Any + Debug {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
    fn local_normal_at(&self, point: Point, world: &World) -> Vector3;
    fn local_intersect(&self, ray: Ray, world: &World) -> Intersections;
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix4;
    fn id(&self) -> Option<NodeId>;
    fn set_id(&mut self, id: NodeId);

    fn normal_at(&self, world_point: Point, world: &World) -> Vector3 {
        let local_point = self.world_to_object(world_point, world);
        let local_normal = self.local_normal_at(local_point, world);
        self.normal_to_world(local_normal, world)
    }

    fn intersect(&self, ray: Ray, world: &World) -> Intersections {
        let local_ray = ray.transform(self.transform().inverse());
        self.local_intersect(local_ray, world)
    }

    fn world_to_object(&self, mut point: Point, world: &World) -> Point {
        let id = self.id().expect("id must be set");
        let node = &world.objects[id];
        if let Some(parent_id) = node.parent() {
            let parent = &world.objects[parent_id].data;
            point = parent.world_to_object(point, world);
        }

        self.transform().inverse() * point
    }

    fn normal_to_world(&self, mut normal: Vector3, world: &World) -> Vector3 {
        normal = self.transform().inverse().transpose() * normal;
        normal = normal.normalize();

        let id = self.id().expect("id must be set");
        let node = &world.objects[id];
        if let Some(parent_id) = node.parent() {
            let parent = &world.objects[parent_id].data;
            normal = parent.normal_to_world(normal, world);
        }

        normal
    }
}

impl PartialEq for &Shape {
    fn eq(&self, other: &&Shape) -> bool {
        std::ptr::eq(*self, *other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transforms, WorldBuilder};
    use std::cell::RefCell;
    use std::f64::consts::PI;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct TestShape {
        transform: Matrix4,
        material: Material,
        saved_ray: Mutex<RefCell<Option<Ray>>>,
        id: Option<NodeId>,
    }

    impl Default for TestShape {
        fn default() -> Self {
            TestShape {
                transform: Matrix4::identity(),
                material: Material::default(),
                saved_ray: Mutex::new(RefCell::new(None)),
                id: None,
            }
        }
    }

    impl Shape for TestShape {
        fn as_any(&self) -> &Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut Any {
            self
        }

        fn local_normal_at(&self, Point { x, y, z }: Point, _: &World) -> Vector3 {
            Vector3::new(x, y, z)
        }

        fn local_intersect(&self, ray: Ray, _: &World) -> Intersections {
            *self.saved_ray.lock().unwrap().borrow_mut() = Some(ray);
            Intersections(vec![])
        }

        fn material(&self) -> &Material {
            &self.material
        }

        fn transform(&self) -> &Matrix4 {
            &self.transform
        }

        fn set_id(&mut self, id: NodeId) {
            self.id = Some(id)
        }

        fn id(&self) -> Option<NodeId> {
            self.id
        }
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = TestShape::default();
        s.transform = transforms::scaling(2.0, 2.0, 2.0);

        let w = WorldBuilder::default().object(s).build();
        let s = &w.objects[NodeId::new(0)].data;
        s.intersect(r, &w);

        let saved_ray = s
            .as_any()
            .downcast_ref::<TestShape>()
            .unwrap()
            .saved_ray
            .lock()
            .unwrap();
        assert_eq!(
            Point::new(0.0, 0.0, -2.5),
            saved_ray.borrow().unwrap().origin
        );
        assert_eq!(
            Vector3::new(0.0, 0.0, 0.5),
            saved_ray.borrow().unwrap().direction
        );
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut s = TestShape::default();
        s.transform = transforms::translation(5.0, 0.0, 0.0);

        let w = WorldBuilder::default().object(s).build();
        let s = &w.objects[NodeId::new(0)].data;
        s.intersect(r, &w);

        let saved_ray = s
            .as_any()
            .downcast_ref::<TestShape>()
            .unwrap()
            .saved_ray
            .lock()
            .unwrap();
        assert_eq!(
            Point::new(-5.0, 0.0, -5.0),
            saved_ray.borrow().unwrap().origin
        );
        assert_eq!(
            Vector3::new(0.0, 0.0, 1.0),
            saved_ray.borrow().unwrap().direction
        );
    }

    #[test]
    fn normal_on_translated_shape() {
        let mut s = TestShape::default();
        s.transform = transforms::translation(0.0, 1.0, 0.0);

        let w = WorldBuilder::default().object(s).build();
        let s = &w.objects[NodeId::new(0)].data;
        assert_eq!(
            Vector3::new(0.0, 0.70711, -0.70711),
            s.normal_at(Point::new(0.0, 1.70711, -0.70711), &w)
        );
    }

    #[test]
    fn normal_on_transformed_shape() {
        let mut s = TestShape::default();
        s.transform = transforms::scaling(1.0, 0.5, 1.0) * transforms::rotation_z(PI / 5.0);

        let w = WorldBuilder::default().object(s).build();
        let s = &w.objects[NodeId::new(0)].data;
        assert_eq!(
            Vector3::new(0.0, 0.97014, -0.24254),
            s.normal_at(
                Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0),
                &w
            )
        );
    }

    #[test]
    fn converting_point_from_world_to_object_space() {
        let w = WorldBuilder::default()
            .start_group(
                GroupBuilder::default()
                    .transform(transforms::rotation_y(PI / 2.0))
                    .build()
                    .unwrap(),
            )
            .start_group(
                GroupBuilder::default()
                    .transform(transforms::scaling(2.0, 2.0, 2.0))
                    .build()
                    .unwrap(),
            )
            .object(
                SphereBuilder::default()
                    .transform(transforms::translation(5.0, 0.0, 0.0))
                    .build()
                    .unwrap(),
            )
            .end_group()
            .end_group()
            .build();
        let s = &w.objects[NodeId::new(2)].data;
        assert_eq!(
            Point::new(0.0, 0.0, -1.0),
            s.world_to_object(Point::new(-2.0, 0.0, -10.0), &w)
        );
    }

    #[test]
    fn converting_normal_from_object_to_world_space() {
        let w = WorldBuilder::default()
            .start_group(
                GroupBuilder::default()
                    .transform(transforms::rotation_y(PI / 2.0))
                    .build()
                    .unwrap(),
            )
            .start_group(
                GroupBuilder::default()
                    .transform(transforms::scaling(1.0, 2.0, 3.0))
                    .build()
                    .unwrap(),
            )
            .object(
                SphereBuilder::default()
                    .transform(transforms::translation(5.0, 0.0, 0.0))
                    .build()
                    .unwrap(),
            )
            .end_group()
            .end_group()
            .build();
        let s = &w.objects[NodeId::new(2)].data;
        assert_eq!(
            Vector3::new(0.2857, 0.4286, -0.8571),
            s.normal_to_world(
                Vector3::new(
                    f64::sqrt(3.0) / 3.0,
                    f64::sqrt(3.0) / 3.0,
                    f64::sqrt(3.0) / 3.0
                ),
                &w
            )
        );
    }

    #[test]
    fn finding_normal_on_child_object() {
        let w = WorldBuilder::default()
            .start_group(
                GroupBuilder::default()
                    .transform(transforms::rotation_y(PI / 2.0))
                    .build()
                    .unwrap(),
            )
            .start_group(
                GroupBuilder::default()
                    .transform(transforms::scaling(1.0, 2.0, 3.0))
                    .build()
                    .unwrap(),
            )
            .object(
                SphereBuilder::default()
                    .transform(transforms::translation(5.0, 0.0, 0.0))
                    .build()
                    .unwrap(),
            )
            .end_group()
            .end_group()
            .build();
        let s = &w.objects[NodeId::new(2)].data;
        assert_eq!(
            Vector3::new(0.2857, 0.4286, -0.8571),
            s.normal_at(Point::new(1.7321, 1.1547, -5.5774), &w)
        );
    }
}
