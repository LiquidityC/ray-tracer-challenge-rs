mod math;

use std::fmt;

use math::Tuple;

#[derive(Debug)]
struct Projectile {
    pos: Tuple,
    vel: Tuple
}

struct Environment {
    grav: Tuple,
    wind: Tuple
}

impl fmt::Display for Projectile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} {:.2} {:.2}", self.pos.x, self.pos.y, self.pos.z)
    }
}

fn main() {
    let mut p = Projectile {
        pos: Tuple::point(0.0, 1.0, 0.0),
        vel: Tuple::vector(2.0, 1.0, 0.0).normal()
    };

    let env = Environment {
        grav: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0)
    };

    loop {
        println!("{}", p);
        p.pos = p.pos + p.vel;
        p.vel = p.vel + env.grav + env.wind;
        if p.pos.y <= 0.0 {
            break;
        }
    }
}
