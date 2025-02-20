use std::ptr;

use ord_subset::OrdSubsetIterExt;

use crate::{EPSILON, Point, Ray, Shape, Vector};

pub fn intersection<T>(t: T, object: &Shape) -> Intersection
where
    T: Into<f64>,
{
    Intersection {
        time: t.into(),
        object,
    }
}

pub fn hit<'a, T>(iter: T) -> Option<Intersection<'a>>
where
    T: IntoIterator<Item = &'a Intersection<'a>>,
{
    iter.into_iter()
        .filter(|i| i.time >= 0.0)
        .ord_subset_min_by_key(|i| i.time)
        .copied()
}

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    pub time: f64,
    pub object: &'a Shape,
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && ptr::eq(self.object, other.object)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Computations<'a> {
    pub time: f64,
    pub object: &'a Shape,
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

impl Computations<'_> {
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

impl<'a> Intersection<'a> {
    pub fn prepare_computations(
        self,
        ray: Ray,
        intersections: &[Intersection],
    ) -> Computations<'a> {
        let time = self.time;
        let object = self.object;
        let point = ray.position(time);
        let eye_vector = -ray.direction;
        let mut normal_vector = object.normal_at(point);
        let inside = normal_vector.dot(eye_vector) < 0.0;

        if inside {
            normal_vector = -normal_vector;
        }

        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;
        let reflect_vector = ray.direction.reflect(normal_vector);

        let mut containers: Vec<&Shape> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;

        for i in intersections {
            if i == &self {
                if let Some(object) = containers.last() {
                    n1 = object.material.refractive_index;
                }
            }

            if let Some(index) = containers.iter().position(|&c| c == i.object) {
                containers.remove(index);
            } else {
                containers.push(i.object);
            }

            if i == &self {
                if let Some(object) = containers.last() {
                    n2 = object.material.refractive_index;
                }

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

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{
        EPSILON, ORIGIN, plane, point, ray, shapes::sphere::glass_sphere, sphere,
        transform::translation, vector,
    };

    use super::*;

    #[test]
    fn hit_when_all_positive_intersections() {
        let s = sphere();
        let i1 = intersection(1, &s);
        let i2 = intersection(2, &s);
        let xs = vec![i2, i1];
        assert_eq!(Some(i1), hit(&xs));
    }

    #[test]
    fn hit_when_some_negative_intersections() {
        let s = sphere();
        let i1 = intersection(-1, &s);
        let i2 = intersection(1, &s);
        let xs = vec![i2, i1];
        assert_eq!(Some(i2), hit(&xs));
    }

    #[test]
    fn hit_when_all_negative_intersections() {
        let s = sphere();
        let i1 = intersection(-2, &s);
        let i2 = intersection(-1, &s);
        let xs = vec![i2, i1];
        assert_eq!(None, hit(&xs));
    }

    #[test]
    fn hit_is_lowest_nonnegative_intersection() {
        let s = sphere();
        let i1 = intersection(5, &s);
        let i2 = intersection(7, &s);
        let i3 = intersection(-3, &s);
        let i4 = intersection(2, &s);
        let xs = vec![i1, i2, i3, i4];
        assert_eq!(Some(i4), hit(&xs));
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere();
        let i = intersection(4, &shape);
        let comps = i.prepare_computations(r, &[i]);
        assert_eq!(i.time, comps.time);
        assert_eq!(i.object, comps.object);
        assert_eq!(point(0, 0, -1), comps.point);
        assert_eq!(vector(0, 0, -1), comps.eye_vector);
        assert_eq!(vector(0, 0, -1), comps.normal_vector);
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere();
        let i = intersection(4, &shape);
        let comps = i.prepare_computations(r, &[i]);
        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = sphere();
        let i = intersection(1, &shape);
        let comps = i.prepare_computations(r, &[i]);
        assert!(comps.inside);
        assert_eq!(point(0, 0, 1), comps.point);
        assert_eq!(vector(0, 0, -1), comps.eye_vector);
        assert_eq!(vector(0, 0, -1), comps.normal_vector);
    }

    #[test]
    fn hit_should_offset_point() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut shape = sphere();
        shape.transform = translation(0, 0, 1);
        let i = intersection(5, &shape);
        let comps = i.prepare_computations(r, &[i]);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z)
    }

    #[test]
    fn precomputing_reflection_vector() {
        let shape = plane();
        let r = ray(
            point(0, 1, -1),
            vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersection(2.0_f64.sqrt(), &shape);
        let comps = i.prepare_computations(r, &[i]);
        assert_eq!(
            vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
            comps.reflect_vector
        );
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut shape = glass_sphere();
        shape.transform = translation(0, 0, 1);
        let i = intersection(5, &shape);
        let xs = vec![i];
        let comps = i.prepare_computations(r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let r = ray(point(0.0, 0.0, 2_f64.sqrt() / 2.0), vector(0, 1, 0));
        let xs = vec![
            intersection(-(2_f64.sqrt()) / 2.0, &shape),
            intersection(2_f64.sqrt() / 2.0, &shape),
        ];
        let comps = xs[1].prepare_computations(r, &xs);
        assert_eq!(1.0, comps.schlick())
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = ray(ORIGIN, vector(0, 1, 0));
        let xs = vec![intersection(-1, &shape), intersection(1, &shape)];
        let comps = xs[1].prepare_computations(r, &xs);
        assert_abs_diff_eq!(0.04, comps.schlick())
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_greater_n1() {
        let shape = glass_sphere();
        let r = ray(point(0.0, 0.99, -2.0), vector(0, 0, 1));
        let xs = vec![intersection(1.8589, &shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        assert_abs_diff_eq!(0.48873, comps.schlick(), epsilon = EPSILON)
    }
}
