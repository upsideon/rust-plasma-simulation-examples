use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Dimensions {
    x: usize,
    y: usize,
    z: usize,
}

impl Dimensions {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Dimensions { x: x, y: y, z: z }
    }
}

impl From<Dimensions> for (usize, usize, usize) {
    fn from(dimensions: Dimensions) -> (usize, usize, usize) {
        (dimensions.x, dimensions.y, dimensions.z)
    }
}

#[derive(Debug)]
pub struct BoxMesh {
    /// Specifies coordinates of the origin in 3-dimensional space.
    origin: Vec3,
    /// Specifies the corner diagonally opposite of the origin.
    max_bound: Vec3,
    /// Specifies the number of nodes along the x, y and z axes.
    dimensions: Dimensions,
    /// Specifies the cell spacings for the x, y, and z axes.
    cell_spacings: [f64; 3],
    /// Specifies the centroid of the mesh.
    centroid: Vec3,
}

impl BoxMesh {
    pub fn new(origin: Vec3, max_bound: Vec3, dimensions: Dimensions) -> Self {
        let centroid = (origin + max_bound) * 0.5;

        let cell_spacings = [
            (max_bound.x - origin.x) / dimensions.x as f64,
            (max_bound.y - origin.y) / dimensions.y as f64,
            (max_bound.z - origin.z) / dimensions.z as f64,
        ];

        BoxMesh {
            origin: origin,
            max_bound: max_bound,
            dimensions: dimensions,
            cell_spacings: cell_spacings,
            centroid: centroid,
        }
    }
}
