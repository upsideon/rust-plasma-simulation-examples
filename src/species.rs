use std::string::String;

pub struct Species {
    name: String,
    mass: f64,
    charge: f64,
}

impl Species {
    pub fn new(name: String, mass: f64, charge: f64) -> Self {
        Species {
            name: name,
            mass: mass,
            charge: charge,
        }
    }
}
