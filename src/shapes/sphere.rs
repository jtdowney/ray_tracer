use crate::{
    Intersection, Intersections, Material, MaterialBuilder, Matrix4, Point, Ray, Shape, Vector3,
    World,
};
use derive_builder::Builder;
use indextree::NodeId;
use std::any::Any;
use std::vec;

#[derive(Builder, Clone, Debug)]
pub struct Sphere {
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default)]
    pub material: Material,
    #[builder(setter(skip))]
    id: Option<NodeId>,
}

impl SphereBuilder {
    pub fn glass() -> Self {
        let mut builder = SphereBuilder::default();
        builder.material(
            MaterialBuilder::default()
                .transparency(1.0)
                .refractive_index(1.5)
                .build()
                .unwrap(),
        );
        builder
    }
}

impl Default for Sphere {
    fn default() -> Self {
        SphereBuilder::default().build().unwrap()
    }
}

impl Shape for Sphere {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn local_normal_at(&self, point: Point, _: &World) -> Vector3 {
        point - Point::default()
    }

    fn local_intersect(&self, ray: Ray, world: &World) -> Intersections {
        let object_to_ray = ray.origin - Point::default();
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(object_to_ray);
        let c = object_to_ray.dot(object_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        let mut intersections = vec![];
        if discriminant >= 0.0 {
            let id = self.id.unwrap();
            let object = &world.objects[id].data;

            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            intersections.push(Intersection {
                time: t1,
                object: object.clone(),
            });
            intersections.push(Intersection {
                time: t2,
                object: object.clone(),
            });
        }

        Intersections(intersections)
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
    fn spheres_default_transformation() {
        let s = Sphere::default();
        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn normal_on_sphere_at_x_axis() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        assert_eq!(
            Vector3::new(1.0, 0.0, 0.0),
            s.normal_at(Point::new(1.0, 0.0, 0.0), &w)
        );
    }

    #[test]
    fn normal_on_sphere_at_y_axis() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        assert_eq!(
            Vector3::new(0.0, 1.0, 0.0),
            s.normal_at(Point::new(0.0, 1.0, 0.0), &w)
        );
    }

    #[test]
    fn normal_on_sphere_at_z_axis() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        assert_eq!(
            Vector3::new(0.0, 0.0, 1.0),
            s.normal_at(Point::new(0.0, 0.0, 1.0), &w)
        );
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        assert_eq!(
            Vector3::new(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            ),
            s.normal_at(
                Point::new(
                    f64::sqrt(3.0) / 3.0,
                    f64::sqrt(3.0) / 3.0,
                    f64::sqrt(3.0) / 3.0
                ),
                &w
            )
        );
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        let n = s.normal_at(
            Point::new(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
            ),
            &w,
        );
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r, &w).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r, &w).into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn ray_misses_sphere() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = s.intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r, &w).into_iter();
        assert_eq!(-1.0, xs.next().unwrap().time);
        assert_eq!(1.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn sphere_behind_ray() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r, &w).into_iter();
        assert_eq!(-6.0, xs.next().unwrap().time);
        assert_eq!(-4.0, xs.next().unwrap().time);
        assert!(xs.next().is_none());
    }

    #[test]
    fn intersection_has_time_and_object() {
        let s = Sphere::default();
        let object = Arc::new(s) as Arc<Shape + Sync + Send>;
        let i = Intersection {
            time: 3.5,
            object: object.clone(),
        };

        assert_eq!(3.5, i.time);
        assert!(Arc::ptr_eq(&object, &i.object));
    }

    #[test]
    fn intersect_sets_objects() {
        let w = WorldBuilder::default().object(Sphere::default()).build();
        let s = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = s.intersect(r, &w).into_iter();
        assert!(Arc::ptr_eq(&s, &xs.next().unwrap().object));
        assert!(Arc::ptr_eq(&s, &xs.next().unwrap().object));
        assert!(xs.next().is_none());
    }

    #[test]
    fn glass_sphere_helper() {
        let s = SphereBuilder::glass().build().unwrap();
        assert_eq!(1.0, s.material.transparency);
        assert_eq!(1.5, s.material.refractive_index);
    }
}
