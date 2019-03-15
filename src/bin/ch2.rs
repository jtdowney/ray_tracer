use ray_tracer::{Canvas, Color, Point, Vector};

#[derive(Copy, Clone, Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Simulation {
    gravity: Vector,
    wind: Vector,
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
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let mut canvas = Canvas::new(900, 550);
    let c = Color::new(0.8, 0.2, 0.1);

    while projectile.position.y >= 0.0 {
        projectile = simulation.tick(projectile);

        let x = projectile.position.x.round() as usize;
        let y = (canvas.height as f32 - projectile.position.y).round() as usize;

        if x > canvas.width || y > canvas.height {
            continue;
        }

        canvas.write_pixel(x, y, c);
    }

    let data = canvas.to_ppm().unwrap();
    print!("{}", data);
}
