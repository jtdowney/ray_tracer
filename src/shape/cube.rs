use crate::{
    intersection, material, point, vector, Intersections, Material, Matrix4, Ray, Shape, Vector,
    EPSILON,
};
use derive_builder::Builder;
use ord_subset::OrdSubsetIterExt;
use point::Point;

pub fn cube() -> Cube {
    CubeBuilder::default().build().unwrap()
}

#[derive(Builder, Clone)]
pub struct Cube {
    #[builder(default)]
    id: usize,
    #[builder(default = "Matrix4::identity()")]
    pub transform: Matrix4,
    #[builder(default = "material()")]
    pub material: Material,
}

impl Shape for Cube {
    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: Ray) -> Intersections {
        let xt = Self::check_axis(ray.origin.x, ray.direction.x);
        let yt = Self::check_axis(ray.origin.y, ray.direction.y);
        let zt = Self::check_axis(ray.origin.z, ray.direction.z);

        let tmin = &[xt, yt, zt]
            .iter()
            .flat_map(|&n| n.map(|(min, _)| min))
            .ord_subset_max()
            .unwrap();
        let tmax = &[xt, yt, zt]
            .iter()
            .flat_map(|&n| n.map(|(_, max)| max))
            .ord_subset_min()
            .unwrap();

        if tmin > tmax {
            Intersections::empty()
        } else {
            vec![intersection(*tmin, self), intersection(*tmax, self)].into()
        }
    }

    fn local_normal_at(&self, Point { x, y, z }: Point) -> Vector {
        let max = [x, y, z].iter().map(|n| n.abs()).ord_subset_max().unwrap();
        if max == x.abs() {
            vector(x, 0.0, 0.0)
        } else if max == y.abs() {
            vector(0.0, y, 0.0)
        } else {
            vector(0.0, 0.0, z)
        }
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
    use crate::{ray, vector};

    #[test]
    fn ray_intersects_cube() {
        let c = cube();

        let r = ray(point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = ray(point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = ray(point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = ray(point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = ray(point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = ray(point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(4.0, xs.next().unwrap().time);
        assert_eq!(6.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());

        let r = ray(point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(-1.0, xs.next().unwrap().time);
        assert_eq!(1.0, xs.next().unwrap().time);
        assert_eq!(None, xs.next());
    }

    #[test]
    fn ray_misses_cube() {
        let c = cube();

        let r = ray(point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(None, xs.next());

        let r = ray(point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(None, xs.next());

        let r = ray(point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673));
        let mut xs = c.local_intersect(r).into_iter();
        assert_eq!(None, xs.next());
    }

    #[test]
    fn normal_on_surface_of_cube() {
        let c = cube();

        let p = point(1.0, 0.5, -0.8);
        assert_eq!(vector(1.0, 0.0, 0.0), c.local_normal_at(p));

        let p = point(-1.0, -0.2, 0.9);
        assert_eq!(vector(-1.0, 0.0, 0.0), c.local_normal_at(p));

        let p = point(-0.4, 1.0, -0.1);
        assert_eq!(vector(0.0, 1.0, 0.0), c.local_normal_at(p));

        let p = point(0.3, -1.0, -0.7);
        assert_eq!(vector(0.0, -1.0, 0.0), c.local_normal_at(p));

        let p = point(-0.6, 0.3, 1.0);
        assert_eq!(vector(0.0, 0.0, 1.0), c.local_normal_at(p));

        let p = point(0.4, 0.4, -1.0);
        assert_eq!(vector(0.0, 0.0, -1.0), c.local_normal_at(p));

        let p = point(1.0, 1.0, 1.0);
        assert_eq!(vector(1.0, 0.0, 0.0), c.local_normal_at(p));

        let p = point(-1.0, -1.0, -1.0);
        assert_eq!(vector(-1.0, 0.0, 0.0), c.local_normal_at(p));
    }
}
