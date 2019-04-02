use crate::{Matrix4, Point, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> Point {
        self.origin + self.direction * time
    }

    pub fn transform(self, transform: Matrix4) -> Ray {
        Ray {
            origin: transform * self.origin,
            direction: transform * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transforms;

    #[test]
    fn creating_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(Point::new(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn translating_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector3::new(0.0, 1.0, 0.0));
        let m = transforms::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(Point::new(4.0, 6.0, 8.0), r2.origin);
        assert_eq!(Vector3::new(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scaling_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector3::new(0.0, 1.0, 0.0));
        let m = transforms::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(Point::new(2.0, 6.0, 12.0), r2.origin);
        assert_eq!(Vector3::new(0.0, 3.0, 0.0), r2.direction);
    }
}
