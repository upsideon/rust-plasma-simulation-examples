use crate::vector::Vec3;

/// Represents a particle.
pub struct Particle {
    /// The position of the particle.
    pub position: Vec3,
    /// The velocity of the particle.
    pub velocity: Vec3,
    /// The macroparticle weight.
    pub macroparticle_weight: f64,
}

impl Particle {
    /// Creates a new particle.
    pub fn new(position: Vec3, velocity: Vec3, macroparticle_weight: f64) -> Self {
        Particle {
            position: position,
            velocity: velocity,
            macroparticle_weight: macroparticle_weight,
        }
    }
}
