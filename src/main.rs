use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process;

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
        self.vel[1] += 0.5 * (self.acc[1] + self.acc0[1]) * dt;
        self.vel[2] += 0.5 * (self.acc[2] + self.acc0[2]) * dt;
    }

    fn calculate_ke(&self) -> f64 {
        let v2 = self.vel[0].powi(2) + self.vel[1].powi(2) + self.vel[2].powi(2);
        0.5 * self.mass * v2
    }

    fn get_distance(&self, other: &Particle) -> f64 {
        let dx = other.pos[0] - self.pos[0];
        let dy = other.pos[1] - self.pos[1];
        let dz = other.pos[2] - self.pos[2];
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}

fn calculate_pe(particles: &Vec<Particle>) -> f64 {
    let mut pe = 0.0;

    for i in 0..particles.len() {
        for j in (i+1)..particles.len() {
            let dr = particles[j].get_distance(&particles[i]);
            pe -= particles[i].mass * particles[j].mass / dr;
        }
    }

    pe
}

fn calculate_ke(particles: &Vec<Particle>) -> f64 {
    let mut ke = 0.0;

    for p in particles {
        ke += p.calculate_ke();
    }

    ke
}

fn update_accelerations(particles: &mut Vec<Particle>) {
    for i in 0..particles.len() {
        particles[i].acc0 = particles[i].acc;
        particles[i].acc = [0f64, 0f64, 0f64];

        for j in 0..particles.len() {
            if i == j { continue; }

            let dr = [particles[j].pos[0] - particles[i].pos[0],
                      particles[j].pos[1] - particles[i].pos[1],
                      particles[j].pos[2] - particles[i].pos[2]];

            let dr_cubed = (dr[0]*dr[0] + dr[1]*dr[1] + dr[2]*dr[2]).powf(1.5);

            particles[i].acc[0] += particles[j].mass * dr[0] / dr_cubed;
            particles[i].acc[1] += particles[j].mass * dr[1] / dr_cubed;
            particles[i].acc[2] += particles[j].mass * dr[2] / dr_cubed;
        }
    }
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("no input file specified");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // TODO: Panics if you don't give command line input
    let input_path = config.filename;
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
    let t_max = 1.0;
    let mut steps = 1;

    let ke0 = calculate_ke(&particles);
    let pe0 = calculate_pe(&particles);
    let e0 = ke0 + pe0;
    println!("Timestep {}:", steps);
    println!("E: {:.5}\tKE: {:.5}\tPE: {:.5}", e0, ke0, pe0);

    update_accelerations(&mut particles);

    while t <= t_max {
        for p in particles.iter_mut() {
            p.update_position(dt);
        }

        update_accelerations(&mut particles);

        for p in particles.iter_mut() {
            p.update_velocity(dt);
        }

        if steps % 10 == 0 {
            let ke = calculate_ke(&particles);
            let pe = calculate_pe(&particles);
            let e = ke+pe;
            println!("Timestep {}:", steps);
            println!("E: {:.5}\tKE: {:.5}\tPE: {:.5}\tdE: {:+.5e}", e, ke, pe, (e-e0)/e0);
        }

        t += dt;
        steps += 1;
    }
}
