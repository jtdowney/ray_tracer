use crate::{
    Bounds, Intersection, Intersections, Material, Matrix4, Point, Ray, Shape, Vector3, World,
};
use derive_builder::Builder;
use indextree::NodeId;
use std::any::Any;

#[derive(Builder, Clone, Debug)]
pub struct Group {
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(setter(skip))]
    id: Option<NodeId>,
}

impl Default for Group {
    fn default() -> Self {
        GroupBuilder::default().build().unwrap()
    }
}

impl Shape for Group {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn bounds(&self, world: &World) -> Bounds {
        let id = self.id.unwrap();
        id.children(&world.objects)
            .map(|i| {
                let object = &world.objects[i].data;
                let object_bounds = object.bounds(&world);
                let object_transform = *object.transform();

                object_bounds * object_transform
            })
            .sum()
    }

    fn local_normal_at(&self, _: Point) -> Vector3 {
        unimplemented!()
    }

    fn local_intersect(&self, ray: Ray, world: &World) -> Intersections {
        if !self.bounds(world).intersect(ray) {
            return Intersections(vec![]);
        }

        let id = self.id.unwrap();
        let mut intersections = id
            .children(&world.objects)
            .flat_map(|i| world.objects[i].data.intersect(ray, world))
            .collect::<Vec<Intersection>>();
        intersections.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        Intersections(intersections)
    }

    fn material(&self) -> &Material {
        unimplemented!()
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
    use crate::{transforms, Group, Sphere, SphereBuilder, WorldBuilder};
    use std::sync::Arc;

    #[test]
    fn creating_a_group() {
        let g = Group::default();
        assert_eq!(Matrix4::identity(), g.transform);
    }

    #[test]
    fn intersecting_ray_with_empty_group() {
        let w = WorldBuilder::default()
            .start_group(Group::default())
            .end_group()
            .build();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let g = &w.objects[NodeId::new(0)].data;
        let xs = g.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn intersecting_ray_with_nonempty_group() {
        let w = WorldBuilder::default()
            .start_group(Group::default())
            .object(Sphere::default())
            .object(
                SphereBuilder::default()
                    .transform(transforms::translation(0.0, 0.0, -3.0))
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
            .build();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let g = &w.objects[NodeId::new(0)].data;
        let s1 = &w.objects[NodeId::new(1)].data;
        let s2 = &w.objects[NodeId::new(2)].data;
        let mut xs = g.local_intersect(r, &w).into_iter();
        assert!(Arc::ptr_eq(&s2, &xs.next().unwrap().object));
        assert!(Arc::ptr_eq(&s2, &xs.next().unwrap().object));
        assert!(Arc::ptr_eq(&s1, &xs.next().unwrap().object));
        assert!(Arc::ptr_eq(&s1, &xs.next().unwrap().object));
        assert!(xs.next().is_none());
    }

    #[test]
    fn intersecting_ray_with_transformed_group() {
        let w = WorldBuilder::default()
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
            .build();
        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector3::new(0.0, 0.0, 1.0));
        let g = &w.objects[NodeId::new(0)].data;
        let xs = g.intersect(r, &w).into_iter();
        assert_eq!(2, xs.count());
    }
}
