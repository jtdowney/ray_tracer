use crate::{Point, Ray, Shape, Vector, EPSILON};
use core::f64;
use ord_subset::OrdSubsetIterExt;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    iter::FromIterator,
    ops::Index,
    ptr, slice, vec,
};

pub fn intersection(time: f64, object: &dyn Shape) -> Intersection {
    Intersection { time, object }
}

#[derive(Clone)]
pub struct Intersection<'o> {
    pub time: f64,
    pub object: &'o dyn Shape,
}

impl<'o> Debug for Intersection<'o> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Intersection")
            .field("time", &self.time)
            .field("object", &format!("{:p}", self.object))
            .finish()
    }
}

impl<'o> PartialEq for Intersection<'o> {
    fn eq(&self, other: &Self) -> bool {
        let self_ptr = self.object as *const _ as *const u8;
        let other_ptr = other.object as *const _ as *const u8;
        self.time == other.time && ptr::eq(self_ptr, other_ptr)
    }
}

impl<'o> Intersection<'o> {
    pub fn prepare_computations(
        &self,
        ray: Ray,
        intersections: &Intersections<'o>,
    ) -> Computations<'o> {
        let time = self.time;
        let object = self.object;
        let point = ray.position(time);
        let eye_vector = -ray.direction;
        let mut normal_vector = object.normal_at(point);
        let inside;

        if normal_vector.dot(eye_vector).is_sign_negative() {
            inside = true;
            normal_vector = -normal_vector;
        } else {
            inside = false;
        }

        let reflect_vector = ray.direction.reflect(normal_vector);
        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        let mut objects: HashMap<usize, &dyn Shape> = HashMap::new();
        let mut containers = vec![];
        for i in intersections.iter() {
            if i == self {
                n1 = containers
                    .last()
                    .map(|p| objects[p].material().refractive_index)
                    .unwrap_or(1.0);
            }

            let ptr = i.object as *const _ as *const u8 as usize;
            if containers.contains(&ptr) {
                containers.retain(|&p| p != ptr);
            } else {
                containers.push(ptr);
                objects.insert(ptr, i.object);
            }

            if i == self {
                n2 = containers
                    .last()
                    .map(|p| objects[p].material().refractive_index)
                    .unwrap_or(1.0);
                break;
            }
        }

        Computations {
            time,
            object,
            point,
            over_point,
            under_point,
            eye_vector,
            normal_vector,
            reflect_vector,
            inside,
            n1,
            n2,
        }
    }
}

pub struct Computations<'o> {
    pub time: f64,
    pub object: &'o dyn Shape,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
    pub reflect_vector: Vector,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

impl<'o> Computations<'o> {
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye_vector.dot(self.normal_vector);
        if self.n1 > self.n2 {
            let n_ratio = self.n1 / self.n2;
            let sin2_t = n_ratio.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = f64::sqrt(1.0 - sin2_t);
        }

        let r0 = f64::powi((self.n1 - self.n2) / (self.n1 + self.n2), 2);
        r0 + (1.0 - r0) * f64::powi(1.0 - cos, 5)
    }
}

pub struct Intersections<'o>(Vec<Intersection<'o>>);

impl<'o> Intersections<'o> {
    pub fn empty() -> Self {
        Intersections(vec![])
    }

    pub fn iter(&self) -> slice::Iter<Intersection<'o>> {
        self.0.iter()
    }
}

impl<'o> Intersections<'o> {
    pub fn hit(&self) -> Option<&Intersection<'o>> {
        self.iter()
            .filter(|i| i.time >= 0.0)
            .ord_subset_min_by_key(|i| i.time)
    }
}

impl<'o> Index<usize> for Intersections<'o> {
    type Output = Intersection<'o>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'o> From<Vec<Intersection<'o>>> for Intersections<'o> {
    fn from(intersections: Vec<Intersection<'o>>) -> Self {
        Intersections(intersections)
    }
}

impl<'o> FromIterator<Intersection<'o>> for Intersections<'o> {
    fn from_iter<I: IntoIterator<Item = Intersection<'o>>>(iter: I) -> Self {
        let intersections = iter.into_iter().collect();
        Intersections(intersections)
    }
}

