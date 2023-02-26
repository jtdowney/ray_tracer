use ray_tracer::{identity_matrix, matrix, vector};

fn main() {
    dbg!(identity_matrix::<4>().inverse());

    let m = matrix([[3, -9, 7, 3], [3, -8, 2, -9], [-4, 4, 4, 1], [-6, 5, -1, 1]]);
    dbg!(m);
    dbg!(m * m.inverse());

    dbg!(m.inverse().transpose());
    dbg!(m.transpose().inverse());

    let mut i = identity_matrix();
    let v = vector(1, 2, 3);
    dbg!(i * v);

    i[(1, 0)] = 2.0;
    dbg!(i * v);
}
