use crate::vector::Vec3;

pub struct Particle {
    position: Vec3,
    velocity: Vec3,
    macroparticle_weight: f64,
}

impl Particle {
    pub fn new(position: Vec3, velocity: Vec3, macroparticle_weight: f64) -> Self {
        Particle {
            position: position,
            velocity: velocity,
            macroparticle_weight: macroparticle_weight,
        }
    }
}
