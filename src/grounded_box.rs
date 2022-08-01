use std::ops;

use ndarray::prelude::*;
use ndarray::{Array3, Dim};

use num_traits::identities::Zero;

#[derive(Clone, Copy, Debug)]
struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> ops::Mul<T> for Point
where
    f64: From<T>,
    T: Copy,
{
    type Output = Point;

    fn mul(self, other: T) -> Self {
        Point {
            x: self.x * f64::from(other),
            y: self.y * f64::from(other),
            z: self.z * f64::from(other),
        }
    }
}

#[derive(Debug)]
struct MeshDimensions {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Debug)]
struct Field<T: Clone + Zero> {
    data: Array3<T>,
}

impl<T: Clone + Zero> ops::Index<[usize; 3]> for Field<T> {
    type Output = T;

    fn index(&self, indices: [usize; 3]) -> &Self::Output {
        &self.data[indices]
    }
}

impl<T: Clone + Zero> Field<T> {
    fn new(dimensions: MeshDimensions) -> Self {
        let data = Array3::<T>::zeros((
            dimensions.x as usize,
            dimensions.y as usize,
            dimensions.z as usize,
        ));

        Field { data: data }
    }
}

#[derive(Debug)]
struct GroundedBoxMesh {
    /// Specifies coordinates of the origin in 3-dimensional space.
    origin: Point,
    /// Specifies the corner diagonally opposite of the origin.
    max_bound: Point,
    /// Specifies the number of nodes along the x, y and z axes.
    dimensions: MeshDimensions,
    /// Specifies the cell spacings for the x, y, and z axes.
    cell_spacings: [f64; 3],
    /// Specifies the centroid of the mesh.
    centroid: Point,
}

impl GroundedBoxMesh {
    fn new(origin: Point, max_bound: Point, dimensions: MeshDimensions) -> Self {
        let centroid = (origin + max_bound) * 0.5;

        let cell_spacings = [
            (max_bound.x - origin.x) / dimensions.x as f64,
            (max_bound.y - origin.y) / dimensions.y as f64,
            (max_bound.z - origin.z) / dimensions.z as f64,
        ];

        GroundedBoxMesh {
            origin: origin,
            max_bound: max_bound,
            dimensions: dimensions,
            cell_spacings: cell_spacings,
            centroid: centroid,
        }
    }
}

pub fn simulate(num_mesh_nodes: usize) -> std::io::Result<()> {
    // Note that the mesh dimensions must be high enough, relative to the distance
    // between the origin and maximum bound, that the maximum dimension of a cell is
    // less than the Debye length. Otherwise, we won't be able to properly simulate
    // electrostatic interactions between particles.
    let grounded_box_mesh = GroundedBoxMesh::new(
        Point {
            x: -0.1,
            y: -0.1,
            z: -0.1,
        },
        Point {
            x: 0.1,
            y: 0.1,
            z: 0.2,
        },
        MeshDimensions {
            x: 21,
            y: 21,
            z: 21,
        },
    );

    Ok(())
}
