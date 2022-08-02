mod constants;
mod field;
mod mesh;
mod vector;

mod grounded_box;
mod single_particle;

fn main() -> std::io::Result<()> {
    const NUM_MESH_NODES: usize = 21;

    println!("Rust Plasma Physics Simulation");

    println!("Running single particle simulation...");
    single_particle::simulate(NUM_MESH_NODES)?;
    println!("Single particle simulation complete.");

    println!("Running grounded box multi-particle simulation...");
    grounded_box::simulate(NUM_MESH_NODES)?;
    println!("Grounded box multi-particle simulation complete.");

    Ok(())
}
