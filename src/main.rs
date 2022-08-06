mod constants;
mod field;
mod mesh;
mod output;
mod particle;
mod species;
mod vector;

mod grounded_box;
mod single_particle;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    const NUM_MESH_NODES: usize = 21;

    println!("Rust Plasma Physics Simulation");

    println!("Running single particle simulation...");
    let now = Instant::now();
    single_particle::simulate(NUM_MESH_NODES)?;
    let elapsed_time = now.elapsed();
    println!("Simulation took {} seconds.", elapsed_time.as_secs());
    println!("Single particle simulation complete.");

    println!("Running grounded box multi-particle simulation...");
    let now = Instant::now();
    grounded_box::simulate(NUM_MESH_NODES)?;
    let elapsed_time = now.elapsed();
    println!("Simulation took {} seconds.", elapsed_time.as_secs());
    println!("Grounded box multi-particle simulation complete.");

    Ok(())
}
