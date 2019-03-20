#[derive(Debug)]
pub struct Particle {
    pub mass: f64,
    pub pos: [f64; 3],
    pub vel: [f64; 3],
    pub acc: [f64; 3],
    pub acc0: [f64; 3],
}

impl Particle {
    // Static constructor
    pub fn new(mass: f64, pos: [f64; 3], vel: [f64; 3]) -> Particle {
        Particle {
            mass,
            pos,
            vel,
            acc: [0.0, 0.0, 0.0],
            acc0: [0.0, 0.0, 0.0]
        }
    }

    pub fn update_position(&mut self, dt: f64) -> () {
        self.pos[0] += (self.vel[0] * dt) + (0.5 * self.acc[0] * dt * dt);
        self.pos[1] += (self.vel[1] * dt) + (0.5 * self.acc[1] * dt * dt);
        self.pos[2] += (self.vel[2] * dt) + (0.5 * self.acc[2] * dt * dt);
    }

    pub fn update_velocity(&mut self, dt: f64) -> () {
        self.vel[0] += 0.5 * (self.acc[0] + self.acc0[0]) * dt;
        self.vel[1] += 0.5 * (self.acc[1] + self.acc0[1]) * dt;
        self.vel[2] += 0.5 * (self.acc[2] + self.acc0[2]) * dt;
    }

    pub fn calculate_ke(&self) -> f64 {
        let v2 = self.vel[0].powi(2) + self.vel[1].powi(2) + self.vel[2].powi(2);
        0.5 * self.mass * v2
    }

    pub fn get_distance(&self, other: &Particle) -> f64 {
        let dx = other.pos[0] - self.pos[0];
        let dy = other.pos[1] - self.pos[1];
        let dz = other.pos[2] - self.pos[2];
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}
