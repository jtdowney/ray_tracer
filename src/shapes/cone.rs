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
pub struct Cone {
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

impl Default for Cone {
    fn default() -> Self {
        ConeBuilder::default().build().unwrap()
    }
}

impl Shape for Cone {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn bounds(&self, _: &World) -> Bounds {
        let radius = self.maximum.abs().max(self.minimum.abs());

        Bounds::default()
            + Point::new(-radius, self.minimum, -radius)
            + Point::new(radius, self.maximum, radius)
    }

    fn local_normal_at(&self, Point { x, y, z }: Point) -> Vector3 {
        let dist = x.powi(2) + z.powi(2);
        if dist < 1.0 && y >= self.maximum - EPSILON {
            Vector3::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && y <= self.minimum + EPSILON {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            let ny = dist.sqrt();
            let ny = if y > 0.0 { -ny } else { ny };
            Vector3::new(x, ny, z)
        }
    }

    fn local_intersect(&self, ray: Ray, world: &World) -> Intersections {
        let a = ray.direction[0].powi(2) - ray.direction[1].powi(2) + ray.direction[2].powi(2);
        let b = 2.0 * ray.origin.x * ray.direction[0] - 2.0 * ray.origin.y * ray.direction[1]
            + 2.0 * ray.origin.z * ray.direction[2];
        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        let mut intersections = vec![];
        let a_zero = relative_eq!(a, 0.0, epsilon = EPSILON);
        let b_zero = relative_eq!(b, 0.0, epsilon = EPSILON);
        if a_zero && b_zero {
            return Intersections(intersections);
        }

        let id = self.id.unwrap();
        let object = &world.objects[id].data;

        if a_zero {
            let time = -c / (2.0 * b);
            intersections.push(Intersection {
                time,
                object: object.clone(),
            });
        } else {
            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                return Intersections(intersections);
            }

            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);
            let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

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
        }

        if self.closed {
            let cap_intersections = self.intersect_caps(ray, object.clone());
            intersections.extend_from_slice(&cap_intersections);
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

impl Cone {
    fn check_cap(&self, ray: Ray, time: f64, radius: f64) -> bool {
        let x = ray.origin.x + time * ray.direction[0];
        let z = ray.origin.z + time * ray.direction[2];

        (x.powi(2) + z.powi(2)) <= radius
    }

    fn intersect_caps(&self, ray: Ray, object: Arc<Shape + Sync + Send>) -> Vec<Intersection> {
        let mut intersections = vec![];

        if !self.closed || relative_eq!(ray.direction[1], 0.0, epsilon = EPSILON) {
            return intersections;
        }

        let time = (self.minimum - ray.origin.y) / ray.direction[1];
        if self.check_cap(ray, time, self.minimum.abs()) {
            intersections.push(Intersection {
                time,
                object: object.clone(),
            })
        }

        let time = (self.maximum - ray.origin.y) / ray.direction[1];
        if self.check_cap(ray, time, self.maximum.abs()) {
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
    fn ray_strikes_cone() {
        let w = WorldBuilder::default().object(Cone::default()).build();
        let cone = &w.objects[NodeId::new(0)].data;

        let direction = Vector3::new(0.0, 0.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), direction);
        let mut xs = cone.local_intersect(r, &w).into_iter();
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(5.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let direction = Vector3::new(1.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), direction);
        let mut xs = cone.local_intersect(r, &w).into_iter();
        assert_relative_eq!(8.66025, xs.next().unwrap().time, epsilon = EPSILON);
        assert_relative_eq!(8.66025, xs.next().unwrap().time, epsilon = EPSILON);
        assert_eq!(None, xs.next());

        let direction = Vector3::new(-0.5, -1.0, 1.0).normalize();
        let r = Ray::new(Point::new(1.0, 1.0, -5.0), direction);
        let mut xs = cone.local_intersect(r, &w).into_iter();
        assert_relative_eq!(4.55006, xs.next().unwrap().time, epsilon = EPSILON);
        assert_relative_eq!(49.44994, xs.next().unwrap().time, epsilon = EPSILON);
        assert_eq!(None, xs.next());
    }

    #[test]
    fn intersecting_a_cone_parallel_to_one_of_its_halves() {
        let w = WorldBuilder::default().object(Cone::default()).build();
        let cone = &w.objects[NodeId::new(0)].data;

        let direction = Vector3::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
        let mut xs = cone.local_intersect(r, &w).into_iter();
        assert_relative_eq!(0.35355, xs.next().unwrap().time, epsilon = EPSILON);
        assert_eq!(None, xs.next());
    }

    #[test]
    fn intersecting_codes_end_caps() {
        let w = WorldBuilder::default()
            .object(
                ConeBuilder::default()
                    .minimum(-0.5)
                    .maximum(0.5)
                    .closed(true)
                    .build()
                    .unwrap(),
            )
            .build();
        let cone = &w.objects[NodeId::new(0)].data;

        let direction = Vector3::new(0.0, 1.0, 0.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), direction);
        let xs = cone.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());

        let direction = Vector3::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -0.25), direction);
        let xs = cone.local_intersect(r, &w).into_iter();
        assert_eq!(2, xs.count());

        let direction = Vector3::new(0.0, 1.0, 0.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -0.25), direction);
        let xs = cone.local_intersect(r, &w).into_iter();
        assert_eq!(4, xs.count());
    }

    #[test]
    fn computing_normal_vector_on_cone() {
        let w = WorldBuilder::default().object(Cone::default()).build();
        let cone = &w.objects[NodeId::new(0)].data;

        assert_eq!(
            Vector3::new(0.0, 0.0, 0.0),
            cone.local_normal_at(Point::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Vector3::new(1.0, -f64::sqrt(2.0), 1.0),
            cone.local_normal_at(Point::new(1.0, 1.0, 1.0))
        );
        assert_eq!(
            Vector3::new(-1.0, 1.0, 0.0),
            cone.local_normal_at(Point::new(-1.0, -1.0, 0.0))
        );
    }
}
