use crate::{
    Intersection, Intersections, Material, Matrix4, Point, Ray, Shape, Vector3, World, EPSILON,
};
use approx::relative_eq;
use derive_builder::Builder;
use indextree::NodeId;
use std::any::Any;
use std::vec;

#[derive(Builder, Clone, Debug)]
pub struct Cube {
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default)]
    pub material: Material,
    #[builder(setter(skip))]
    id: Option<NodeId>,
}

impl Default for Cube {
    fn default() -> Self {
        CubeBuilder::default().build().unwrap()
    }
}

impl Shape for Cube {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn local_normal_at(&self, Point { x, y, z }: Point, _: &World) -> Vector3 {
        let max = x.abs().max(y.abs()).max(z.abs());
        if relative_eq!(max, x.abs()) {
            Vector3::new(x, 0.0, 0.0)
        } else if relative_eq!(max, y.abs()) {
            Vector3::new(0.0, y, 0.0)
        } else {
            Vector3::new(0.0, 0.0, z)
        }
    }

    fn local_intersect(&self, ray: Ray, world: &World) -> Intersections {
        let xt = Self::check_axis(ray.origin.x, ray.direction[0]);
        let yt = Self::check_axis(ray.origin.y, ray.direction[1]);
        let zt = Self::check_axis(ray.origin.z, ray.direction[2]);

        let tmin = &[xt, yt, zt]
            .iter()
            .filter_map(|&n| n)
            .map(|(min, _)| min)
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();
        let tmax = &[xt, yt, zt]
            .iter()
            .filter_map(|&n| n)
            .map(|(_, max)| max)
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();

        let id = self.id.unwrap();
        let object = &world.objects[id].data;

        if tmin > tmax {
            Intersections(vec![])
        } else {
            Intersections(vec![
                Intersection {
                    time: *tmin,
                    object: object.clone(),
                },
                Intersection {
                    time: *tmax,
                    object: object.clone(),
                },
            ])
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

impl Cube {
    fn check_axis(origin: f64, direction: f64) -> Option<(f64, f64)> {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            let tmin = tmin_numerator / direction;
            let tmax = tmax_numerator / direction;
            (tmin, tmax)
        } else {
            return None;
        };

        if tmin > tmax {
            Some((tmax, tmin))
        } else {
            Some((tmin, tmax))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorldBuilder;

    #[test]
    fn cubes_default_transformation() {
        let s = Cube::default();
        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn ray_intersecting_cube() {
        let w = WorldBuilder::default().object(Cube::default()).build();
        let c = &w.objects[NodeId::new(0)].data;

        let r = Ray::new(Point::new(5.0, 0.5, 0.0), Vector3::new(-1.0, 0.0, 0.0));
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = Ray::new(Point::new(-5.0, 0.5, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = Ray::new(Point::new(0.5, 5.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = Ray::new(Point::new(0.5, -5.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = Ray::new(Point::new(0.5, 0.0, 5.0), Vector3::new(0.0, 0.0, -1.0));
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = Ray::new(Point::new(0.5, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = Ray::new(Point::new(0.0, 0.5, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(-1.0, xs.next().unwrap().time);
        assert_eq!(1.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());
    }

    #[test]
    fn ray_misses_cube() {
        let w = WorldBuilder::default().object(Cube::default()).build();
        let c = &w.objects[NodeId::new(0)].data;

        let r = Ray::new(
            Point::new(-2.0, 0.0, 0.0),
            Vector3::new(0.2673, 0.5345, 0.8018),
        );
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(None, xs.next());

        let r = Ray::new(
            Point::new(0.0, -2.0, 0.0),
            Vector3::new(0.8018, 0.2673, 0.5345),
        );
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(None, xs.next());

        let r = Ray::new(
            Point::new(0.0, 0.0, -2.0),
            Vector3::new(0.5345, 0.8018, 0.2673),
        );
        let mut xs = c.local_intersect(r, &w).into_iter();
        assert_eq!(None, xs.next());
    }

    #[test]
    fn normal_on_surface_of_cube() {
        let w = WorldBuilder::default().object(Cube::default()).build();
        let c = &w.objects[NodeId::new(0)].data;

        let p = Point::new(1.0, 0.5, -0.8);
        assert_eq!(Vector3::new(1.0, 0.0, 0.0), c.local_normal_at(p, &w));

        let p = Point::new(-1.0, -0.2, 0.9);
        assert_eq!(Vector3::new(-1.0, 0.0, 0.0), c.local_normal_at(p, &w));

        let p = Point::new(-0.4, 1.0, -0.1);
        assert_eq!(Vector3::new(0.0, 1.0, 0.0), c.local_normal_at(p, &w));

        let p = Point::new(0.3, -1.0, -0.7);
        assert_eq!(Vector3::new(0.0, -1.0, 0.0), c.local_normal_at(p, &w));

        let p = Point::new(-0.6, 0.3, 1.0);
        assert_eq!(Vector3::new(0.0, 0.0, 1.0), c.local_normal_at(p, &w));

        let p = Point::new(0.4, 0.4, -1.0);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), c.local_normal_at(p, &w));

        let p = Point::new(1.0, 1.0, 1.0);
        assert_eq!(Vector3::new(1.0, 0.0, 0.0), c.local_normal_at(p, &w));

        let p = Point::new(-1.0, -1.0, -1.0);
        assert_eq!(Vector3::new(-1.0, 0.0, 0.0), c.local_normal_at(p, &w));
    }
}
