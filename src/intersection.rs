use crate::{Point, Ray, Shape, Vector3, World, EPSILON};
use std::sync::Arc;
use std::vec;

#[derive(Debug)]
pub struct Computations {
    pub time: f64,
    pub object: Arc<dyn Shape + Sync + Send>,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eye_vector: Vector3,
    pub normal_vector: Vector3,
    pub reflect_vector: Vector3,
    pub inside: bool,
    pub n1: f32,
    pub n2: f32,
}

impl Computations {
    pub fn schlick(&self) -> f32 {
        let mut cos = self.eye_vector.dot(self.normal_vector) as f32;
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[derive(Clone, Debug)]
pub struct Intersection {
    pub time: f64,
    pub object: Arc<dyn Shape + Sync + Send>,
}

impl Intersection {
    pub fn prepare_computations(
        &self,
        ray: Ray,
        intersections: &Intersections,
        world: &World,
    ) -> Computations {
        let point = ray.position(self.time);
        let eye_vector = -ray.direction;
        let mut normal_vector = self.object.normal_at(point, world);
        let inside: bool;

        if normal_vector.dot(eye_vector) < 0.0 {
            inside = true;
            normal_vector = -normal_vector;
        } else {
            inside = false;
        }

        let reflect_vector = ray.direction.reflect(normal_vector);
        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;

        let mut containers: Vec<Arc<dyn Shape + Sync + Send>> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for i in intersections.0.iter() {
            if self == i {
                n1 = containers
                    .last()
                    .map_or(1.0, |o| o.material().refractive_index);
            }

            if let Some(idx) = containers.iter().position(|o| Arc::ptr_eq(o, &i.object)) {
                containers.remove(idx);
            } else {
                containers.push(i.object.clone());
            }

            if self == i {
                n2 = containers
                    .last()
                    .map_or(1.0, |o| o.material().refractive_index);
                break;
            }
        }

        Computations {
            time: self.time,
            object: self.object.clone(),
            over_point,
            under_point,
            point,
            eye_vector,
            normal_vector,
            reflect_vector,
            inside,
            n1,
            n2,
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.time == other.time && Arc::ptr_eq(&self.object, &other.object)
    }
}

pub struct Intersections(pub Vec<Intersection>);

impl IntoIterator for Intersections {
    type Item = Intersection;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Intersections {
    pub fn hit(&self) -> Option<Intersection> {
        self.0
            .iter()
            .cloned()
            .filter(|i| i.time >= 0.0)
            .min_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transforms, Plane, Sphere, SphereBuilder, WorldBuilder};
    use approx::assert_relative_eq;
    use indextree::NodeId;

    #[test]
    fn hit_with_all_positive_times() {
        let s = Sphere::default();
        let object = Arc::new(s);
        let i1 = Intersection {
            time: 1.0,
            object: object.clone(),
        };
        let i2 = Intersection {
            time: 2.0,
            object: object.clone(),
        };
        let xs = Intersections(vec![i1.clone(), i2]);
        let i = xs.hit().unwrap();
        assert_eq!(i1, i);
    }

    #[test]
    fn hit_with_some_negative_times() {
        let s = Sphere::default();
        let object = Arc::new(s);
        let i1 = Intersection {
            time: -1.0,
            object: object.clone(),
        };
        let i2 = Intersection {
            time: 1.0,
            object: object.clone(),
        };
        let xs = Intersections(vec![i2.clone(), i1]);
        let i = xs.hit().unwrap();
        assert_eq!(i2, i);
    }

    #[test]
    fn hit_with_all_negative_times() {
        let s = Sphere::default();
        let object = Arc::new(s);
        let i1 = Intersection {
            time: -2.0,
            object: object.clone(),
        };
        let i2 = Intersection {
            time: -1.0,
            object: object.clone(),
        };
        let xs = Intersections(vec![i2, i1]);
        assert!(xs.hit().is_none());
    }

    #[test]
    fn hit_lowest_positive_intersection() {
        let s = Sphere::default();
        let object = Arc::new(s);
        let i1 = Intersection {
            time: 5.0,
            object: object.clone(),
        };
        let i2 = Intersection {
            time: 7.0,
            object: object.clone(),
        };
        let i3 = Intersection {
            time: -3.0,
            object: object.clone(),
        };
        let i4 = Intersection {
            time: 2.0,
            object: object.clone(),
        };
        let xs = Intersections(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit().unwrap();
        assert_eq!(i4, i);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let w = WorldBuilder::test_world().object(Sphere::default()).build();
        let shape = &w.objects[NodeId::new(2)].data;
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 4.0,
            object: shape.clone(),
        };
        let xs = Intersections(vec![i.clone()]);
        let comps = i.prepare_computations(r, &xs, &w);
        assert_eq!(4.0, comps.time);
        assert!(Arc::ptr_eq(&shape, &comps.object));
        assert_eq!(Point::new(0.0, 0.0, -1.0), comps.point);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.eye_vector);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.normal_vector);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn precomputing_state_of_intersection_with_hit_inside() {
        let w = WorldBuilder::test_world().object(Sphere::default()).build();
        let shape = &w.objects[NodeId::new(2)].data;
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 1.0,
            object: shape.clone(),
        };
        let xs = Intersections(vec![i.clone()]);
        let comps = i.prepare_computations(r, &xs, &w);
        assert_eq!(Point::new(0.0, 0.0, 1.0), comps.point);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.eye_vector);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.normal_vector);
        assert_eq!(true, comps.inside);
    }

