use crate::{Matrix4, Point, Vector3};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut output = Matrix4::identity();
    output[(0, 3)] = x;
    output[(1, 3)] = y;
    output[(2, 3)] = z;
    output
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut output = Matrix4::default();
    output[(0, 0)] = x;
    output[(1, 1)] = y;
    output[(2, 2)] = z;
    output[(3, 3)] = 1.0;
    output
}

pub fn shearing(x1: f64, x2: f64, y1: f64, y2: f64, z1: f64, z2: f64) -> Matrix4 {
    let mut output = Matrix4::identity();
    output[(0, 1)] = x1;
    output[(0, 2)] = x2;
    output[(1, 0)] = y1;
    output[(1, 2)] = y2;
    output[(2, 0)] = z1;
    output[(2, 1)] = z2;
    output
}

pub fn rotation_x(rotation: f64) -> Matrix4 {
    let (rotation_sin, rotation_cos) = rotation.sin_cos();
    let mut output = Matrix4::default();
    output[(0, 0)] = 1.0;
    output[(1, 1)] = rotation_cos;
    output[(1, 2)] = -rotation_sin;
    output[(2, 1)] = rotation_sin;
    output[(2, 2)] = rotation_cos;
    output[(3, 3)] = 1.0;
    output
}

pub fn rotation_y(rotation: f64) -> Matrix4 {
    let (rotation_sin, rotation_cos) = rotation.sin_cos();
    let mut output = Matrix4::default();
    output[(0, 0)] = rotation_cos;
    output[(0, 2)] = rotation_sin;
    output[(1, 1)] = 1.0;
    output[(2, 0)] = -rotation_sin;
    output[(2, 2)] = rotation_cos;
    output[(3, 3)] = 1.0;
    output
}

pub fn rotation_z(rotation: f64) -> Matrix4 {
    let (rotation_sin, rotation_cos) = rotation.sin_cos();
    let mut output = Matrix4::default();
    output[(0, 0)] = rotation_cos;
    output[(0, 1)] = -rotation_sin;
    output[(1, 0)] = rotation_sin;
    output[(1, 1)] = rotation_cos;
    output[(2, 2)] = 1.0;
    output[(3, 3)] = 1.0;
    output
}

pub fn view(from: Point, to: Point, up: Vector3) -> Matrix4 {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);
    let orientation = Matrix4::new(&[
        left[0],
        left[1],
        left[2],
        0.0,
        true_up[0],
        true_up[1],
        true_up[2],
        0.0,
        -forward[0],
        -forward[1],
        -forward[2],
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]);

    orientation * translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Vector3};
    use std::f64::consts::PI;

    #[test]
    fn test_multiplying_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(Point::new(2.0, 1.0, 7.0), transform * p);
    }

    #[test]
    fn test_multiplying_by_inverse_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(Point::new(-8.0, 7.0, 3.0), inv * p);
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Vector3::new(-3.0, 4.0, 5.0);
        assert_eq!(v, transform * v);
    }

    #[test]
    fn test_scaling_matrix_with_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(Point::new(-8.0, 18.0, 32.0), transform * p);
    }

    #[test]
    fn test_scaling_matrix_with_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Vector3::new(-4.0, 6.0, 8.0);
        assert_eq!(Vector3::new(-8.0, 18.0, 32.0), transform * v);
    }

    #[test]
    fn test_inverse_scaling_matrix_with() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Vector3::new(-4.0, 6.0, 8.0);
        assert_eq!(Vector3::new(-2.0, 2.0, 2.0), inv * v);
    }

    #[test]
    fn test_reflecting_with_scaling_matrix() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let v = Vector3::new(2.0, 3.0, 4.0);
        assert_eq!(Vector3::new(-2.0, 3.0, 4.0), transform * v);
    }

    #[test]
    fn test_rotating_point_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        assert_eq!(
            Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
            half_quarter * p
        );

        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(Point::new(0.0, 0.0, 1.0), full_quarter * p);
    }

    #[test]
    fn test_inverse_rotating_point_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();
        assert_eq!(
            Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0),
            inv * p
        );
    }

    #[test]
    fn test_rotating_point_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        assert_eq!(
            Point::new(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0),
            half_quarter * p
        );

        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(Point::new(1.0, 0.0, 0.0), full_quarter * p);
    }

    #[test]
    fn test_rotating_point_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        assert_eq!(
            Point::new(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0),
            half_quarter * p
        );

        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(Point::new(-1.0, 0.0, 0.0), full_quarter * p);
    }

    #[test]
    fn test_shearing_moves_x_proportional_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(Point::new(5.0, 3.0, 4.0), transform * p);
    }

    #[test]
    fn test_shearing_moves_x_proportional_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(Point::new(6.0, 3.0, 4.0), transform * p);
    }

    #[test]
    fn test_shearing_moves_y_proportional_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(Point::new(2.0, 5.0, 4.0), transform * p);
    }

    #[test]
    fn test_shearing_moves_y_proportional_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(Point::new(2.0, 7.0, 4.0), transform * p);
    }

    #[test]
    fn test_shearing_moves_z_proportional_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(Point::new(2.0, 3.0, 6.0), transform * p);
    }

    #[test]
    fn test_shearing_moves_z_proportional_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(Point::new(2.0, 3.0, 7.0), transform * p);
    }

    #[test]
    fn test_individual_transformations_applied_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(Point::new(1.0, -1.0, 0.0), p2);

        let p3 = b * p2;
        assert_eq!(Point::new(5.0, -5.0, 0.0), p3);

        let p4 = c * p3;
        assert_eq!(Point::new(15.0, 0.0, 7.0), p4);
    }

    #[test]
    fn test_chained_transformations_applied_in_reverse_order() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let transform = c * b * a;
        assert_eq!(Point::new(15.0, 0.0, 7.0), transform * p);
    }

    #[test]
    fn test_view_transform_for_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(Matrix4::identity(), view(from, to, up));
    }

    #[test]
    fn test_view_transform_looks_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(scaling(-1.0, 1.0, -1.0), view(from, to, up));
    }

    #[test]
    fn test_view_transform_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(translation(0.0, 0.0, -8.0), view(from, to, up));
    }

    #[test]
    fn test_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector3::new(1.0, 1.0, 0.0);
        assert_eq!(
            Matrix4::new(&[
                -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843,
                -0.35857, 0.59761, -0.71714, 0.0, 0.0, 0.0, 0.0, 1.0
            ]),
            view(from, to, up)
        );
    }
}
