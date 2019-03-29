use crate::{matrix, Point, Ray, Sphere, Vector3, EPSILON};
use std::vec;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Computations<'a> {
    pub time: f64,
    pub object: &'a Sphere,
    pub point: Point,
    pub over_point: Point,
    pub eye_vector: Vector3,
    pub normal_vector: Vector3,
    pub inside: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub time: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn prepare_computations(
        &self,
        ray: Ray,
    ) -> Result<Computations, matrix::NotInvertableError> {
        let point = ray.position(self.time);
        let eye_vector = -ray.direction;
        let mut normal_vector = self.object.normal_at(point)?;
        let inside: bool;

        if normal_vector.dot(eye_vector) < 0.0 {
            inside = true;
            normal_vector = -normal_vector;
        } else {
            inside = false;
        }

        let over_point = point + normal_vector * EPSILON;

        Ok(Computations {
            time: self.time,
            object: self.object,
            over_point,
            point,
            eye_vector,
            normal_vector,
            inside,
        })
    }
}

#[derive(Debug)]
pub struct Intersections<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

impl<'a> IntoIterator for Intersections<'a> {
    type Item = Intersection<'a>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.intersections.into_iter()
    }
}

impl<'a> Intersections<'a> {
    pub fn hit(self) -> Option<Intersection<'a>> {
        self.into_iter()
            .filter(|i| i.time >= 0.0)
            .min_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transforms;

    #[test]
    fn test_hit_with_all_positive_times() {
        let s = Sphere::default();
        let i1 = Intersection {
            time: 1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 2.0,
            object: &s,
        };
        let xs = Intersections {
            intersections: vec![i1, i2],
        };
        let i = xs.hit().unwrap();
        assert_eq!(i1, i);
    }

    #[test]
    fn test_hit_with_some_negative_times() {
        let s = Sphere::default();
        let i1 = Intersection {
            time: -1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 1.0,
            object: &s,
        };
        let xs = Intersections {
            intersections: vec![i2, i1],
        };
        let i = xs.hit().unwrap();
        assert_eq!(i2, i);
    }

    #[test]
    fn test_hit_with_all_negative_times() {
        let s = Sphere::default();
        let i1 = Intersection {
            time: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            time: -1.0,
            object: &s,
        };
        let xs = Intersections {
            intersections: vec![i2, i1],
        };
        assert!(xs.hit().is_none());
    }

    #[test]
    fn test_hit_lowest_positive_intersection() {
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
        let xs = Intersections {
            intersections: vec![i1, i2, i3, i4],
        };
        let i = xs.hit().unwrap();
        assert_eq!(i4, i);
    }

    #[test]
    fn test_precomputing_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection {
            time: 4.0,
            object: &shape,
        };
        let comps = i.prepare_computations(r).unwrap();
        assert_eq!(4.0, comps.time);
        assert_eq!(&shape, comps.object);
        assert_eq!(Point::new(0.0, 0.0, -1.0), comps.point);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.eye_vector);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.normal_vector);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn test_precomputing_state_of_intersection_with_hit_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection {
            time: 1.0,
            object: &shape,
        };
        let comps = i.prepare_computations(r).unwrap();
        assert_eq!(Point::new(0.0, 0.0, 1.0), comps.point);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.eye_vector);
        assert_eq!(Vector3::new(0.0, 0.0, -1.0), comps.normal_vector);
        assert_eq!(true, comps.inside);
    }

    #[test]
    fn test_hit_should_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::default();
        shape.transform = transforms::translation(0.0, 0.0, 1.0);
        let i = Intersection {
            time: 5.0,
            object: &shape,
        };
        let comps = i.prepare_computations(r).unwrap();
        assert!(comps.over_point.z < -(EPSILON / 2.0));
        assert!(comps.point.z > comps.over_point.z);
    }
}