    #[test]
    fn hit_should_offset_point() {
        let w = WorldBuilder::test_world()
            .object(
                SphereBuilder::default()
                    .transform(transforms::translation(0.0, 0.0, 1.0))
                    .build()
                    .unwrap(),
            )
            .build();
        let shape = &w.objects[NodeId::new(2)].data;
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 5.0,
            object: shape.clone(),
        };
        let xs = Intersections(vec![i.clone()]);
        let comps = i.prepare_computations(r, &xs, &w);
        assert!(comps.over_point.z < -(EPSILON / 2.0));
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let w = WorldBuilder::test_world().object(Plane::default()).build();
        let shape = &w.objects[NodeId::new(2)].data;
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: shape.clone(),
        };
        let xs = Intersections(vec![i.clone()]);
        let comps = i.prepare_computations(r, &xs, &w);
        assert_eq!(
            Vector3::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
            comps.reflect_vector
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut w = WorldBuilder::test_world()
            .object(
                SphereBuilder::glass()
                    .transform(transforms::scaling(2.0, 2.0, 2.0))
                    .build()
                    .unwrap(),
            )
            .object(
                SphereBuilder::glass()
                    .transform(transforms::translation(0.0, 0.0, -0.25))
                    .build()
                    .unwrap(),
            )
            .object(
                SphereBuilder::glass()
                    .transform(transforms::translation(0.0, 0.0, 0.25))
                    .build()
                    .unwrap(),
            )
            .build();

        {
            let a = &mut w.objects[NodeId::new(2)].data;
            Arc::get_mut(a)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<Sphere>()
                .unwrap()
                .material
                .refractive_index = 1.5;
        }

        {
            let b = &mut w.objects[NodeId::new(3)].data;
            Arc::get_mut(b)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<Sphere>()
                .unwrap()
                .material
                .refractive_index = 2.0;
        }

        {
            let c = &mut w.objects[NodeId::new(4)].data;
            Arc::get_mut(c)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<Sphere>()
                .unwrap()
                .material
                .refractive_index = 2.5;
        }

        let a = &w.objects[NodeId::new(2)].data;
        let b = &w.objects[NodeId::new(3)].data;
        let c = &w.objects[NodeId::new(4)].data;

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector3::new(0.0, 0.0, 1.0));
        let intersections = vec![
            Intersection {
                time: 2.0,
                object: a.clone(),
            },
            Intersection {
                time: 2.75,
                object: b.clone(),
            },
            Intersection {
                time: 3.25,
                object: c.clone(),
            },
            Intersection {
                time: 4.75,
                object: b.clone(),
            },
            Intersection {
                time: 5.25,
                object: c.clone(),
            },
            Intersection {
                time: 6.0,
                object: a.clone(),
            },
        ];
        let xs = Intersections(intersections.clone());

        let comps = intersections[0].prepare_computations(r, &xs, &w);
        assert_eq!(1.0, comps.n1);
        assert_eq!(1.5, comps.n2);

        let comps = intersections[1].prepare_computations(r, &xs, &w);
        assert_eq!(1.5, comps.n1);
        assert_eq!(2.0, comps.n2);

        let comps = intersections[2].prepare_computations(r, &xs, &w);
        assert_eq!(2.0, comps.n1);
        assert_eq!(2.5, comps.n2);

        let comps = intersections[3].prepare_computations(r, &xs, &w);
        assert_eq!(2.5, comps.n1);
        assert_eq!(2.5, comps.n2);

        let comps = intersections[4].prepare_computations(r, &xs, &w);
        assert_eq!(2.5, comps.n1);
        assert_eq!(1.5, comps.n2);

        let comps = intersections[5].prepare_computations(r, &xs, &w);
        assert_eq!(1.5, comps.n1);
        assert_eq!(1.0, comps.n2);
    }

    #[test]
    fn under_point_is_below_surface() {
        let w = WorldBuilder::test_world()
            .object(
                SphereBuilder::glass()
                    .transform(transforms::translation(0.0, 0.0, 1.0))
                    .build()
                    .unwrap(),
            )
            .build();
        let shape = &w.objects[NodeId::new(2)].data;
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 5.0,
            object: shape.clone(),
        };
        let xs = Intersections(vec![i.clone()]);
        let comps = i.prepare_computations(r, &xs, &w);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let w = WorldBuilder::test_world()
            .object(SphereBuilder::glass().build().unwrap())
            .build();
        let shape = &w.objects[NodeId::new(2)].data;
        let r = Ray::new(
            Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let i1 = Intersection {
            time: -f64::sqrt(2.0) / 2.0,
            object: shape.clone(),
        };
        let i2 = Intersection {
            time: f64::sqrt(2.0) / 2.0,
            object: shape.clone(),
        };
        let xs = Intersections(vec![i1, i2.clone()]);
        let comps = i2.prepare_computations(r, &xs, &w);
        assert_eq!(1.0, comps.schlick());
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let w = WorldBuilder::test_world()
            .object(SphereBuilder::glass().build().unwrap())
            .build();
        let shape = &w.objects[NodeId::new(2)].data;
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let i1 = Intersection {
            time: -1.0,
            object: shape.clone(),
        };
        let i2 = Intersection {
            time: 1.0,
            object: shape.clone(),
        };
        let xs = Intersections(vec![i1, i2.clone()]);
        let comps = i2.prepare_computations(r, &xs, &w);
        assert_relative_eq!(0.04, comps.schlick(), epsilon = EPSILON as f32);
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let w = WorldBuilder::test_world()
            .object(SphereBuilder::glass().build().unwrap())
            .build();
        let shape = &w.objects[NodeId::new(2)].data;
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 1.8589,
            object: shape.clone(),
        };
        let xs = Intersections(vec![i.clone()]);
        let comps = i.prepare_computations(r, &xs, &w);
        assert_relative_eq!(0.48873, comps.schlick(), epsilon = EPSILON as f32);
    }
}
