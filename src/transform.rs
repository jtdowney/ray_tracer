use num_traits::AsPrimitive;

use crate::{Matrix4, Point, Vector, identity_matrix, matrix};

#[must_use]
pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix4 {
    let forward = (to - from).normalize();
    let up = up.normalize();
    let left = forward.cross(&up);
    let true_up = left.cross(&forward);

    let orientation = matrix([
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    orientation * translation(-from.x, -from.y, -from.z)
}

pub fn translation(
    x: impl AsPrimitive<f32>,
    y: impl AsPrimitive<f32>,
    z: impl AsPrimitive<f32>,
) -> Matrix4 {
    let mut transform = identity_matrix();
    transform[(0, 3)] = x.as_();
    transform[(1, 3)] = y.as_();
    transform[(2, 3)] = z.as_();
    transform
}

pub fn scaling(
    x: impl AsPrimitive<f32>,
    y: impl AsPrimitive<f32>,
    z: impl AsPrimitive<f32>,
) -> Matrix4 {
    let mut transform = identity_matrix();
    transform[(0, 0)] = x.as_();
    transform[(1, 1)] = y.as_();
    transform[(2, 2)] = z.as_();
    transform
}

pub fn rotation_x(radians: impl AsPrimitive<f32>) -> Matrix4 {
    let mut transform = identity_matrix();
    let (sin, cos) = radians.as_().sin_cos();
    transform[(1, 1)] = cos;
    transform[(1, 2)] = -sin;
    transform[(2, 1)] = sin;
    transform[(2, 2)] = cos;
    transform
}

pub fn rotation_y(radians: impl AsPrimitive<f32>) -> Matrix4 {
    let mut transform = identity_matrix();
    let (sin, cos) = radians.as_().sin_cos();
    transform[(0, 0)] = cos;
    transform[(0, 2)] = sin;
    transform[(2, 0)] = -sin;
    transform[(2, 2)] = cos;
    transform
}

pub fn rotation_z(radians: impl AsPrimitive<f32>) -> Matrix4 {
    let mut transform = identity_matrix();
    let (sin, cos) = radians.as_().sin_cos();
    transform[(0, 0)] = cos;
    transform[(0, 1)] = -sin;
    transform[(1, 0)] = sin;
    transform[(1, 1)] = cos;
    transform
}

pub fn shearing(
    xy: impl AsPrimitive<f32>,
    xz: impl AsPrimitive<f32>,
    yx: impl AsPrimitive<f32>,
    yz: impl AsPrimitive<f32>,
    zx: impl AsPrimitive<f32>,
    zy: impl AsPrimitive<f32>,
) -> Matrix4 {
    let mut transform = identity_matrix();
    transform[(0, 1)] = xy.as_();
    transform[(0, 2)] = xz.as_();
    transform[(1, 0)] = yx.as_();
    transform[(1, 2)] = yz.as_();
    transform[(2, 0)] = zx.as_();
    transform[(2, 1)] = zy.as_();
    transform
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, point, vector};

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = translation(5, -3, 2);
        let p = point(-3, 4, 5);
        assert_eq!(transform * p, point(2, 1, 7));
    }

    #[test]
    fn multiplying_by_inverse_of_translation_matrix() {
        let transform = translation(5, -3, 2);
        let inv = transform.inverse().unwrap();
        let p = point(-3, 4, 5);
        assert_eq!(inv * p, point(-8, 7, 3));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5, -3, 2);
        let v = vector(-3, 4, 5);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2, 3, 4);
        let p = point(-4, 6, 8);
        assert_eq!(transform * p, point(-8, 18, 32));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2, 3, 4);
        let v = vector(-4, 6, 8);
        assert_eq!(transform * v, vector(-8, 18, 32));
    }

    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = scaling(2, 3, 4);
        let inv = transform.inverse().unwrap();
        let v = vector(-4, 6, 8);
        assert_eq!(inv * v, vector(-2, 2, 2));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1, 1, 1);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(-2, 3, 4));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        let sqrt2_over_2 = 2.0_f32.sqrt() / 2.0;
        let result = half_quarter * p;
        assert_relative_eq!(result.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(result.y, sqrt2_over_2, epsilon = EPSILON);
        assert_relative_eq!(result.z, sqrt2_over_2, epsilon = EPSILON);

        let result = full_quarter * p;
        assert_relative_eq!(result.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(result.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(result.z, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn inverse_of_x_rotation_rotates_opposite_direction() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        let sqrt2_over_2 = 2.0_f32.sqrt() / 2.0;
        let result = inv * p;
        assert_relative_eq!(result.x, 0.0, epsilon = EPSILON);
        assert_relative_eq!(result.y, sqrt2_over_2, epsilon = EPSILON);
        assert_relative_eq!(result.z, -sqrt2_over_2, epsilon = EPSILON);
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0, 0, 1);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        let sqrt2_over_2 = 2.0_f32.sqrt() / 2.0;
        let result = half_quarter * p;
        assert_relative_eq!(result.x, sqrt2_over_2, epsilon = EPSILON);
        assert_relative_eq!(result.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(result.z, sqrt2_over_2, epsilon = EPSILON);

        let result = full_quarter * p;
        assert_relative_eq!(result.x, 1.0, epsilon = EPSILON);
        assert_relative_eq!(result.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(result.z, 0.0, epsilon = EPSILON);
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        let sqrt2_over_2 = 2.0_f32.sqrt() / 2.0;
        let result = half_quarter * p;
        assert_relative_eq!(result.x, -sqrt2_over_2, epsilon = EPSILON);
        assert_relative_eq!(result.y, sqrt2_over_2, epsilon = EPSILON);
        assert_relative_eq!(result.z, 0.0, epsilon = EPSILON);

        let result = full_quarter * p;
        assert_relative_eq!(result.x, -1.0, epsilon = EPSILON);
        assert_relative_eq!(result.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(result.z, 0.0, epsilon = EPSILON);
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let transform = shearing(1, 0, 0, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(5, 3, 4));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let transform = shearing(0, 1, 0, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(6, 3, 4));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let transform = shearing(0, 0, 1, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 5, 4));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let transform = shearing(0, 0, 0, 1, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 7, 4));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let transform = shearing(0, 0, 0, 0, 1, 0);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 3, 6));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let transform = shearing(0, 0, 0, 0, 0, 1);
        let p = point(2, 3, 4);
        assert_eq!(transform * p, point(2, 3, 7));
    }

    #[test]
    fn individual_transformations_applied_in_sequence() {
        let p = point(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5, 5, 5);
        let c = translation(10, 5, 7);

        let p2 = a * p;
        assert_relative_eq!(p2.x, 1.0, epsilon = EPSILON);
        assert_relative_eq!(p2.y, -1.0, epsilon = EPSILON);
        assert_relative_eq!(p2.z, 0.0, epsilon = EPSILON);

        let p3 = b * p2;
        assert_relative_eq!(p3.x, 5.0, epsilon = EPSILON);
        assert_relative_eq!(p3.y, -5.0, epsilon = EPSILON);
        assert_relative_eq!(p3.z, 0.0, epsilon = EPSILON);

        let p4 = c * p3;
        assert_relative_eq!(p4.x, 15.0, epsilon = EPSILON);
        assert_relative_eq!(p4.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(p4.z, 7.0, epsilon = EPSILON);
    }

    #[test]
    fn chained_transformations_applied_in_reverse_order() {
        let p = point(1, 0, 1);
        let rotation = rotation_x(PI / 2.0);
        let scale = scaling(5, 5, 5);
        let translate = translation(10, 5, 7);

        let transform = translate * scale * rotation;
        let result = transform * p;
        assert_relative_eq!(result.x, 15.0, epsilon = EPSILON);
        assert_relative_eq!(result.y, 0.0, epsilon = EPSILON);
        assert_relative_eq!(result.z, 7.0, epsilon = EPSILON);
    }

    #[test]
    fn view_transformation_for_default_orientation() {
        let from = point(0, 0, 0);
        let to = point(0, 0, -1);
        let up = vector(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(t, crate::identity_matrix());
    }

    #[test]
    fn view_transformation_looking_in_positive_z() {
        let from = point(0, 0, 0);
        let to = point(0, 0, 1);
        let up = vector(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(t, scaling(-1, 1, -1));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = point(0, 0, 8);
        let to = point(0, 0, 0);
        let up = vector(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(t, translation(0, 0, -8));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = point(1, 3, 2);
        let to = point(4, -2, 8);
        let up = vector(1, 1, 0);
        let t = view_transform(from, to, up);

        assert_relative_eq!(t[(0, 0)], -0.50709, epsilon = EPSILON);
        assert_relative_eq!(t[(0, 1)], 0.50709, epsilon = EPSILON);
        assert_relative_eq!(t[(0, 2)], 0.67612, epsilon = EPSILON);
        assert_relative_eq!(t[(0, 3)], -2.36643, epsilon = EPSILON);

        assert_relative_eq!(t[(1, 0)], 0.76772, epsilon = EPSILON);
        assert_relative_eq!(t[(1, 1)], 0.60609, epsilon = EPSILON);
        assert_relative_eq!(t[(1, 2)], 0.12122, epsilon = EPSILON);
        assert_relative_eq!(t[(1, 3)], -2.82843, epsilon = EPSILON);

        assert_relative_eq!(t[(2, 0)], -0.35857, epsilon = EPSILON);
        assert_relative_eq!(t[(2, 1)], 0.59761, epsilon = EPSILON);
        assert_relative_eq!(t[(2, 2)], -0.71714, epsilon = EPSILON);
        assert_relative_eq!(t[(2, 3)], 0.00000, epsilon = EPSILON);

        assert_relative_eq!(t[(3, 0)], 0.00000, epsilon = EPSILON);
        assert_relative_eq!(t[(3, 1)], 0.00000, epsilon = EPSILON);
        assert_relative_eq!(t[(3, 2)], 0.00000, epsilon = EPSILON);
        assert_relative_eq!(t[(3, 3)], 1.00000, epsilon = EPSILON);
    }
}
