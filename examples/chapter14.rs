use indicatif::ProgressBar;
use ray_tracer::{
    color, transforms, Camera, CheckersPatternBuilder, Color, CubeBuilder, CylinderBuilder,
    GroupBuilder, MaterialBuilder, Point, PointLight, SphereBuilder, Vector3, WorldBuilder,
};
use std::f64::consts::PI;
use std::fmt;

fn hexagon_corner(builder: &mut WorldBuilder) {
    builder.object(
        SphereBuilder::default()
            .transform(
                transforms::translation(0.0, 0.0, -1.0) * transforms::scaling(0.25, 0.25, 0.25),
            )
            .material(
                MaterialBuilder::default()
                    .color(Color::new(0.8, 0.8, 0.4))
                    .diffuse(0.3)
                    .ambient(0.2)
                    .specular(0.2)
                    .reflective(0.9)
                    .shininess(100.0)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );
}

fn hexagon_edge(builder: &mut WorldBuilder) {
    builder.object(
        CylinderBuilder::default()
            .minimum(0.0)
            .maximum(1.0)
            .transform(
                transforms::translation(0.0, 0.0, -1.0)
                    * transforms::rotation_y(-PI / 6.0)
                    * transforms::rotation_z(-PI / 2.0)
                    * transforms::scaling(0.25, 1.0, 0.25),
            )
            .material(
                MaterialBuilder::default()
                    .color(Color::new(0.8, 0.8, 0.4))
                    .diffuse(0.3)
                    .ambient(0.2)
                    .specular(0.2)
                    .reflective(0.9)
                    .shininess(100.0)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );
}

fn hexagon(builder: &mut WorldBuilder) {
    for n in 0..=5 {
        builder.start_group(
            GroupBuilder::default()
                .transform(transforms::rotation_y(f64::from(n) * PI / 3.0))
                .build()
                .unwrap(),
        );

        hexagon_corner(builder);
        hexagon_edge(builder);

        builder.end_group();
    }
}

fn main() -> Result<(), fmt::Error> {
    let mut builder = WorldBuilder::default();
    builder.object(
        CubeBuilder::default() // room
            .transform(
                transforms::rotation_y(PI / 3.5)
                    * transforms::translation(0.0, 12.0, 0.0)
                    * transforms::scaling(15.0, 12.0, 15.0),
            )
            .material(
                MaterialBuilder::default()
                    .pattern(
                        CheckersPatternBuilder::default()
                            .first(color::WHITE)
                            .second(color::BLACK)
                            .transform(transforms::scaling(1.0 / 12.0, 1.0 / 12.0, 1.0 / 12.0))
                            .build()
                            .unwrap(),
                    )
                    .specular(0.0)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );
    builder.light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        color::WHITE,
    ));

    builder.start_group(
        GroupBuilder::default()
            .transform(transforms::translation(0.0, 1.5, 0.0) * transforms::rotation_x(-PI / 6.0))
            .build()
            .unwrap(),
    );

    hexagon(&mut builder);
    builder.end_group();

    let world = builder.build();

    let width = 1000;
    let height = 500;
    let mut camera = Camera::new(width as u16, height as u16, PI / 3.0);
    camera.transform = transforms::view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let bar = ProgressBar::new(width * height);
    let canvas = ray_tracer::render_parallel(camera, world, || bar.inc(1));
    print!("{}", canvas.to_ppm()?);

    Ok(())
}
