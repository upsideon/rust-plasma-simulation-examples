use std::string::String;

use rand;
use rand::Rng;

use crate::field::Field;
use crate::mesh::BoxMesh;
use crate::particle::Particle;
use crate::vector::Vec3;

pub struct Species<'a> {
    name: String,
    mass: f64,
    charge: f64,
    number_density: Field<f64>,
    mesh: &'a BoxMesh,
    particles: Vec<Particle>,
}

impl<'a> Species<'a> {
    pub fn new(name: String, mass: f64, charge: f64, mesh: &'a BoxMesh) -> Self {
        Species {
            name: name,
            mass: mass,
            charge: charge,
            number_density: Field::<f64>::new(mesh.dimensions()),
            mesh: mesh,
            particles: Vec::<Particle>::new(),
        }
    }

    pub fn charge(&self) -> f64 {
        self.charge
    }

    pub fn number_density(&self) -> Field<f64> {
        self.number_density.clone()
    }

    /// Adds a particle.
    pub fn add_particle(&mut self, position: Vec3, velocity: Vec3, macroparticle_weight: f64) {
        self.particles
            .push(Particle::new(position, velocity, macroparticle_weight));
    }

    pub fn advance(&mut self, timestep: f64) {
        let origin = self.mesh.origin();
        let max_bound = self.mesh.max_bound();

        for particle in &mut self.particles {
            let logical_coordinate = self.mesh.position_to_logical_coordinate(particle.position);
            let electric_field = self.mesh.electric_field().gather(logical_coordinate);
            particle.velocity += electric_field * (timestep * self.charge / self.mass);
            particle.position += particle.velocity * timestep;
        }
    }

    pub fn compute_number_density(&mut self) {
        self.number_density.clear();

        for particle in &self.particles {
            let logical_coordinate = self.mesh.position_to_logical_coordinate(particle.position);
            self.number_density
                .scatter(logical_coordinate, particle.macroparticle_weight);
        }

        self.number_density = self.number_density.clone() / self.mesh.node_volumes();
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
