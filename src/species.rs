use std::string::String;

use rand;
use rand::Rng;

use crate::field::Field;
use crate::mesh::{BoxMesh, Dimensions};
use crate::particle::Particle;
use crate::vector::Vec3;

/// Represents a species of particle.
pub struct Species {
    /// The name of the species.
    name: String,
    /// The mass of the particles in the species.
    mass: f64,
    /// The charge of the particles in the species.
    charge: f64,
    /// The number density of the species.
    number_density: Field<f64>,
    /// The particles within the species.
    particles: Vec<Particle>,
}

impl Species {
    /// Creates a new instance of Species.
    pub fn new(name: String, mass: f64, charge: f64, mesh_dimensions: Dimensions) -> Self {
        Species {
            name: name,
            mass: mass,
            charge: charge,
            number_density: Field::<f64>::new(mesh_dimensions),
            particles: Vec::<Particle>::new(),
        }
    }

    /// Returns the name of the species.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the charge of the particles in the species.
    pub fn charge(&self) -> f64 {
        self.charge
    }

    /// Returns the number density of the species.
    pub fn number_density(&self) -> Field<f64> {
        self.number_density.clone()
    }

    /// Adds a particle.
    pub fn add_particle(
        &mut self,
        position: Vec3,
        velocity: Vec3,
        macroparticle_weight: f64,
        mesh: &BoxMesh,
    ) {
        let lc = mesh.position_to_logical_coordinate(position);
        let electric_field = mesh.electric_field().gather(lc);

        let updated_velocity =
            velocity - electric_field * (self.charge / self.mass) * (0.5 * mesh.timestep());

        self.particles.push(Particle::new(
            position,
            updated_velocity,
            macroparticle_weight,
        ));
    }

    /// Adjusts particle positions and velocities.
    pub fn advance(&mut self, mesh: &BoxMesh) {
        let origin = mesh.origin();
        let max_bound = mesh.max_bound();
        let dimensions = mesh.dimensions();
        let dt = mesh.timestep();

        for particle in &mut self.particles {
            let lc = mesh.position_to_logical_coordinate(particle.position);
            let electric_field = mesh.electric_field().gather(lc);
            particle.velocity += electric_field * (dt * (self.charge / self.mass));
            particle.position += particle.velocity * dt;

            // Reflecting particles leaving the mesh.
            if lc.x < 0.0 {
                particle.position.x = 2.0 * origin.x - particle.position.x;
                particle.velocity.x *= -1.0;
            } else if lc.x >= (dimensions.x - 1) as f64 {
                particle.position.x = 2.0 * max_bound.x - particle.position.x;
                particle.velocity.x *= -1.0;
            }

            if lc.y < 0.0 {
                particle.position.y = 2.0 * origin.y - particle.position.y;
                particle.velocity.y *= -1.0;
            } else if lc.y >= (dimensions.y - 1) as f64 {
                particle.position.y = 2.0 * max_bound.y - particle.position.y;
                particle.velocity.y *= -1.0;
            }

            if lc.z < 0.0 {
                particle.position.z = 2.0 * origin.z - particle.position.z;
                particle.velocity.z *= -1.0;
            } else if lc.z >= (dimensions.z - 1) as f64 {
                particle.position.z = 2.0 * max_bound.z - particle.position.z;
                particle.velocity.z *= -1.0;
            }
        }
    }

    /// Computes the number density of the species based on the simulation mesh.
    pub fn compute_number_density(&mut self, mesh: &BoxMesh) {
        self.number_density.clear();

        for particle in &self.particles {
            let logical_coordinate = mesh.position_to_logical_coordinate(particle.position);
            self.number_density
                .scatter(logical_coordinate, particle.macroparticle_weight);
        }

        self.number_density = self.number_density.clone() / mesh.node_volumes();
    }

    /// Loads particles in a box defined by points in opposite corners of the box.
    pub fn _load_particles_box(
        &mut self,
        origin: Vec3,
        opposite: Vec3,
        number_density: f64,
        num_macroparticles: usize,
        mesh: &BoxMesh,
    ) {
        let diagonal_vector = opposite - origin;
        let box_volume = diagonal_vector.x * diagonal_vector.y * diagonal_vector.z;
        let num_real_particles = number_density * box_volume;
        let macroparticle_weight = num_real_particles / num_macroparticles as f64;

        self.particles.reserve(num_macroparticles);

        let mut rng = rand::thread_rng();

        for _ in 0..num_macroparticles {
            let r = rng.gen::<f64>();
            let position = origin + diagonal_vector * r;
            let velocity = Vec3::new(0.0, 0.0, 0.0);
            self.add_particle(position, velocity, macroparticle_weight, mesh);
        }
    }

    /// Loads particles in a box using the quite start method.
    pub fn load_particles_box_qs(
        &mut self,
        origin: Vec3,
        opposite: Vec3,
        number_density: f64,
        num_macroparticles: (usize, usize, usize),
        mesh: &BoxMesh,
    ) {
        let diagonal_vector = opposite - origin;
        let box_volume = diagonal_vector.x * diagonal_vector.y * diagonal_vector.z;
        let total_macroparticles =
            (num_macroparticles.0 - 1) * (num_macroparticles.1 - 1) * (num_macroparticles.2 - 1);
        let num_real_particles = number_density * box_volume;
        let macroparticle_weight = num_real_particles / total_macroparticles as f64;

        let di = diagonal_vector.x / (num_macroparticles.0 - 1) as f64;
        let dj = diagonal_vector.y / (num_macroparticles.1 - 1) as f64;
        let dk = diagonal_vector.z / (num_macroparticles.2 - 1) as f64;

        self.particles.reserve(total_macroparticles);

        for i in 0..num_macroparticles.0 {
            for j in 0..num_macroparticles.1 {
                for k in 0..num_macroparticles.2 {
                    let mut position =
                        origin + Vec3::new(i as f64 * di, j as f64 * dj, k as f64 * dk);

                    // Shifting particles on maximum faces back to the domain.
                    if position.x == opposite.x {
                        position.x -= 1e-4 * di;
                    }

                    if position.y == opposite.y {
                        position.y -= 1e-4 * dj;
                    }

                    if position.z == opposite.z {
                        position.z -= 1e-4 * dk;
                    }

                    let mut weight_factor = 1.0;

                    if i == 0 || i == num_macroparticles.0 - 1 {
                        weight_factor *= 0.5;
                    }

                    if j == 0 || j == num_macroparticles.1 - 1 {
                        weight_factor *= 0.5;
                    }

                    if k == 0 || k == num_macroparticles.2 - 1 {
                        weight_factor *= 0.5;
                    }

                    let velocity = Vec3::new(0.0, 0.0, 0.0);
                    self.add_particle(
                        position,
                        velocity,
                        macroparticle_weight * weight_factor,
                        mesh,
                    );
                }
            }
        }
    }
}
