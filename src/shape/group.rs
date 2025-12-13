use std::any::Any;

use bon::builder;
use ord_subset::OrdSubsetSliceExt;

use crate::{
    Intersection, Material, Point, Ray, Vector, identity_matrix, material,
    matrix::Matrix4,
    shape::{Geometry, Shape},
};

pub struct Group {
    pub children: Vec<Shape>,
}

impl Geometry for Group {
    fn local_intersection(&self, _shape: &Shape, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .children
            .iter()
            .flat_map(|child| child.intersect(ray))
            .collect();
        intersections.ord_subset_sort_by_key(|i| i.time);
        intersections
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        panic!("Groups do not have surface normals")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[builder(finish_fn = build)]
#[must_use]
pub fn group(
    #[builder(default = identity_matrix())] transform: Matrix4,
    #[builder(default = material(), into)] material: Material,
) -> Shape {
    let shape = Shape::new(Group { children: vec![] });
    shape.set_transform(transform);
    shape.set_material(material);
    shape
}

impl Group {
    #[must_use]
    pub fn children(&self) -> &[Shape] {
        &self.children
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, point, ray, shape::sphere, transform, vector};

    #[test]
    fn creating_new_group() {
        let g = group().build();
        assert_eq!(g.inner().transform, identity_matrix());
        let inner = g.inner();
        let group_geom = inner
            .geometry
            .as_any()
            .downcast_ref::<Group>()
            .expect("Should be a Group");
        assert!(group_geom.is_empty());
    }

    #[test]
    fn adding_child_to_group() {
        let g = group().build();
        let s = sphere().build();
        let s_clone = s.clone();
        g.add_child(s);

        let inner = g.inner();
        let group_geom = inner
            .geometry
            .as_any()
            .downcast_ref::<Group>()
            .expect("Should be a Group");
        assert!(!group_geom.is_empty());
        assert!(group_geom.children().contains(&s_clone));
        assert_eq!(s_clone.parent(), Some(g.clone()));
    }

    #[test]
    fn intersecting_ray_with_empty_group() {
        let g = group().build();
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let xs = g.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_ray_with_nonempty_group() {
        let g = group().build();
        let s1 = sphere().build();
        let s2 = sphere().transform(transform::translation(0, 0, -3)).build();
        let s3 = sphere().transform(transform::translation(5, 0, 0)).build();

        let s1_clone = s1.clone();
        let s2_clone = s2.clone();

        g.add_child(s1);
        g.add_child(s2);
        g.add_child(s3);

        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let xs = g.intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, s2_clone);
        assert_eq!(xs[1].object, s2_clone);
        assert_eq!(xs[2].object, s1_clone);
        assert_eq!(xs[3].object, s1_clone);
    }

    #[test]
    fn intersecting_transformed_group() {
        let g = group().transform(transform::scaling(2, 2, 2)).build();
        let s = sphere().transform(transform::translation(5, 0, 0)).build();
        g.add_child(s);

        let r = ray(point(10, 0, -10), vector(0, 0, 1));
        let xs = g.intersect(r);

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn converting_point_from_world_to_object_space() {
        let g1 = group().transform(transform::rotation_y(FRAC_PI_2)).build();
        let g2 = group().transform(transform::scaling(2, 2, 2)).build();
        g1.add_child(g2.clone());

        let s = sphere().transform(transform::translation(5, 0, 0)).build();
        g2.add_child(s.clone());

        let p = s.world_to_object(point(-2, 0, -10));
        assert_relative_eq!(p.x(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(p.y(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(p.z(), -1.0, epsilon = EPSILON);
    }

    #[test]
    fn converting_normal_from_object_to_world_space() {
        let g1 = group().transform(transform::rotation_y(FRAC_PI_2)).build();
        let g2 = group().transform(transform::scaling(1, 2, 3)).build();
        g1.add_child(g2.clone());

        let s = sphere().transform(transform::translation(5, 0, 0)).build();
        g2.add_child(s.clone());

        let sqrt3_over_3 = 3.0_f32.sqrt() / 3.0;
        let n = s.normal_to_world(vector(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3));
        assert_relative_eq!(n.x(), 0.2857, epsilon = EPSILON);
        assert_relative_eq!(n.y(), 0.4286, epsilon = EPSILON);
        assert_relative_eq!(n.z(), -0.8571, epsilon = EPSILON);
    }

    #[test]
    fn finding_normal_on_child_object() {
        let g1 = group().transform(transform::rotation_y(FRAC_PI_2)).build();
        let g2 = group().transform(transform::scaling(1, 2, 3)).build();
        g1.add_child(g2.clone());

        let s = sphere().transform(transform::translation(5, 0, 0)).build();
        g2.add_child(s.clone());

        let n = s.normal_at(point(1.7321, 1.1547, -5.5774));
        assert_relative_eq!(n.x(), 0.2857, epsilon = EPSILON);
        assert_relative_eq!(n.y(), 0.4286, epsilon = EPSILON);
        assert_relative_eq!(n.z(), -0.8571, epsilon = EPSILON);
    }

    #[test]
    #[should_panic(expected = "Groups do not have surface normals")]
    fn group_local_normal_at_panics() {
        let group = Group { children: vec![] };
        group.local_normal_at(point(0, 0, 0));
    }
}
