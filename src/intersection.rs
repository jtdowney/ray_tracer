use approx::relative_eq;
use ord_subset::OrdSubsetIterExt;

use crate::{EPSILON, Point, Ray, Vector, shape::Shape};

pub fn intersection<T>(t: T, object: Shape) -> Intersection
where
    T: Into<f64>,
{
    Intersection {
        time: t.into(),
        object,
    }
}

pub fn hit<T>(iter: T) -> Option<Intersection>
where
    T: IntoIterator<Item = Intersection>,
{
    iter.into_iter()
        .filter(|i| i.time >= 0.0)
        .ord_subset_min_by_key(|i| i.time)
}

#[derive(Clone)]
pub struct Intersection {
    pub time: f64,
    pub object: Shape,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.time, other.time, epsilon = EPSILON) && self.object == other.object
    }
}

impl Intersection {
    #[must_use]
    pub fn prepare_computations(&self, ray: Ray, xs: &[Intersection]) -> Computations {
        let point = ray.position(self.time);
        let eyev = -ray.direction;
        let normalv = self.object.normal_at(point);
        let inside = normalv.dot(&eyev) < 0.0;
        let normalv = if inside { -normalv } else { normalv };
        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        let reflectv = ray.direction.reflect(&normalv);

        let (_, n1, n2) = xs.iter().fold(
            (Vec::<Shape>::new(), 1.0, 1.0),
            |(mut containers, mut n1, mut n2), intersection| {
                let is_hit = intersection.object == self.object
                    && relative_eq!(intersection.time, self.time, epsilon = EPSILON);
                if is_hit {
                    n1 = containers
                        .last()
                        .map_or(1.0, |obj| obj.inner().material.refractive_index);
                }

                if let Some(pos) = containers
                    .iter()
                    .position(|obj| *obj == intersection.object)
                {
                    containers.remove(pos);
                } else {
                    containers.push(intersection.object.clone());
                }

                if is_hit {
                    n2 = containers
                        .last()
                        .map_or(1.0, |obj| obj.inner().material.refractive_index);
                }

                (containers, n1, n2)
            },
        );

        Computations {
            time: self.time,
            object: self.object.clone(),
            point,
            over_point,
            under_point,
            eyev,
            normalv,
            reflectv,
            n1,
            n2,
            inside,
        }
    }
}

pub struct Computations {
    pub time: f64,
    pub object: Shape,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub reflectv: Vector,
    pub n1: f64,
    pub n2: f64,
    pub inside: bool,
}

