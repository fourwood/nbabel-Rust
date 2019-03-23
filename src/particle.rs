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
        self.pos[0] += self.vel[0] * dt + 0.5 * self.acc[0] * dt.powi(2);
        self.pos[1] += self.vel[1] * dt + 0.5 * self.acc[1] * dt.powi(2);
        self.pos[2] += self.vel[2] * dt + 0.5 * self.acc[2] * dt.powi(2);
    }

    pub fn update_velocity(&mut self, dt: f64) -> () {
    }

    pub fn calculate_ke(&self) -> f64 {
        let v_2 = self.vel[0].powi(2) + self.vel[1].powi(2) + self.vel[2].powi(2);
        0.5 * self.mass * v_2
    }

    pub fn get_distance(&self, other: &Particle) -> f64 {
        0.0
    }
}
