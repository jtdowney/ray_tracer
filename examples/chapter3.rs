use ray_tracer::{Matrix4, Vector4};

fn main() {
    dbg!(Matrix4::<f32>::identity().inverse().unwrap());

    let m = Matrix4::new(&[
        3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
    ]);
    dbg!(m);
    dbg!(m * m.inverse().unwrap());

    dbg!(m.inverse().unwrap().transpose());
    dbg!(m.transpose().inverse().unwrap());

    let mut i = Matrix4::<f32>::identity();
    let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
    dbg!(i * v);

    i[(1, 0)] = 2.0;
    dbg!(i * v);
}
