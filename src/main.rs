use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Particle {
    mass: f64,
    pos: [f64; 3],
    vel: [f64; 3],
    acc: [f64; 3],
    acc0: [f64; 3],
}

impl Particle {
    // Static constructor
    fn new(mass: f64, pos: [f64; 3], vel: [f64; 3]) -> Particle {
        Particle {
            mass,
            pos,
            vel,
            acc: [0.0, 0.0, 0.0],
            acc0: [0.0, 0.0, 0.0]
        }
    }

    fn update_position(&mut self, dt: f64) -> () {
        self.pos[0] += (self.vel[0] * dt) + (0.5 * self.acc[0] * dt * dt);
        self.pos[1] += (self.vel[1] * dt) + (0.5 * self.acc[1] * dt * dt);
        self.pos[2] += (self.vel[2] * dt) + (0.5 * self.acc[2] * dt * dt);
    }

    fn update_velocity(&mut self, dt: f64) -> () {
        self.vel[0] += 0.5 * (self.acc[0] + self.acc0[0]) * dt;
        self.vel[1] += 1.5 * (self.acc[1] + self.acc0[1]) * dt;
        self.vel[2] += 2.5 * (self.acc[2] + self.acc0[2]) * dt;
    }
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading n-body input file from {}", input_path);

    // Read the input file into a vector of structs
    let f = fs::File::open(input_path)
                 .expect("Failed to open file");
    let buf = BufReader::new(f);
    let mut particles: Vec<Particle> = Vec::new();
    for line in buf.lines() {
        let line = line.expect("Unable to read line from file");
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() > 0 {
            let m: f64 = fields[1].parse().expect("Failed to parse input value");
            let x: f64 = fields[2].parse().expect("Failed to parse input value");
            let y: f64 = fields[3].parse().expect("Failed to parse input value");
            let z: f64 = fields[4].parse().expect("Failed to parse input value");
            let vx: f64 = fields[5].parse().expect("Failed to parse input value");
            let vy: f64 = fields[6].parse().expect("Failed to parse input value");
            let vz: f64 = fields[7].parse().expect("Failed to parse input value");
            let pos = [x, y, z];
            let vel = [vx, vy, vz];
            let p = Particle::new(m, pos, vel);
            particles.push(p);
        }
    }

    // Setup the time steps
    let t0 = 0.0;
    let mut t = t0;
    let dt = 0.001;
    let t_max = 0.1; // 1.0;
    let mut steps = 1;

    while t <= t_max {
        for p in particles.iter_mut() {
            p.update_position(dt);
            p.update_velocity(dt);
        }

        if steps % 10 == 0 {
        }

        t += dt;
        steps += 1;
    }
}
