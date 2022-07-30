const ELEMENTARY_CHARGE: f64 = 1.602176565e-19;
const PERMITTIVITY: f64 = 8.85418782e-12;
const ELECTRON_MASS: f64 = 9.10938215e-31;

pub fn simulate(num_mesh_nodes: usize) {
    let mut potential = vec![0.0_f64; num_mesh_nodes];
    let mut charge_density = vec![ELEMENTARY_CHARGE; num_mesh_nodes];
    let mut electric_field = vec![0.0_f64; num_mesh_nodes];

    let mesh_origin: f64 = 0.0;
    let mesh_end: f64 = 0.1;

    let node_spacing = (mesh_end - mesh_origin) / (num_mesh_nodes as f64 - 1 as f64);

    println!("position,potential,charge_density,electric_field");
    
    for i in 0..potential.len() {
        let position = mesh_origin + i as f64 * node_spacing;
        println!("{},{},{},{}", position, potential[i], charge_density[i], electric_field[i]);
    }
}

