use ray_tracer::{matrix, vector, Matrix4};

fn main() {
    dbg!(Matrix4::<f64>::identity().inverse());

    let m = matrix(&[
        3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
    ]);
    dbg!(m);
    dbg!(m * m.inverse());

    dbg!(m.inverse().transpose());
    dbg!(m.transpose().inverse());

    let mut i = Matrix4::identity();
    let v = vector(1.0, 2.0, 3.0);
    dbg!(i * v);

    i[(1, 0)] = 2.0;
    dbg!(i * v);
}
