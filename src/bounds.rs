use crate::{Matrix4, Point, Ray, EPSILON};
use std::f64::{INFINITY, NEG_INFINITY};
use std::iter::Sum;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bounds {
    minimum: Point,
    maximum: Point,
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            minimum: Point::new(INFINITY, INFINITY, INFINITY),
            maximum: Point::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
        }
    }
}

impl Add<Bounds> for Bounds {
    type Output = Bounds;

    fn add(self, other: Bounds) -> Self::Output {
        self + other.minimum + other.maximum
    }
}

impl Add<Point> for Bounds {
    type Output = Bounds;

    fn add(self, other: Point) -> Self::Output {
        Bounds {
            minimum: Point::new(
                self.minimum.x.min(other.x),
                self.minimum.y.min(other.y),
                self.minimum.z.min(other.z),
            ),
            maximum: Point::new(
                self.maximum.x.max(other.x),
                self.maximum.y.max(other.y),
                self.maximum.z.max(other.z),
            ),
        }
    }
}

impl Mul<Matrix4> for Bounds {
    type Output = Bounds;

    fn mul(self, other: Matrix4) -> Self::Output {
        [
            Point::new(self.maximum.x, self.maximum.y, self.minimum.z),
            Point::new(self.maximum.x, self.minimum.y, self.maximum.z),
            Point::new(self.maximum.x, self.minimum.y, self.minimum.z),
            Point::new(self.minimum.x, self.maximum.y, self.maximum.z),
            Point::new(self.minimum.x, self.maximum.y, self.minimum.z),
            Point::new(self.minimum.x, self.minimum.y, self.maximum.z),
            self.maximum,
            self.minimum,
        ]
        .iter()
        .cloned()
        .map(|p| other * p)
        .sum()
    }
}

impl Sum<Bounds> for Bounds {
    fn sum<I: Iterator<Item = Bounds>>(iter: I) -> Self {
        iter.fold(Bounds::default(), |bounds, b| bounds + b)
    }
}

impl Sum<Point> for Bounds {
    fn sum<I: Iterator<Item = Point>>(iter: I) -> Self {
        iter.fold(Bounds::default(), |bounds, point| bounds + point)
    }
}

impl Bounds {
    pub fn intersect(&self, ray: Ray) -> bool {
        let xt = Self::check_axis(
            ray.origin.x,
            ray.direction[0],
            self.minimum.x,
            self.maximum.x,
        );
        let yt = Self::check_axis(
            ray.origin.y,
            ray.direction[1],
            self.minimum.y,
            self.maximum.y,
        );
        let zt = Self::check_axis(
            ray.origin.z,
            ray.direction[2],
            self.minimum.z,
            self.maximum.z,
        );

        let tmin = &[xt, yt, zt]
            .iter()
            .filter_map(|&n| n)
            .map(|(min, _)| min)
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();
        let tmax = &[xt, yt, zt]
            .iter()
            .filter_map(|&n| n)
            .map(|(_, max)| max)
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();

        tmin < tmax
    }

    fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> Option<(f64, f64)> {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

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
    use crate::{transforms, Vector3};

    #[test]
    fn adding_point() {
        let bounds = Bounds::default();
        let point = Point::default();
        assert_eq!(
            Bounds {
                minimum: point,
                maximum: point
            },
            bounds + point
        );
    }

    #[test]
    fn adding_bounds() {
        let bounds = Bounds::default();
        let point = Point::default();
        let other = bounds + point;
        assert_eq!(
            Bounds {
                minimum: point,
                maximum: point
            },
            bounds + other
        );
    }

    #[test]
    fn transforming_bounds() {
        let bounds = Bounds::default() + Point::default() + Point::new(1.0, 1.0, 1.0);
        assert_eq!(
            Bounds {
                minimum: Point::new(1.0, 2.0, 3.0),
                maximum: Point::new(2.0, 3.0, 4.0),
            },
            bounds * transforms::translation(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn ray_intersecting_bounds() {
        let b = Bounds::default() + Point::new(-1.0, -1.0, -1.0) + Point::new(1.0, 1.0, 1.0);
        let r = Ray::new(Point::new(5.0, 0.5, 0.0), Vector3::new(-1.0, 0.0, 0.0));
        assert!(b.intersect(r));

        let r = Ray::new(Point::new(-5.0, 0.5, 0.0), Vector3::new(1.0, 0.0, 0.0));
        assert!(b.intersect(r));

        let r = Ray::new(Point::new(0.5, 5.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        assert!(b.intersect(r));

        let r = Ray::new(Point::new(0.5, -5.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        assert!(b.intersect(r));

        let r = Ray::new(Point::new(0.5, 0.0, 5.0), Vector3::new(0.0, 0.0, -1.0));
        assert!(b.intersect(r));

        let r = Ray::new(Point::new(0.5, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        assert!(b.intersect(r));

        let r = Ray::new(Point::new(0.0, 0.5, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert!(b.intersect(r));
    }

    #[test]
    fn ray_misses_bounds() {
        let b = Bounds::default() + Point::new(-1.0, -1.0, -1.0) + Point::new(1.0, 1.0, 1.0);
        let r = Ray::new(
            Point::new(-2.0, 0.0, 0.0),
            Vector3::new(0.2673, 0.5345, 0.8018),
        );
        assert!(!b.intersect(r));

        let r = Ray::new(
            Point::new(0.0, -2.0, 0.0),
            Vector3::new(0.8018, 0.2673, 0.5345),
        );
        assert!(!b.intersect(r));

        let r = Ray::new(
            Point::new(0.0, 0.0, -2.0),
            Vector3::new(0.5345, 0.8018, 0.2673),
        );
        assert!(!b.intersect(r));
    }
}
