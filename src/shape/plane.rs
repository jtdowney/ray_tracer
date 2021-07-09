use crate::{intersection, material, vector, Intersections, Material, Matrix4, Shape, EPSILON};
use derive_builder::Builder;

pub fn plane() -> Plane {
    PlaneBuilder::default().build().unwrap()
}

#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct Plane {
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default = "material()")]
    pub material: Material,
}

impl Shape for Plane {
    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn material(&self) -> Material {
        self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: crate::Ray) -> crate::Intersections {
        if ray.direction.y.abs() < EPSILON {
            Intersections::empty()
        } else {
            let t = -ray.origin.y / ray.direction.y;
            let intersections = vec![intersection(t, self)];
            intersections.into()
        }
    }

    fn local_normal_at(&self, _point: crate::Point) -> crate::Vector {
        vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, ray};

    #[test]
    fn normal_of_a_plane_is_constant_everywhere() {
        let p = plane();
        let n1 = p.local_normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn plane_intersect_with_parallel_ray() {
        let p = plane();
        let r = ray(point(0.0, 10.0, 10.0), vector(0.0, 0.0, 1.0));
        let mut xs = p.local_intersect(r).into_iter();
        assert!(xs.next().is_none());
    }

    #[test]
    fn plane_intersect_with_coplaner_ray() {
        let p = plane();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut xs = p.local_intersect(r).into_iter();
        assert!(xs.next().is_none());
    }

    #[test]
    fn plane_intersect_from_above() {
        let p = plane();
        let r = ray(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let mut xs = p.local_intersect(r).into_iter();
        let i = xs.next().unwrap();
        assert_eq!(i.time, 1.0);
        assert!(xs.next().is_none());
    }

    #[test]
    fn plane_intersect_from_below() {
        let p = plane();
        let r = ray(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let mut xs = p.local_intersect(r).into_iter();
        let i = xs.next().unwrap();
        assert_eq!(i.time, 1.0);
        assert_eq!(None, xs.next());
    }
}
