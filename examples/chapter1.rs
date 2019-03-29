use ray_tracer::{Point, Vector3};

#[derive(Copy, Clone, Debug)]
struct Projectile {
    position: Point,
    velocity: Vector3,
}

struct Simulation {
    gravity: Vector3,
    wind: Vector3,
}

impl Simulation {
    fn tick(&self, mut projectile: Projectile) -> Projectile {
        projectile.position = projectile.position + projectile.velocity;
        projectile.velocity = projectile.velocity + self.gravity + self.wind;
        projectile
    }
}

fn main() {
    let simulation = Simulation {
        gravity: Vector3::new(0.0, -0.05, 0.0),
        wind: Vector3::new(0.00, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector3::new(1.0, 1.0, 0.0).normalize(),
    };

    while projectile.position.y >= 0.0 {
        dbg!(projectile);
        projectile = simulation.tick(projectile);
    }
}
