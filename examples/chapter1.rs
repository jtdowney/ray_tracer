use ray_tracer::{point, vector, Point, Vector};
use std::fmt;

type N = f32;

#[derive(Copy, Clone)]
struct Projectile {
    position: Point<N>,
    velocity: Vector<N>,
}

struct Simulation {
    gravity: Vector<N>,
    wind: Vector<N>,
    projectile: Projectile,
}

impl Simulation {
    fn tick(&mut self) {
        self.projectile.position = self.projectile.position + self.projectile.velocity;
        self.projectile.velocity = self.projectile.velocity + self.gravity + self.wind;
    }

    fn is_running(&self) -> bool {
        self.projectile.position.y >= 0.0
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "projectile.x={}, projectile.y={}",
            self.projectile.position.x, self.projectile.position.y
        )
    }
}

fn main() {
    let projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.0, 0.0).normalize(),
    };

    let mut simulation = Simulation {
        gravity: vector(0.0, -0.05, 0.0),
        wind: vector(0.0, 0.0, 0.0),
        projectile,
    };

    while simulation.is_running() {
        println!("{}", simulation);
        simulation.tick();
    }
}
