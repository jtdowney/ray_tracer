use crate::{identity_matrix, Matrix4};

pub fn translation<T: Into<f64>>(x: T, y: T, z: T) -> Matrix4 {
    let mut transform = identity_matrix();
    transform[(0, 3)] = x.into();
    transform[(1, 3)] = y.into();
    transform[(2, 3)] = z.into();
    transform
}

pub fn scaling<T: Into<f64>>(x: T, y: T, z: T) -> Matrix4 {
    let mut transform = identity_matrix();
    transform[(0, 0)] = x.into();
    transform[(1, 1)] = y.into();
    transform[(2, 2)] = z.into();
    transform
}

pub fn rotation_x<T: Into<f64>>(theta: T) -> Matrix4 {
    let (theta_sin, theta_cos) = theta.into().sin_cos();
    let mut transform = identity_matrix();
    transform[(1, 1)] = theta_cos;
    transform[(1, 2)] = -theta_sin;
    transform[(2, 1)] = theta_sin;
    transform[(2, 2)] = theta_cos;

    transform
}

pub fn rotation_y<T: Into<f64>>(theta: T) -> Matrix4 {
    let (theta_sin, theta_cos) = theta.into().sin_cos();
    let mut transform = identity_matrix();
    transform[(0, 0)] = theta_cos;
    transform[(0, 2)] = theta_sin;
    transform[(2, 0)] = -theta_sin;
    transform[(2, 2)] = theta_cos;

    transform
}

pub fn rotation_z<T: Into<f64>>(theta: T) -> Matrix4 {
    let (theta_sin, theta_cos) = theta.into().sin_cos();
    let mut transform = identity_matrix();
    transform[(0, 0)] = theta_cos;
    transform[(0, 1)] = -theta_sin;
    transform[(1, 0)] = theta_sin;
    transform[(1, 1)] = theta_cos;

    transform
}

pub fn shearing<T: Into<f64>>(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Matrix4 {
    let mut transform = identity_matrix();
    transform[(0, 1)] = xy.into();
    transform[(0, 2)] = xz.into();
    transform[(1, 0)] = yx.into();
    transform[(1, 2)] = yz.into();
    transform[(2, 0)] = zx.into();
    transform[(2, 1)] = zy.into();
    transform
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use crate::{point, vector};

    use super::*;

    #[test]
    fn translating_point() {
        let transform = translation(5, -3, 2);
        let p = point(-3, 4, 5);
        assert_eq!(point(2, 1, 7), transform * p);
    }

    #[test]
    fn inverse_translating_point() {
        let transform = translation(5, -3, 2);
        let inv = transform.inverse();
        let p = point(-3, 4, 5);
        assert_eq!(point(-8, 7, 3), inv * p);
    }

    #[test]
    fn translation_does_not_change_vectors() {
        let transform = translation(5, -3, 2);
        let v = vector(-3, 4, 5);
        assert_eq!(v, transform * v);
    }

    #[test]
    fn scaling_point() {
        let transform = scaling(2, 3, 4);
        let p = point(-4, 6, 8);
        assert_eq!(point(-8, 18, 32), transform * p);
    }

    #[test]
    fn scaling_vector() {
        let transform = scaling(2, 3, 4);
        let v = vector(-4, 6, 8);
        assert_eq!(vector(-8, 18, 32), transform * v);
    }

    #[test]
    fn inverse_scaling_vector() {
        let transform = scaling(2, 3, 4);
        let inv = transform.inverse();
        let v = vector(-4, 6, 8);
        assert_eq!(vector(-2, 2, 2), inv * v);
    }

    #[test]
    fn reflecting_point() {
        let transform = scaling(-1, 1, 1);
        let p = point(2, 3, 4);
        assert_eq!(point(-2, 3, 4), transform * p);
    }

    #[test]
    fn rotating_point_around_x() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_abs_diff_eq!(
            point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
            half_quarter * p
        );
        assert_abs_diff_eq!(point(0, 0, 1), full_quarter * p);
    }

    #[test]
    fn inverse_rotating_point_around_x() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();
        assert_abs_diff_eq!(
            point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
            inv * p
        );
    }

    #[test]
    fn rotating_point_around_y() {
        let p = point(0, 0, 1);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_abs_diff_eq!(
            point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0),
            half_quarter * p
        );
        assert_abs_diff_eq!(point(1, 0, 0), full_quarter * p);
    }

    #[test]
    fn rotating_point_around_z() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_abs_diff_eq!(
            point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0),
            half_quarter * p
        );
        assert_abs_diff_eq!(point(-1, 0, 0), full_quarter * p);
    }

    #[test]
    fn shearing_moving_x_proportional_to_y() {
        let transform = shearing(1, 0, 0, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(point(5, 3, 4), transform * p);
    }

    #[test]
    fn shearing_moving_x_proportional_to_z() {
        let transform = shearing(0, 1, 0, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(point(6, 3, 4), transform * p);
    }

    #[test]
    fn shearing_moving_y_proportional_to_x() {
        let transform = shearing(0, 0, 1, 0, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(point(2, 5, 4), transform * p);
    }

    #[test]
    fn shearing_moving_y_proportional_to_z() {
        let transform = shearing(0, 0, 0, 1, 0, 0);
        let p = point(2, 3, 4);
        assert_eq!(point(2, 7, 4), transform * p);
    }

    #[test]
    fn shearing_moving_z_proportional_to_x() {
        let transform = shearing(0, 0, 0, 0, 1, 0);
        let p = point(2, 3, 4);
        assert_eq!(point(2, 3, 6), transform * p);
    }

    #[test]
    fn shearing_moving_z_proportional_to_y() {
        let transform = shearing(0, 0, 0, 0, 0, 1);
        let p = point(2, 3, 4);
        assert_eq!(point(2, 3, 7), transform * p);
    }

    #[test]
    fn transforms_in_sequence() {
        let p = point(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5, 5, 5);
        let c = translation(10, 5, 7);

        let p2 = a * p;
        assert_abs_diff_eq!(point(1, -1, 0), p2);

        let p3 = b * p2;
        assert_abs_diff_eq!(point(5, -5, 0), p3);

        let p4 = c * p3;
        assert_abs_diff_eq!(point(15, 0, 7), p4);
    }

    #[test]
    fn chained_transforms() {
        let p = point(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5, 5, 5);
        let c = translation(10, 5, 7);
        let t = c * b * a;
        assert_abs_diff_eq!(point(15, 0, 7), t * p);
    }
}
