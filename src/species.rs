use std::string::String;

use rand;
use rand::Rng;

use crate::particle::Particle;
use crate::vector::Vec3;

pub struct Species {
    name: String,
    mass: f64,
    charge: f64,
    particles: Vec<Particle>,
}

impl Species {
    pub fn new(name: String, mass: f64, charge: f64) -> Self {
        Species {
            name: name,
            mass: mass,
            charge: charge,
            particles: Vec::<Particle>::new(),
        }
    }

    /// Adds a particle.
    pub fn add_particle(&mut self, position: Vec3, velocity: Vec3, macroparticle_weight: f64) {
        self.particles
            .push(Particle::new(position, velocity, macroparticle_weight));
    }

    /// Loads particles in a box defined by points in opposite corners of the box.
    pub fn load_particles_box(
        &mut self,
        origin: Vec3,
        opposite: Vec3,
        number_density: f64,
        num_macroparticles: usize,
    ) {
        let diagonal_vector = opposite - origin;
        let box_volume = diagonal_vector.x * diagonal_vector.y * diagonal_vector.z;
        let num_real_particles = number_density * box_volume;
        let macroparticle_weight = num_real_particles / num_macroparticles as f64;

        self.particles.reserve(num_macroparticles);

        let mut rng = rand::thread_rng();

        for i in 0..num_macroparticles {
            let r = rng.gen::<f64>();
            let position = origin + diagonal_vector * r;
            let velocity = Vec3::new(0.0, 0.0, 0.0);
            self.add_particle(position, velocity, macroparticle_weight);
        }
    }
}