impl<'o> IntoIterator for Intersections<'o> {
    type Item = Intersection<'o>;
    type IntoIter = vec::IntoIter<Intersection<'o>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::{
        plane, point, ray, sphere, transformations, vector, MaterialBuilder, Sphere, SphereBuilder,
    };

    fn glass_sphere_helper(index: f64) -> Sphere {
        SphereBuilder::default()
            .material(
                MaterialBuilder::default()
                    .transparency(1.0)
                    .refractive_index(index)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }

    #[test]
    fn intersection_has_time_and_object() {
        let s = sphere();
        let i = intersection(3.5, &s);

        assert_eq!(i.time, 3.5);
        assert!(ptr::eq(i.object, &s as &dyn Shape));
    }

    #[test]
    fn hit_with_all_positive_times() {
        let s = sphere();
        let i1 = Intersection {
            time: 1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 2.0,
            object: &s,
        };
        let xs = Intersections(vec![i1.clone(), i2]);
        let i = xs.hit().unwrap();
        assert_eq!(i, &i1);
    }

    #[test]
    fn hit_with_some_negative_times() {
        let s = sphere();
        let i1 = Intersection {
            time: -1.0,
            object: &s,
        };
        let i2 = Intersection {
            time: 1.0,
            object: &s,
        };
        let xs = Intersections(vec![i2.clone(), i1]);
        let i = xs.hit().unwrap();
        assert_eq!(i, &i2);
    }

    #[test]
    fn hit_with_all_negative_times() {
        let s = sphere();
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
        let s = sphere();
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
        let xs = Intersections(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit().unwrap();
        assert_eq!(i, &i4);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, &shape);
        let comps = i.prepare_computations(r, &Intersections::empty());

        assert_eq!(comps.time, i.time);
        assert!(ptr::eq(comps.object, i.object));
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vector, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vector, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_an_intersection_occurs_on_the_outside() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(4.0, &shape);
        let comps = i.prepare_computations(r, &Intersections::empty());

        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = intersection(1.0, &shape);
        let comps = i.prepare_computations(r, &Intersections::empty());

        assert!(comps.inside);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vector, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vector, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = SphereBuilder::default()
            .transform(transformations::translation(0.0, 0.0, 1.0))
            .build()
            .unwrap();
        let i = intersection(5.0, &shape);
        let comps = i.prepare_computations(r, &Intersections::empty());
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let shape = plane();
        let r = ray(
            point(0.0, 1.0, -1.0),
            vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = intersection(f64::sqrt(2.0), &shape);
        let comps = i.prepare_computations(r, &Intersections::empty());
        assert_eq!(
            comps.reflect_vector,
            vector(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = glass_sphere_helper(1.5);
        a.set_transform(transformations::scaling(2.0, 2.0, 2.0));
        let mut b = glass_sphere_helper(2.0);
        b.set_transform(transformations::translation(0.0, 0.0, -0.25));
        let mut c = glass_sphere_helper(2.5);
        c.set_transform(transformations::translation(0.0, 0.0, 0.25));
        let r = ray(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let xs: Intersections = vec![
            intersection(2.0, &a),
            intersection(2.75, &b),
            intersection(3.25, &c),
            intersection(4.75, &b),
            intersection(5.25, &c),
            intersection(6.0, &a),
        ]
        .into();

        let comps = xs[0].prepare_computations(r, &xs);
        assert_eq!(comps.n1, 1.0);
        assert_eq!(comps.n2, 1.5);
        let comps = xs[1].prepare_computations(r, &xs);
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 2.0);
        let comps = xs[2].prepare_computations(r, &xs);
        assert_eq!(comps.n1, 2.0);
        assert_eq!(comps.n2, 2.5);
        let comps = xs[3].prepare_computations(r, &xs);
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 2.5);
        let comps = xs[4].prepare_computations(r, &xs);
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 1.5);
        let comps = xs[5].prepare_computations(r, &xs);
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 1.0);
    }

    #[test]
    fn under_point_is_below_the_surface() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = glass_sphere_helper(1.0);
        shape.set_transform(transformations::translation(0.0, 0.0, 1.0));
        let i = intersection(5.0, &shape);
        let xs = vec![i.clone()].into();
        let comps = i.prepare_computations(r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere_helper(1.5);
        let r = ray(point(0.0, 0.0, f64::sqrt(2.0) / 2.0), vector(0.0, 1.0, 0.0));
        let xs: Intersections = vec![
            intersection(-f64::sqrt(2.0) / 2.0, &shape),
            intersection(f64::sqrt(2.0) / 2.0, &shape),
        ]
        .into();
        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn schlick_approximation_with_perpendicular_ray() {
        let shape = glass_sphere_helper(1.5);
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs: Intersections = vec![intersection(-1.0, &shape), intersection(1.0, &shape)].into();
        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = comps.schlick();
        assert_abs_diff_eq!(reflectance, 0.04);
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = glass_sphere_helper(1.5);
        let r = ray(point(0.0, 0.99, -2.0), vector(0.0, 0.0, 1.0));
        let xs: Intersections = vec![intersection(1.8589, &shape)].into();
        let comps = xs[0].prepare_computations(r, &xs);
        let reflectance = comps.schlick();
        assert_abs_diff_eq!(reflectance, 0.48873, epsilon = EPSILON);
    }
}