#[must_use]
pub fn schlick(comps: &Computations) -> f64 {
    let mut cos = comps.eyev.dot(&comps.normalv);

    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
        if sin2_t > 1.0 {
            return 1.0;
        }
        cos = (1.0 - sin2_t).sqrt();
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[cfg(test)]
mod tests {
    use std::slice;

    use approx::assert_relative_eq;

    use super::*;
    use crate::{
        EPSILON, Material, point, ray,
        shape::{glass_sphere, plane, sphere},
        transform, vector,
    };

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = sphere().build();
        let i = intersection(3.5, s.clone());
        assert_relative_eq!(i.time, 3.5, epsilon = EPSILON);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = sphere().build();
        let i1 = intersection(1, s.clone());
        let i2 = intersection(2, s);
        let xs = [i1, i2];
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 1.0, epsilon = EPSILON);
        assert_relative_eq!(xs[1].time, 2.0, epsilon = EPSILON);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = sphere().build();
        let i1 = intersection(1, s.clone());
        let i2 = intersection(2, s);
        let xs = vec![i2, i1.clone()];
        let i = hit(xs).unwrap();
        assert_relative_eq!(i.time, i1.time, epsilon = EPSILON);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = sphere().build();
        let i1 = intersection(-1, s.clone());
        let i2 = intersection(1, s);
        let xs = vec![i2.clone(), i1];
        let i = hit(xs).unwrap();
        assert_relative_eq!(i.time, i2.time, epsilon = EPSILON);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = sphere().build();
        let i1 = intersection(-2, s.clone());
        let i2 = intersection(-1, s);
        let xs = vec![i2, i1];
        let i = hit(xs);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = sphere().build();
        let i1 = intersection(5, s.clone());
        let i2 = intersection(7, s.clone());
        let i3 = intersection(-3, s.clone());
        let i4 = intersection(2, s);
        let xs = vec![i1, i2, i3, i4.clone()];
        let i = hit(xs).unwrap();
        assert_relative_eq!(i.time, i4.time, epsilon = EPSILON);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere().build();
        let i = intersection(4, shape.clone());
        let comps = i.prepare_computations(r, slice::from_ref(&i));
        assert_relative_eq!(comps.time, i.time, epsilon = EPSILON);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0, 0, -1));
        assert_eq!(comps.eyev, vector(0, 0, -1));
        assert_eq!(comps.normalv, vector(0, 0, -1));
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere().build();
        let i = intersection(4, shape);
        let comps = i.prepare_computations(r, slice::from_ref(&i));
        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = sphere().build();
        let i = intersection(1, shape);
        let comps = i.prepare_computations(r, slice::from_ref(&i));
        assert_eq!(comps.point, point(0, 0, 1));
        assert_eq!(comps.eyev, vector(0, 0, -1));
        assert!(comps.inside);
        assert_eq!(comps.normalv, vector(0, 0, -1));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere().transform(transform::translation(0, 0, 1)).build();
        let i = intersection(5, shape);
        let comps = i.prepare_computations(r, slice::from_ref(&i));
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let shape = plane().build();
        let r = ray(point(0, 1, -1), vector(0.0, -sqrt2_over_2, sqrt2_over_2));
        let i = intersection(2.0_f64.sqrt(), shape);
        let comps = i.prepare_computations(r, slice::from_ref(&i));
        assert_relative_eq!(comps.reflectv.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(comps.reflectv.y, sqrt2_over_2, epsilon = EPSILON);
        assert_relative_eq!(comps.reflectv.z, sqrt2_over_2, epsilon = EPSILON);
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = sphere()
            .transform(transform::scaling(2, 2, 2))
            .material(Material::builder().transparency(1.0).refractive_index(1.5))
            .build();
        let b = sphere()
            .transform(transform::translation(0.0, 0.0, -0.25))
            .material(Material::builder().transparency(1.0).refractive_index(2.0))
            .build();
        let c = sphere()
            .transform(transform::translation(0.0, 0.0, 0.25))
            .material(Material::builder().transparency(1.0).refractive_index(2.5))
            .build();
        let r = ray(point(0, 0, -4), vector(0, 0, 1));
        let xs = [
            intersection(2, a.clone()),
            intersection(2.75, b.clone()),
            intersection(3.25, c.clone()),
            intersection(4.75, b),
            intersection(5.25, c),
            intersection(6, a),
        ];

        let test_cases = [
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];

        for (index, expected_n1, expected_n2) in test_cases {
            let comps = xs[index].prepare_computations(r, &xs);
            assert_relative_eq!(comps.n1, expected_n1, epsilon = EPSILON);
            assert_relative_eq!(comps.n2, expected_n2, epsilon = EPSILON);
        }
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = sphere()
            .transform(transform::translation(0, 0, 1))
            .material(Material::builder().transparency(1.0).refractive_index(1.5))
            .build();
        let i = intersection(5, shape);
        let xs = [i.clone()];
        let comps = i.prepare_computations(r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let shape = glass_sphere();
        let r = ray(point(0.0, 0.0, sqrt2_over_2), vector(0, 1, 0));
        let xs = [
            intersection(-sqrt2_over_2, shape.clone()),
            intersection(sqrt2_over_2, shape),
        ];
        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = schlick(&comps);
        assert_relative_eq!(reflectance, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = ray(point(0, 0, 0), vector(0, 1, 0));
        let xs = [intersection(-1, shape.clone()), intersection(1, shape)];
        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = schlick(&comps);
        assert_relative_eq!(reflectance, 0.04, epsilon = EPSILON);
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = glass_sphere();
        let r = ray(point(0.0, 0.99, -2.0), vector(0, 0, 1));
        let xs = [intersection(1.8589, shape)];
        let comps = xs[0].prepare_computations(r, &xs);
        let reflectance = schlick(&comps);
        assert_relative_eq!(reflectance, 0.48873, epsilon = EPSILON);
    }
}
