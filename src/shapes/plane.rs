use crate::{
    Intersection, Intersections, Material, Matrix4, Point, Ray, Shape, Vector3, World, EPSILON,
};
use approx::relative_eq;
use derive_builder::Builder;
use indextree::NodeId;
use std::any::Any;
use std::vec;

#[derive(Builder, Clone, Debug)]
pub struct Plane {
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default)]
    pub material: Material,
    #[builder(setter(skip))]
    id: Option<NodeId>,
}

impl Default for Plane {
    fn default() -> Self {
        PlaneBuilder::default().build().unwrap()
    }
}

impl Shape for Plane {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn local_normal_at(&self, _: Point, _: &World) -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, ray: Ray, world: &World) -> Intersections {
        let id = self.id.unwrap();
        let object = &world.objects[id].data;
        if relative_eq!(ray.direction[1], 0.0, epsilon = EPSILON) {
            Intersections(vec![])
        } else {
            let time = -ray.origin.y / ray.direction[1];
            Intersections(vec![Intersection {
                time,
                object: object.clone(),
            }])
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorldBuilder;
    use std::sync::Arc;

    #[test]
    fn planes_default_transformation() {
        let s = Plane::default();
        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn normal_of_plane_is_constant() {
        let w = WorldBuilder::default().object(Plane::default()).build();
        let p = &w.objects[NodeId::new(0)].data;
        let n1 = p.local_normal_at(Point::new(0.0, 0.0, 0.0), &w);
        let n2 = p.local_normal_at(Point::new(10.0, 0.0, -10.0), &w);
        let n3 = p.local_normal_at(Point::new(-5.0, 0.0, 150.0), &w);
        assert_eq!(Vector3::new(0.0, 1.0, 0.0), n1);
        assert_eq!(Vector3::new(0.0, 1.0, 0.0), n2);
        assert_eq!(Vector3::new(0.0, 1.0, 0.0), n3);
    }

    #[test]
    fn plane_intersect_with_parallel_ray() {
        let w = WorldBuilder::default().object(Plane::default()).build();
        let p = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 10.0, 10.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = p.local_intersect(r, &w).into_iter();
        assert_eq!(None, xs.next());
    }

    #[test]
    fn plane_intersect_with_coplaner_ray() {
        let w = WorldBuilder::default().object(Plane::default()).build();
        let p = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = p.local_intersect(r, &w).into_iter();
        assert_eq!(None, xs.next());
    }

    #[test]
    fn plane_intersect_from_above() {
        let w = WorldBuilder::default().object(Plane::default()).build();
        let p = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        let mut xs = p.local_intersect(r, &w).into_iter();
        let i = xs.next().unwrap();
        assert_eq!(1.0, i.time);
        assert!(Arc::ptr_eq(&p, &i.object));
        assert_eq!(None, xs.next());
    }

    #[test]
    fn plane_intersect_from_below() {
        let w = WorldBuilder::default().object(Plane::default()).build();
        let p = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let mut xs = p.local_intersect(r, &w).into_iter();
        let i = xs.next().unwrap();
        assert_eq!(1.0, i.time);
        assert!(Arc::ptr_eq(&p, &i.object));
        assert_eq!(None, xs.next());
    }
}
