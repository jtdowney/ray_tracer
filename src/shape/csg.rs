use std::any::Any;

use ord_subset::OrdSubsetSliceExt;

use crate::{
    Intersection, Point, Ray, Vector,
    shape::{Geometry, Group, Shape},
};

#[must_use]
pub fn csg(operation: CsgOperation, left: &Shape, right: &Shape) -> Shape {
    let shape = Shape::new(Csg {
        operation,
        left: left.clone(),
        right: right.clone(),
    });
    left.set_parent(shape.downgrade());
    right.set_parent(shape.downgrade());
    shape
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsgOperation {
    Union,
    Intersection,
    Difference,
}

pub struct Csg {
    pub operation: CsgOperation,
    pub left: Shape,
    pub right: Shape,
}

impl Geometry for Csg {
    fn local_intersection(&self, _shape: &Shape, ray: Ray) -> Vec<Intersection> {
        let mut left_xs = self.left.intersect(ray);
        let mut right_xs = self.right.intersect(ray);

        let mut xs = vec![];
        xs.append(&mut left_xs);
        xs.append(&mut right_xs);
        xs.ord_subset_sort_by_key(|i| i.time);

        filter_intersections(self, &xs)
    }

    fn local_normal_at(&self, _point: Point, _hit: Option<&Intersection>) -> Vector {
        panic!("CSG shapes delegate normals to children")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[must_use]
fn intersection_allowed(op: CsgOperation, lhit: bool, inl: bool, inr: bool) -> bool {
    match op {
        CsgOperation::Union => (lhit && !inr) || (!lhit && !inl),
        CsgOperation::Intersection => (lhit && inr) || (!lhit && inl),
        CsgOperation::Difference => (lhit && !inr) || (!lhit && inl),
    }
}

fn includes(shape: &Shape, target: &Shape) -> bool {
    let inner = shape.inner();
    let geometry = &inner.geometry;

    if let Some(group) = geometry.as_any().downcast_ref::<Group>() {
        group.children().iter().any(|child| includes(child, target))
    } else if let Some(csg) = geometry.as_any().downcast_ref::<Csg>() {
        includes(&csg.left, target) || includes(&csg.right, target)
    } else {
        shape == target
    }
}

#[must_use]
fn filter_intersections(csg: &Csg, xs: &[Intersection]) -> Vec<Intersection> {
    xs.iter()
        .scan((false, false), |(inl, inr), i| {
            let lhit = includes(&csg.left, &i.object);
            let allowed = intersection_allowed(csg.operation, lhit, *inl, *inr);

            if lhit {
                *inl = !*inl;
            } else {
                *inr = !*inr;
            }

            Some(allowed.then(|| i.clone()))
        })
        .flatten()
        .collect()
}

impl Csg {
    #[must_use]
    pub fn operation(&self) -> CsgOperation {
        self.operation
    }

    #[must_use]
    pub fn left(&self) -> &Shape {
        &self.left
    }

    #[must_use]
    pub fn right(&self) -> &Shape {
        &self.right
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{
        EPSILON, intersection, point, ray,
        shape::{cube, sphere},
        transform, vector,
    };

    #[test]
    fn csg_created_with_operation_and_two_shapes() {
        let s1 = sphere().build();
        let s2 = cube().build();
        let c = csg(CsgOperation::Union, &s1, &s2);

        {
            let inner = c.inner();
            let csg_geom = inner
                .geometry
                .as_any()
                .downcast_ref::<Csg>()
                .expect("Should be a Csg");

            assert_eq!(csg_geom.operation(), CsgOperation::Union);
            assert_eq!(*csg_geom.left(), s1);
            assert_eq!(*csg_geom.right(), s2);
        }

        assert_eq!(s1.parent(), Some(c.clone()));
        assert_eq!(s2.parent(), Some(c));
    }

    #[test]
    fn evaluating_rule_for_csg_union_operation() {
        let test_cases = [
            (true, true, true, false),
            (true, true, false, true),
            (true, false, true, false),
            (true, false, false, true),
            (false, true, true, false),
            (false, true, false, false),
            (false, false, true, true),
            (false, false, false, true),
        ];

        for (lhit, inl, inr, expected) in test_cases {
            let result = intersection_allowed(CsgOperation::Union, lhit, inl, inr);
            assert_eq!(
                result, expected,
                "union: lhit={lhit}, inl={inl}, inr={inr} => expected {expected}, got {result}"
            );
        }
    }

    #[test]
    fn evaluating_rule_for_csg_intersection_operation() {
        let test_cases = [
            (true, true, true, true),
            (true, true, false, false),
            (true, false, true, true),
            (true, false, false, false),
            (false, true, true, true),
            (false, true, false, true),
            (false, false, true, false),
            (false, false, false, false),
        ];

        for (lhit, inl, inr, expected) in test_cases {
            let result = intersection_allowed(CsgOperation::Intersection, lhit, inl, inr);
            assert_eq!(
                result, expected,
                "intersection: lhit={lhit}, inl={inl}, inr={inr} => expected {expected}, got {result}"
            );
        }
    }

    #[test]
    fn evaluating_rule_for_csg_difference_operation() {
        let test_cases = [
            (true, true, true, false),
            (true, true, false, true),
            (true, false, true, false),
            (true, false, false, true),
            (false, true, true, true),
            (false, true, false, true),
            (false, false, true, false),
            (false, false, false, false),
        ];

        for (lhit, inl, inr, expected) in test_cases {
            let result = intersection_allowed(CsgOperation::Difference, lhit, inl, inr);
            assert_eq!(
                result, expected,
                "difference: lhit={lhit}, inl={inl}, inr={inr} => expected {expected}, got {result}"
            );
        }
    }

    #[test]
    fn filtering_list_of_intersections_for_union() {
        let s1 = sphere().build();
        let s2 = cube().build();
        let c = csg(CsgOperation::Union, &s1, &s2);

        let xs = [
            intersection(1, s1.clone()),
            intersection(2, s2.clone()),
            intersection(3, s1),
            intersection(4, s2),
        ];

        let inner = c.inner();
        let csg_geom = inner
            .geometry
            .as_any()
            .downcast_ref::<Csg>()
            .expect("Should be a Csg");

        let result = filter_intersections(csg_geom, &xs);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], xs[0]);
        assert_eq!(result[1], xs[3]);
    }

    #[test]
    fn filtering_list_of_intersections_for_intersection() {
        let s1 = sphere().build();
        let s2 = cube().build();
        let c = csg(CsgOperation::Intersection, &s1, &s2);

        let xs = [
            intersection(1, s1.clone()),
            intersection(2, s2.clone()),
            intersection(3, s1),
            intersection(4, s2),
        ];

        let inner = c.inner();
        let csg_geom = inner
            .geometry
            .as_any()
            .downcast_ref::<Csg>()
            .expect("Should be a Csg");

        let result = filter_intersections(csg_geom, &xs);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], xs[1]);
        assert_eq!(result[1], xs[2]);
    }

    #[test]
    fn filtering_list_of_intersections_for_difference() {
        let s1 = sphere().build();
        let s2 = cube().build();
        let c = csg(CsgOperation::Difference, &s1, &s2);

        let xs = [
            intersection(1, s1.clone()),
            intersection(2, s2.clone()),
            intersection(3, s1),
            intersection(4, s2),
        ];

        let inner = c.inner();
        let csg_geom = inner
            .geometry
            .as_any()
            .downcast_ref::<Csg>()
            .expect("Should be a Csg");

        let result = filter_intersections(csg_geom, &xs);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], xs[0]);
        assert_eq!(result[1], xs[1]);
    }

    #[test]
    fn ray_misses_csg_object() {
        let c = csg(CsgOperation::Union, &sphere().build(), &cube().build());
        let r = ray(point(0, 2, -5), vector(0, 0, 1));
        let xs = c.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_hits_csg_object() {
        let s1 = sphere().build();
        let s2 = sphere()
            .transform(transform::translation(0.0, 0.0, 0.5))
            .build();
        let c = csg(CsgOperation::Union, &s1, &s2);
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let xs = c.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].time, 4.0, epsilon = EPSILON);
        assert_eq!(xs[0].object, s1);
        assert_relative_eq!(xs[1].time, 6.5, epsilon = EPSILON);
        assert_eq!(xs[1].object, s2);
    }

    #[test]
    #[should_panic(expected = "CSG shapes delegate normals to children")]
    fn csg_local_normal_at_panics() {
        let csg_shape = Csg {
            operation: CsgOperation::Union,
            left: sphere().build(),
            right: cube().build(),
        };
        csg_shape.local_normal_at(point(0, 0, 0), None);
    }
}
