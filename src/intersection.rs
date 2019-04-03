use crate::{Point, Ray, Shape, Vector3, EPSILON};
use std::vec;

#[derive(Copy, Clone, Debug)]
pub struct Computations<'a> {
    pub time: f64,
    pub object: &'a Shape,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eye_vector: Vector3,
    pub normal_vector: Vector3,
    pub reflect_vector: Vector3,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Computations<'a> {
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye_vector.dot(self.normal_vector);
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

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    pub time: f64,
    pub object: &'a Shape,
}

impl<'a> Intersection<'a> {
    pub fn prepare_computations(&self, ray: Ray, intersections: &Intersections) -> Computations {
        let point = ray.position(self.time);
        let eye_vector = -ray.direction;
        let mut normal_vector = self.object.normal_at(point);
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

        let mut containers: Vec<&Shape> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for i in intersections.0.iter() {
            if self == i {
                n1 = containers
                    .last()
                    .map(|o| o.material().refractive_index)
                    .unwrap_or(1.0);
            }

            if let Some(idx) = containers.iter().position(|&o| o == i.object) {
                containers.remove(idx);
            } else {
                containers.push(i.object);
            }

            if self == i {
                n2 = containers
                    .last()
                    .map(|o| o.material().refractive_index)
                    .unwrap_or(1.0);
                break;
            }
        }

        Computations {
            time: self.time,
            object: self.object,
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

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Intersection<'a>) -> bool {
        use std::ptr;
        self.time == other.time && ptr::eq(self.object, other.object)
    }
}

pub struct Intersections<'a>(pub Vec<Intersection<'a>>);

impl<'a> IntoIterator for Intersections<'a> {
    type Item = Intersection<'a>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> Intersections<'a> {
    pub fn hit(&self) -> Option<Intersection<'a>> {
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
    use crate::{transforms, Plane, Sphere, SphereBuilder};
    use std::ptr;

    #[test]
    fn hit_with_all_positive_times() {
        let s = Sphere::default();
        let i1 = Intersection {
            time: 1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 2.0,
            object: &s,
        };
        let xs = Intersections(vec![i1, i2]);
        let i = xs.hit().unwrap();
        assert_eq!(i1, i);
    }

    #[test]
    fn hit_with_some_negative_times() {
        let s = Sphere::default();
        let i1 = Intersection {
            time: -1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 1.0,
            object: &s,
        };
        let xs = Intersections(vec![i2, i1]);
        let i = xs.hit().unwrap();
        assert_eq!(i2, i);
    }

    #[test]
    fn hit_with_all_negative_times() {
        let s = Sphere::default();
        let i1 = Intersection {
            time: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            time: -1.0,
            object: &s,
        };
        let xs = Intersections(vec![i2, i1]);
        assert!(xs.hit().is_none());
    }

    #[test]
    fn hit_lowest_positive_intersection() {
        let s = Sphere::default();
        let i1 = Intersection {
            time: 5.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 7.0,
            object: &s,
        };
        let i3 = Intersection {
            time: -3.0,
            object: &s,
        };
        let i4 = Intersection {
            time: 2.0,
            object: &s,
        };
        let xs = Intersections(vec![i1, i2, i3, i4]);
        let i = xs.hit().unwrap();
        assert_eq!(i4, i);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection {
            time: 4.0,
            object: &shape,
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(4.0, comps.time);
        assert!(ptr::eq(&shape as &Shape, comps.object));
        assert_eq!(Point::new(0.0, 0.0, -1.0), comps.point);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.eye_vector);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.normal_vector);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn precomputing_state_of_intersection_with_hit_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection {
            time: 1.0,
            object: &shape,
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(Point::new(0.0, 0.0, 1.0), comps.point);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.eye_vector);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.normal_vector);
        assert_eq!(true, comps.inside);
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = SphereBuilder::default()
            .transform(transforms::translation(0.0, 0.0, 1.0))
            .build()
            .unwrap();
        let i = Intersection {
            time: 5.0,
            object: &shape,
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert!(comps.over_point.z < -(EPSILON / 2.0));
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let shape = Plane::default();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection {
            time: f64::sqrt(2.0),
            object: &shape,
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert_eq!(
            Vector3::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
            comps.reflect_vector
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = SphereBuilder::glass()
            .transform(transforms::scaling(2.0, 2.0, 2.0))
            .build()
            .unwrap();
        a.material.refractive_index = 1.5;
        let mut b = SphereBuilder::glass()
            .transform(transforms::translation(0.0, 0.0, -0.25))
            .build()
            .unwrap();
        b.material.refractive_index = 2.0;
        let mut c = SphereBuilder::glass()
            .transform(transforms::translation(0.0, 0.0, 0.25))
            .build()
            .unwrap();
        c.material.refractive_index = 2.5;

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector3::new(0.0, 0.0, 1.0));
        let intersections = vec![
            Intersection {
                time: 2.0,
                object: &a,
            },
            Intersection {
                time: 2.75,
                object: &b,
            },
            Intersection {
                time: 3.25,
                object: &c,
            },
            Intersection {
                time: 4.75,
                object: &b,
            },
            Intersection {
                time: 5.25,
                object: &c,
            },
            Intersection {
                time: 6.0,
                object: &a,
            },
        ];
        let xs = Intersections(intersections.clone());

        let comps = intersections[0].prepare_computations(r, &xs);
        assert_eq!(1.0, comps.n1);
        assert_eq!(1.5, comps.n2);

        let comps = intersections[1].prepare_computations(r, &xs);
        assert_eq!(1.5, comps.n1);
        assert_eq!(2.0, comps.n2);

        let comps = intersections[2].prepare_computations(r, &xs);
        assert_eq!(2.0, comps.n1);
        assert_eq!(2.5, comps.n2);

        let comps = intersections[3].prepare_computations(r, &xs);
        assert_eq!(2.5, comps.n1);
        assert_eq!(2.5, comps.n2);

        let comps = intersections[4].prepare_computations(r, &xs);
        assert_eq!(2.5, comps.n1);
        assert_eq!(1.5, comps.n2);

        let comps = intersections[5].prepare_computations(r, &xs);
        assert_eq!(1.5, comps.n1);
        assert_eq!(1.0, comps.n2);
    }

    #[test]
    fn under_point_is_below_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = SphereBuilder::glass()
            .transform(transforms::translation(0.0, 0.0, 1.0))
            .build()
            .unwrap();
        let i = Intersection {
            time: 5.0,
            object: &shape,
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = SphereBuilder::glass().build().unwrap();
        let r = Ray::new(
            Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let i1 = Intersection {
            time: -f64::sqrt(2.0) / 2.0,
            object: &shape,
        };
        let i2 = Intersection {
            time: f64::sqrt(2.0) / 2.0,
            object: &shape,
        };
        let xs = Intersections(vec![i1, i2]);
        let comps = i2.prepare_computations(r, &xs);
        assert_eq!(1.0, comps.schlick());
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let shape = SphereBuilder::glass().build().unwrap();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let i1 = Intersection {
            time: -1.0,
            object: &shape,
        };
        let i2 = Intersection {
            time: 1.0,
            object: &shape,
        };
        let xs = Intersections(vec![i1, i2]);
        let comps = i2.prepare_computations(r, &xs);
        assert!((0.04 - comps.schlick()).abs() < EPSILON);
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = SphereBuilder::glass().build().unwrap();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let i = Intersection {
            time: 1.8589,
            object: &shape,
        };
        let xs = Intersections(vec![i]);
        let comps = i.prepare_computations(r, &xs);
        assert!((0.48873 - comps.schlick()).abs() < EPSILON);
    }
}
