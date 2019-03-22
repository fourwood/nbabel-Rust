use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

mod particle;
use particle::Particle;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("no input file specified");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

fn calculate_pe(particles: &Vec<Particle>) -> f64 {
    let mut pe = 0.0;

    pe
}

fn calculate_ke(particles: &Vec<Particle>) -> f64 {
    let mut ke = 0.0;

    ke
}

fn update_accelerations(particles: &mut Vec<Particle>) -> () {
}

fn load_state_from_file(filename: &String) -> Vec<Particle> {
    let f = fs::File::open(filename).expect("Failed to open file");
    let buf = BufReader::new(f);
    let mut particles: Vec<Particle> = Vec::new();
    for line in buf.lines() {
        let line = line.expect("Unable to read line from file");
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() > 0 {
            let m: f64 = fields[1].parse().expect("Unable to parse particle mass!");
            let x: f64 = fields[2].parse().expect("Unable to parse particle x-position!");
            let y: f64 = fields[3].parse().expect("Unable to parse particle y-position!");
            let z: f64 = fields[4].parse().expect("Unable to parse particle z-position!");
            let vx: f64 = fields[5].parse().expect("Unable to parse particle x-velocity!");
            let vy: f64 = fields[6].parse().expect("Unable to parse particle y-velocity!");
            let vz: f64 = fields[7].parse().expect("Unable to parse particle z-velocity!");
            let pos = [x, y, z];
            let vel = [vx, vy, vz];
            let p = Particle::new(m, pos, vel);
            particles.push(p);
        }
    }
    particles
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO: Panics if you don't give command line input
    let input_path = config.filename;
    println!("Reading n-body input file from {}", input_path);

    // Read the input file into a vector of structs
    let mut particles = load_state_from_file(&input_path);

    // Setup the time steps
    let t0 = 0.0;
    let mut t = t0;
    let dt = 0.001;
    let t_max = 1.0;
    let mut steps = 1;

    // TODO: Calculate starting energy
    let E_0: f64 = 0.0;
    let PE_0: f64 = 0.0;
    let KE_0: f64 = 0.0;
    println!("Timestep {}:", steps);
    println!("E: {:.5}\tKE: {:.5}\tPE: {:.5}", E_0, KE_0, PE_0);

    // TODO: Set initial accelerations

    while t <= t_max {
        // TODO: Update particle positions

        // TODO: Update particle accelerations (& a->a0)

        // TODO: Update particle velocities

        if steps % 10 == 0 {
            // TODO: Calculate total system energy
            let E: f64 = 0.0;
            let KE: f64 = 0.0;
            let PE: f64 = 0.0;
            println!("Timestep {}:", steps);
            println!("E: {:.5}\tKE: {:.5}\tPE: {:.5}\tdE: {:+.5e}", E, KE, PE, (E-E_0)/E_0);
        }

        t += dt;
        steps += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input() {
        // The entirety of the input16 file, with unused id set to a float:
        let input16 = [
            [-1.0, 0.0625, 0.214835865608654947, -0.120469325327368135, -0.2661811268232539, 0.757849421156586822, 0.157644506165220161, -0.0715602005208729741],
            [-1.0, 0.0625, -0.129919123158631022, 0.116379094303405983, 0.133080554128754053, 0.486052966602537106, 0.395427627261877745, -0.734692224692122475],
            [-1.0, 0.0625, -0.410208378045080091, 0.109354870524110206, -0.324396822089431269, -0.383791028776759968, 0.0496602615700619254, 0.378808792416775286],
            [-1.0, 0.0625, 0.255086977437966045, -0.123206701489244466, -0.370490384977665566, -0.450466427040683792, -0.281691172697528636, -0.185462114144705426],
            [-1.0, 0.0625, -0.354752159898959474, -0.31060829606615864, 0.314540789088777084, 0.275159898604911246, -0.548791386040452633, -0.209797996849041768],
            [-1.0, 0.0625, 0.16502984445788807, -0.21452299393747018, 1.32555928694859393, 0.140593096267161405, 0.23623605142486559, 0.273061388949567607],
            [-1.0, 0.0625, 0.429578710749940917, 0.486818153487116934, 0.181073664985952287, -0.0539885959200906915, -0.176655306840287851, -0.69464977663931049],
            [-1.0, 0.0625, -0.615104941591427701, 0.297292071802583491, -0.407208633089183814, 0.144247206691717561, 0.880157154616842852, 0.372927641740506965],
            [-1.0, 0.0625, 0.526844391659375466, -0.225238863848920445, -0.0858175235997876429, -0.170693379896528441, 0.525004419487255625, -0.165895533066861667],
            [-1.0, 0.0625, -0.784247975459690938, 2.88024285713089023, 0.422769785428553202, -0.397599351887202745, -0.193469722164481828, 0.129716098638342187],
            [-1.0, 0.0625, 0.557328942603933286, -0.669584458021703521, -1.18669004231490538, -0.193956120455493958, 0.318551106618286173, -0.0385091511689474308],
            [-1.0, 0.0625, 0.663045405926924847, -1.45465317533174687, 1.08382405457320763, -0.256111120227146827, -0.202605323424895567, 0.288501494317296991],
            [-1.0, 0.0625, -0.227323512866007577, 0.348104100173003139, -0.147906561326105135, 0.718552359009200226, -0.0282296194471982724, 0.900931748619264838],
            [-1.0, 0.0625, -0.0756982740351159478, 0.150819419760155976, -0.408069797108747689, -0.0568988462176753251, -0.970986937859766863, -0.0214726046192551326],
            [-1.0, 0.0625, 0.466554170775371646, -0.914680937590023668, 0.279919799571218153, -0.499730803777436394, -0.201272413171611736, 0.313715970489572504],
            [-1.0, 0.0625, -0.681049944165142307, -0.356045815568630075, -0.544007043395975831, -0.0592192741330963002, 0.0410207545018133196, -0.535623533470209079],
        ];

        let filename = String::from("input/input16");
        let mut particles: Vec<Particle> = load_state_from_file(&filename);

        for (i, p) in particles.iter().enumerate() {
            let mass = input16[i][1];
            let pos = [input16[i][2], input16[i][3], input16[i][4]];
            let vel = [input16[i][5], input16[i][6], input16[i][7]];
            assert_eq!(p.mass, mass);
            assert_eq!(p.pos, pos);
            assert_eq!(p.vel, vel);
            assert_eq!(p.acc, [0.0, 0.0, 0.0]);
            assert_eq!(p.acc0, [0.0, 0.0, 0.0]);
        }
    }
}
