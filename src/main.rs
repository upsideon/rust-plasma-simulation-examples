mod single_particle;

fn main() {
    const NUM_MESH_NODES: usize = 21;

    println!("Rust Plasma Physics Simulation");
    single_particle::simulate(NUM_MESH_NODES);
}
