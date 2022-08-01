use std::ops;

use crate::field::Field;
use crate::mesh::{BoxMesh, Dimensions};
use crate::point::Point;

pub fn simulate(num_mesh_nodes: usize) -> std::io::Result<()> {
    // Note that the mesh dimensions must be high enough, relative to the distance
    // between the origin and maximum bound, that the maximum dimension of a cell is
    // less than the Debye length. Otherwise, we won't be able to properly simulate
    // electrostatic interactions between particles.
    let mesh_dimensions = Dimensions::new(21, 21, 21);

    let grounded_box_mesh = BoxMesh::new(
        Point::new(-0.1, -0.1, -0.1),
        Point::new(0.1, 0.1, 0.2),
        mesh_dimensions,
    );

    let field = Field::<f64>::new(mesh_dimensions);

    Ok(())
}
