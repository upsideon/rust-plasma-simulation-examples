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
    );

    grounded_box_mesh.solve_potential(MAX_ITERATIONS, CONVERGENCE_TOLERANCE);
    grounded_box_mesh.compute_electric_field();

    let species = vec![
        Species::new(
            String::from("O+"),
            16.0 * ATOMIC_MASS_UNIT,
            ELEMENTARY_CHARGE,
        ),
        Species::new(String::from("e-"), ELECTRON_MASS, -ELEMENTARY_CHARGE),
    ];

    Ok(())
}
