mod constants;
mod field;
mod mesh;
mod output;
mod particle;
mod species;
mod vector;

mod grounded_box;
mod single_particle;

use std::env;
use std::io::{Error, ErrorKind};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    const NUM_MESH_NODES: usize = 21;

    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    if num_args < 2 || num_args > 2 {
        print_usage();
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Must specify at least one argument.",
        ));
    }

    let argument = &args[1];

    if argument == "-h" || argument == "--help" {
        print_usage();
    } else if argument == "single-particle" {
        println!("Running single particle simulation...");
        let now = Instant::now();
        single_particle::simulate(NUM_MESH_NODES)?;
        let elapsed_time = now.elapsed();
        println!("Simulation took {} seconds.", elapsed_time.as_secs());
        println!("Single particle simulation complete.");
    } else if argument == "grounded-box" {
        println!("Running grounded box multi-particle simulation...");
        let now = Instant::now();
        grounded_box::simulate(NUM_MESH_NODES)?;
        let elapsed_time = now.elapsed();
        println!("Simulation took {} seconds.", elapsed_time.as_secs());
        println!("Grounded box multi-particle simulation complete.");
    }

    Ok(())
}

fn print_usage() {
    println!("Rust Plasma Physics Simulation Examples");
    println!("USAGE:\n\tplasma-simulation {{OPTIONS | SIMULATION}}");
    println!("OPTIONS:\n\t-h, --help\tPrint help information");
    println!("SIMULATION:\n\tsingle-particle\n\tgrounded-box");
}
