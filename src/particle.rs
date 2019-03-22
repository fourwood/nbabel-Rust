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
    }

    pub fn update_velocity(&mut self, dt: f64) -> () {
    }

    pub fn calculate_ke(&self) -> f64 {
        0.0
    }

    pub fn get_distance(&self, other: &Particle) -> f64 {
        0.0
    }
}
