use crate::{
    Bounds, Intersection, Intersections, Material, Matrix4, Point, Ray, Shape, Vector3, World,
    EPSILON,
};
use approx::relative_eq;
use derive_builder::Builder;
use indextree::NodeId;
use std::any::Any;
use std::vec;

#[derive(Builder, Clone, Debug)]
pub struct Triangle {
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default)]
    pub material: Material,
    #[builder(setter(skip))]
    id: Option<NodeId>,
    #[builder(default)]
    points: [Point; 3],
    #[builder(setter(skip), default = "self.precompute_edges()?")]
    edges: [Vector3; 2],
    #[builder(setter(skip), default = "self.precompute_normal()?")]
    normal: Vector3,
}

impl TriangleBuilder {
    fn precompute_edges(&self) -> Result<[Vector3; 2], String> {
        let points = self.points.ok_or("Points must be set")?;
        let e1 = points[1] - points[0];
        let e2 = points[2] - points[0];
        Ok([e1, e2])
    }

    fn precompute_normal(&self) -> Result<Vector3, String> {
        let points = self.points.ok_or("Points must be set")?;
        let e1 = points[1] - points[0];
        let e2 = points[2] - points[0];
        let normal = e2.cross(e1).normalize();
        Ok(normal)
    }
}

impl Shape for Triangle {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn bounds(&self, _: &World) -> Bounds {
        Bounds::default()
    }

    fn local_normal_at(&self, _point: Point) -> Vector3 {
        self.normal
    }

    fn local_intersect(&self, ray: Ray, world: &World) -> Intersections {
        let direction_cross_e2 = ray.direction.cross(self.edges[1]);
        let determinant = self.edges[0].dot(direction_cross_e2);
        if relative_eq!(determinant, 0.0, epsilon = EPSILON) {
            return Intersections(vec![]);
        }

        let f = 1.0 / determinant;
        let p1_to_origin = ray.origin - self.points[0];
        let u = f * p1_to_origin.dot(direction_cross_e2);
        if u < 0.0 || u > 1.0 {
            return Intersections(vec![]);
        }

        let origin_cross_e1 = p1_to_origin.cross(self.edges[0]);
        let v = f * ray.direction.dot(origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return Intersections(vec![]);
        }

        let id = self.id.unwrap();
        let object = &world.objects[id].data;
        let time = f * self.edges[1].dot(origin_cross_e1);
        let intersections = vec![Intersection {
            time,
            object: object.clone(),
        }];

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

    #[test]
    fn constructing_a_triangle() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let t = TriangleBuilder::default()
            .points([p1, p2, p3])
            .build()
            .unwrap();
        assert_eq!(p1, t.points[0]);
        assert_eq!(p2, t.points[1]);

        assert_eq!(p3, t.points[2]);
        assert_eq!(Vector3::new(-1.0, -1.0, 0.0), t.edges[0]);
        assert_eq!(Vector3::new(1.0, -1.0, 0.0), t.edges[1]);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), t.normal);
    }

    #[test]
    fn finding_normal_on_triangle() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let t = TriangleBuilder::default()
            .points([p1, p2, p3])
            .build()
            .unwrap();
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), t.local_normal_at(p1));
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), t.local_normal_at(p2));
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), t.local_normal_at(p3));
    }

    #[test]
    fn intersecting_a_ray_parallel_to_triangle() {
        let w = WorldBuilder::default()
            .object(
                TriangleBuilder::default()
                    .points([
                        Point::new(0.0, 1.0, 0.0),
                        Point::new(-1.0, 0.0, 0.0),
                        Point::new(1.0, 0.0, 0.0),
                    ])
                    .build()
                    .unwrap(),
            )
            .build();
        let t = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector3::new(0.0, 1.0, 0.0));
        let xs = t.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let w = WorldBuilder::default()
            .object(
                TriangleBuilder::default()
                    .points([
                        Point::new(0.0, 1.0, 0.0),
                        Point::new(-1.0, 0.0, 0.0),
                        Point::new(1.0, 0.0, 0.0),
                    ])
                    .build()
                    .unwrap(),
            )
            .build();
        let t = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(1.0, 1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let w = WorldBuilder::default()
            .object(
                TriangleBuilder::default()
                    .points([
                        Point::new(0.0, 1.0, 0.0),
                        Point::new(-1.0, 0.0, 0.0),
                        Point::new(1.0, 0.0, 0.0),
                    ])
                    .build()
                    .unwrap(),
            )
            .build();
        let t = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(-1.0, 1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let w = WorldBuilder::default()
            .object(
                TriangleBuilder::default()
                    .points([
                        Point::new(0.0, 1.0, 0.0),
                        Point::new(-1.0, 0.0, 0.0),
                        Point::new(1.0, 0.0, 0.0),
                    ])
                    .build()
                    .unwrap(),
            )
            .build();
        let t = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r, &w).into_iter();
        assert_eq!(0, xs.count());
    }

    #[test]
    fn ray_strikes_triangle() {
        let w = WorldBuilder::default()
            .object(
                TriangleBuilder::default()
                    .points([
                        Point::new(0.0, 1.0, 0.0),
                        Point::new(-1.0, 0.0, 0.0),
                        Point::new(1.0, 0.0, 0.0),
                    ])
                    .build()
                    .unwrap(),
            )
            .build();
        let t = &w.objects[NodeId::new(0)].data;
        let r = Ray::new(Point::new(0.0, 0.5, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = t.local_intersect(r, &w).into_iter();
        assert_eq!(2.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());
    }
}
