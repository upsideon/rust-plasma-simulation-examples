use crate::constants::{ATOMIC_MASS_UNIT, ELECTRON_MASS, ELEMENTARY_CHARGE};
use crate::mesh::{BoxMesh, Dimensions};
use crate::species::Species;
use crate::vector::Vec3;

const MAX_ITERATIONS: usize = 4000;
const CONVERGENCE_TOLERANCE: f64 = 1e-6;

pub fn simulate(num_mesh_nodes: usize) -> std::io::Result<()> {
    // Note that the mesh dimensions must be high enough, relative to the distance
    // between the origin and maximum bound, that the maximum dimension of a cell is
    // less than the Debye length. Otherwise, we won't be able to properly simulate
    // electrostatic interactions between particles.
    let mesh_dimensions = Dimensions::new(num_mesh_nodes, num_mesh_nodes, num_mesh_nodes);

    let mut grounded_box_mesh = BoxMesh::new(
        Vec3::new(-0.1, -0.1, -0.1),
        Vec3::new(0.1, 0.1, 0.2),
        mesh_dimensions,
        2e-10,
    );

    grounded_box_mesh.solve_potential(MAX_ITERATIONS, CONVERGENCE_TOLERANCE);
    grounded_box_mesh.compute_electric_field();

    let mut species = vec![
        Species::new(
            String::from("O+"),
            16.0 * ATOMIC_MASS_UNIT,
            ELEMENTARY_CHARGE,
            grounded_box_mesh.dimensions(),
        ),
        Species::new(
            String::from("e-"),
            ELECTRON_MASS,
            -ELEMENTARY_CHARGE,
            grounded_box_mesh.dimensions(),
        ),
    ];

    const NUMBER_DENSITY: f64 = 1e11;
    const NUM_IONS: usize = 80000;
    const NUM_ELECTRONS: usize = 10000;

    species[0].load_particles_box(
        grounded_box_mesh.origin(),
        grounded_box_mesh.max_bound(),
        NUMBER_DENSITY,
        NUM_IONS,
        &grounded_box_mesh,
    );
    species[1].load_particles_box(
        grounded_box_mesh.origin(),
        grounded_box_mesh.centroid(),
        NUMBER_DENSITY,
        NUM_ELECTRONS,
        &grounded_box_mesh,
    );

    // Runing the simulation for 10,000 iterations.
    for iteration in 0..2 {
        println!("Iteration: {}", iteration);

        // Computing charge density.
        grounded_box_mesh.compute_charge_density(&species);

        // Update potential.
        grounded_box_mesh.solve_potential(MAX_ITERATIONS, CONVERGENCE_TOLERANCE);

        // Update electric field.
        grounded_box_mesh.compute_electric_field();

        // Computing number density.
        for s in &mut species {
            s.advance(&grounded_box_mesh);
            s.compute_number_density(&grounded_box_mesh);
        }
    }

    Ok(())
}
