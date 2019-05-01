use crate::{
    Bounds, Intersection, Intersections, Material, Matrix4, Point, Ray, Shape, Vector3, World,
    EPSILON,
};
use approx::relative_eq;
use derive_builder::Builder;
use indextree::NodeId;
use std::any::Any;
use std::f64::INFINITY;
use std::sync::Arc;
use std::vec;

#[derive(Builder, Clone, Debug)]
pub struct Cylinder {
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default)]
    pub material: Material,
    #[builder(default = "-INFINITY")]
    minimum: f64,
    #[builder(default = "INFINITY")]
    maximum: f64,
    #[builder(default = "false")]
    closed: bool,
    #[builder(setter(skip))]
    id: Option<NodeId>,
}

impl Default for Cylinder {
    fn default() -> Self {
        CylinderBuilder::default().build().unwrap()
    }
}

impl Shape for Cylinder {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn bounds(&self, _: &World) -> Bounds {
        Bounds::default()
            + Point::new(-1.0, self.minimum, -1.0)
            + Point::new(1.0, self.maximum, 1.0)
    }

    fn local_normal_at(&self, Point { x, y, z }: Point) -> Vector3 {
        let dist = x.powi(2) + z.powi(2);
        if dist < 1.0 && y >= self.maximum - EPSILON {
            Vector3::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && y <= self.minimum + EPSILON {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(x, 0.0, z)
        }
    }

    fn local_intersect(&self, ray: Ray, world: &World) -> Intersections {
        let a = ray.direction[0].powi(2) + ray.direction[2].powi(2);
        let b = 2.0 * ray.origin.x * ray.direction[0] + 2.0 * ray.origin.z * ray.direction[2];
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return Intersections(vec![]);
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

        let mut intersections = vec![];
        let id = self.id.unwrap();
        let object = &world.objects[id].data;

        let y0 = ray.origin.y + t0 * ray.direction[1];
        if self.minimum < y0 && y0 < self.maximum {
            intersections.push(Intersection {
                time: t0,
                object: object.clone(),
            });
        }

        let y1 = ray.origin.y + t1 * ray.direction[1];
        if self.minimum < y1 && y1 < self.maximum {
            intersections.push(Intersection {
                time: t1,
                object: object.clone(),
            });
        }

        let cap_intersections = self.intersect_caps(ray, object.clone());
        intersections.extend_from_slice(&cap_intersections);

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

impl Cylinder {
    fn check_cap(&self, ray: Ray, time: f64) -> bool {
        let x = ray.origin.x + time * ray.direction[0];
        let z = ray.origin.z + time * ray.direction[2];

        (x.powi(2) + z.powi(2)) <= 1.0
    }

    fn intersect_caps(&self, ray: Ray, object: Arc<Shape + Sync + Send>) -> Vec<Intersection> {
        let mut intersections = vec![];

        if !self.closed || relative_eq!(ray.direction[1], 0.0, epsilon = EPSILON) {
            return intersections;
        }

        let time = (self.minimum - ray.origin.y) / ray.direction[1];
        if self.check_cap(ray, time) {
            intersections.push(Intersection {
                time,
                object: object.clone(),
            })
        }

        let time = (self.maximum - ray.origin.y) / ray.direction[1];
        if self.check_cap(ray, time) {
            intersections.push(Intersection {
                time,
                object: object.clone(),
            })
        }

        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorldBuilder;
    use approx::assert_relative_eq;

    #[test]
    fn ray_misses_cylinder() {
        let w = WorldBuilder::default().object(Cylinder::default()).build();
        let cyl = &w.objects[NodeId::new(0)].data;

        let direction = Vector3::new(0.0, 1.0, 0.0).normalize();
        let r = Ray::new(Point::new(1.0, 0.0, 0.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());

        let direction = Vector3::new(0.0, 1.0, 0.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());

        let direction = Vector3::new(1.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_strikes_cylinder() {
        let w = WorldBuilder::default().object(Cylinder::default()).build();
        let cyl = &w.objects[NodeId::new(0)].data;

        let direction = Vector3::new(0.0, 0.0, 1.0).normalize();
        let r = Ray::new(Point::new(1.0, 0.0, -5.0), direction);
        let mut xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let direction = Vector3::new(0.0, 0.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), direction);
        let mut xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let direction = Vector3::new(0.1, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.5, 0.0, -5.0), direction);
        let mut xs = cyl.local_intersect(r, &w).into_iter();
        assert_relative_eq!(6.80798, xs.next().unwrap().time, epsilon = EPSILON);
        assert_relative_eq!(7.08872, xs.next().unwrap().time, epsilon = EPSILON);
        assert_eq!(None, xs.next());
    }

    #[test]
    fn normal_vector_on_cylinder() {
        let w = WorldBuilder::default().object(Cylinder::default()).build();
        let cyl = &w.objects[NodeId::new(0)].data;

        assert_eq!(
            Vector3::new(1.0, 0.0, 0.0),
            cyl.local_normal_at(Point::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Vector3::new(0.0, 0.0, -1.0),
            cyl.local_normal_at(Point::new(0.0, 5.0, -1.0))
        );
        assert_eq!(
            Vector3::new(0.0, 0.0, 1.0),
            cyl.local_normal_at(Point::new(0.0, -2.0, 1.0))
        );
        assert_eq!(
            Vector3::new(-1.0, 0.0, 0.0),
            cyl.local_normal_at(Point::new(-1.0, 1.0, 0.0))
        );
    }

    #[test]
    fn default_minimum_and_maximum_for_cylinder() {
        let cyl = Cylinder::default();
        assert_eq!(-INFINITY, cyl.minimum);
        assert_eq!(INFINITY, cyl.maximum);
    }

    #[test]
    fn default_closed_value_for_cylinder() {
        let cyl = Cylinder::default();
        assert_eq!(false, cyl.closed);
    }

    #[test]
    fn intersecting_constrained_cylinder() {
        let w = WorldBuilder::default()
            .object(
                CylinderBuilder::default()
                    .minimum(1.0)
                    .maximum(2.0)
                    .build()
                    .unwrap(),
            )
            .build();
        let cyl = &w.objects[NodeId::new(0)].data;

        let direction = Vector3::new(0.1, 1.0, 0.0).normalize();
        let r = Ray::new(Point::new(0.0, 1.5, -5.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());

        let direction = Vector3::new(0.0, 0.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 3.0, -5.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());

        let direction = Vector3::new(0.0, 0.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());

        let direction = Vector3::new(0.0, 0.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());

        let direction = Vector3::new(0.0, 0.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());

        let direction = Vector3::new(0.0, 0.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 1.5, -2.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(2, xs.count());
    }

    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        let w = WorldBuilder::default()
            .object(
                CylinderBuilder::default()
                    .minimum(1.0)
                    .maximum(2.0)
                    .closed(true)
                    .build()
                    .unwrap(),
            )
            .build();
        let cyl = &w.objects[NodeId::new(0)].data;

        let direction = Vector3::new(0.0, -1.0, 0.0).normalize();
        let r = Ray::new(Point::new(0.0, 3.0, 0.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(2, xs.count());

        let direction = Vector3::new(0.0, -1.0, 2.0).normalize();
        let r = Ray::new(Point::new(0.0, 3.0, -2.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(2, xs.count());

        let direction = Vector3::new(0.0, -1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 4.0, -2.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(2, xs.count());

        let direction = Vector3::new(0.0, 1.0, 2.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -2.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(2, xs.count());

        let direction = Vector3::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), direction);
        let xs = cyl.local_intersect(r, &w).into_iter();
        assert_eq!(2, xs.count());
    }

    #[test]
    fn normal_vector_on_end_caps_of_closed_cylinder() {
        let w = WorldBuilder::default()
            .object(
                CylinderBuilder::default()
                    .minimum(1.0)
                    .maximum(2.0)
                    .closed(true)
                    .build()
                    .unwrap(),
            )
            .build();
        let cyl = &w.objects[NodeId::new(0)].data;

        assert_eq!(
            Vector3::new(0.0, -1.0, 0.0),
            cyl.local_normal_at(Point::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Vector3::new(0.0, -1.0, 0.0),
            cyl.local_normal_at(Point::new(0.5, 1.0, 0.0))
        );
        assert_eq!(
            Vector3::new(0.0, -1.0, 0.0),
            cyl.local_normal_at(Point::new(0.0, 1.0, 0.5))
        );
        assert_eq!(
            Vector3::new(0.0, 1.0, 0.0),
            cyl.local_normal_at(Point::new(0.0, 2.0, 0.0))
        );
        assert_eq!(
            Vector3::new(0.0, 1.0, 0.0),
            cyl.local_normal_at(Point::new(0.5, 2.0, 0.0))
        );
        assert_eq!(
            Vector3::new(0.0, 1.0, 0.0),
            cyl.local_normal_at(Point::new(0.0, 2.0, 0.5))
        );
    }
}
