use crate::constants::PERMITTIVITY;
use crate::field::Field;
use crate::species::Species;
use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Dimensions {
    pub x: usize,
    pub y: usize,
    pub z: usize,
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

#[derive(Clone, Debug)]
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
    /// Specifies the volumes of nodes for number density calculations.
    node_volumes: Field<f64>,
    /// Specifies the timestep for simulation.
    timestep: f64,
    /// Specifies the potential on the mesh.
    potential: Field<f64>,
    /// Specifies the charge density on the mesh.
    charge_density: Field<f64>,
    /// Specifies the electric field on the mesh.
    electric_field: Field<Vec3>,
}

impl BoxMesh {
    pub fn new(origin: Vec3, max_bound: Vec3, dimensions: Dimensions, timestep: f64) -> Self {
        let centroid = (origin + max_bound) * 0.5;

        let cell_spacings = [
            (max_bound.x - origin.x) / dimensions.x as f64,
            (max_bound.y - origin.y) / dimensions.y as f64,
            (max_bound.z - origin.z) / dimensions.z as f64,
        ];

        let mut mesh = BoxMesh {
            origin: origin,
            max_bound: max_bound,
            dimensions: dimensions,
            cell_spacings: cell_spacings,
            centroid: centroid,
            node_volumes: Field::<f64>::new(dimensions),
            timestep: timestep,
            potential: Field::<f64>::new(dimensions),
            charge_density: Field::<f64>::new(dimensions),
            electric_field: Field::<Vec3>::new(dimensions),
        };

        mesh.compute_node_volumes();

        mesh
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn max_bound(&self) -> Vec3 {
        self.max_bound
    }

    pub fn centroid(&self) -> Vec3 {
        self.centroid
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn node_volumes(&self) -> Field<f64> {
        self.node_volumes.clone()
    }

    pub fn electric_field(&self) -> Field<Vec3> {
        self.electric_field.clone()
    }

    pub fn timestep(&self) -> f64 {
        self.timestep
    }

    pub fn position_to_logical_coordinate(&self, position: Vec3) -> Vec3 {
        let mut logical_coordinate = position - self.origin;
        logical_coordinate.x /= self.dimensions.x as f64;
        logical_coordinate.y /= self.dimensions.y as f64;
        logical_coordinate.z /= self.dimensions.z as f64;
        logical_coordinate
    }

    pub fn compute_charge_density(&mut self, species: Vec<Species>) {
        self.charge_density.clear();

        for s in &species {
            if s.charge() == 0.0 {
                continue;
            }

            self.charge_density += s.number_density() * s.charge();
        }
    }

    pub fn compute_node_volumes(&mut self) {
        let cell_spacings = self.cell_spacings;
        let dimensions = self.dimensions;

        for i in 0..dimensions.x {
            for j in 0..dimensions.y {
                for k in 0..dimensions.z {
                    let mut volume = cell_spacings[0] * cell_spacings[1] * cell_spacings[2];
                    if i == 0 || i == dimensions.x - 1 {
                        volume *= 0.5;
                    }

                    if j == 0 || j == dimensions.y - 1 {
                        volume *= 0.5;
                    }

                    if k == 0 || k == dimensions.z - 1 {
                        volume *= 0.5;
                    }

                    self.node_volumes[[i, j, k]] = volume;
                }
            }
        }
    }

    pub fn solve_potential(&mut self, max_solver_iterations: usize, tolerance: f64) -> bool {
        let dx2 = 1.0 / (self.cell_spacings[0] * self.cell_spacings[0]);
        let dy2 = 1.0 / (self.cell_spacings[1] * self.cell_spacings[1]);
        let dz2 = 1.0 / (self.cell_spacings[2] * self.cell_spacings[2]);

        let dimensions = &self.dimensions;
        let phi = &mut self.potential;
        let rho = &self.charge_density;

        let mut residue_l2_norm = 0.0;
        let mut converged = false;

        // Iterating through mesh to solve potential.
        for iteration in 0..max_solver_iterations {
            for i in 1..dimensions.x - 1 {
                for j in 1..dimensions.y - 1 {
                    for k in 1..dimensions.z - 1 {
                        // Applying the Gauss-Seidel method.
                        let new_phi = ((rho[[i, j, k]] / PERMITTIVITY)
                            + dx2 * (phi[[i - 1, j, k]] + phi[[i + 1, j, k]])
                            + dy2 * (phi[[i, j - 1, k]] + phi[[i, j + 1, k]])
                            + dz2 * (phi[[i, j, k - 1]] + phi[[i, j, k + 1]]))
                            / (2.0 * dx2 + 2.0 * dy2 + 2.0 * dz2);

                        let current_phi = phi[[i, j, k]];

                        // Successive over-relaxation.
                        phi[[i, j, k]] = current_phi + 1.4 * (new_phi - current_phi);
                    }
                }
            }

            // Checking for convergence.
            if iteration != 0 && iteration % 25 == 0 {
                let mut sum = 0.0;

                for i in 1..dimensions.x - 1 {
                    for j in 1..dimensions.y - 1 {
                        for k in 1..dimensions.z - 1 {
                            let r = -phi[[i, j, k]] * (2.0 * dx2 + 2.0 * dy2 + 2.0 * dz2)
                                + (rho[[i, j, k]] / PERMITTIVITY)
                                + dx2 * (phi[[i - 1, j, k]] + phi[[i + 1, j, k]])
                                + dy2 * (phi[[i, j - 1, k]] + phi[[i, j + 1, k]])
                                + dz2 * (phi[[i, j, k - 1]] + phi[[i, j, k + 1]]);
                            sum += r * r;
                        }
                    }
                }

                residue_l2_norm =
                    (sum / (dimensions.x * dimensions.y * dimensions.z) as f64).sqrt();
                if residue_l2_norm < tolerance {
                    converged = true;

                    println!(
                        "Gauss-Seidel solver converged after {} iterations with an L2 norm of {}.",
                        iteration, residue_l2_norm
                    );

                    break;
                }
            }
        }

        if !converged {
            println!(
                "Gauss-Seidel solver failed to converge after {} iterations.",
                max_solver_iterations,
            );
        }

        converged
    }

    pub fn compute_electric_field(&mut self) {
        let dx = self.cell_spacings[0];
        let dy = self.cell_spacings[1];
        let dz = self.cell_spacings[2];

        let dimensions = &self.dimensions;
        let phi = &mut self.potential;

        for i in 0..dimensions.x {
            for j in 0..dimensions.y {
                for k in 0..dimensions.z {
                    let ef = &mut self.electric_field[[i, j, k]];

                    // Computing the x-component.
                    if i == 0 {
                        ef.x = -(-3.0 * phi[[i, j, k]] + 4.0 * phi[[i + 1, j, k]]
                            - phi[[i + 2, j, k]])
                            / (2.0 * dx);
                    } else if i == dimensions.x - 1 {
                        ef.x = -(phi[[i - 2, j, k]] - 4.0 * phi[[i - 1, j, k]]
                            + 3.0 * phi[[i, j, k]])
                            / (2.0 * dx);
                    } else {
                        ef.x = -(phi[[i + 1, j, k]] - phi[[i - 1, j, k]]) / (2.0 * dx);
                    }

                    // Computing the y-component.
                    if j == 0 {
                        ef.y = -(-3.0 * phi[[i, j, k]] + 4.0 * phi[[i, j + 1, k]]
                            - phi[[i, j + 2, k]])
                            / (2.0 * dy);
                    } else if j == dimensions.y - 1 {
                        ef.y = -(phi[[i, j - 2, k]] - 4.0 * phi[[i, j - 1, k]]
                            + 3.0 * phi[[i, j, k]])
                            / (2.0 * dy);
                    } else {
                        ef.y = -(phi[[i, j + 1, k]] - phi[[i, j - 1, k]]) / (2.0 * dy);
                    }

                    // Computing the z-component.
                    if k == 0 {
                        ef.z = -(-3.0 * phi[[i, j, k]] + 4.0 * phi[[i, j, k + 1]]
                            - phi[[i, j, k + 2]])
                            / (2.0 * dz);
                    } else if k == dimensions.z - 1 {
                        ef.z = -(phi[[i, j, k - 2]] - 4.0 * phi[[i, j, k - 1]]
                            + 3.0 * phi[[i, j, k]])
                            / (2.0 * dz);
                    } else {
                        ef.z = -(phi[[i, j, k + 1]] - phi[[i, j, k - 1]]) / (2.0 * dz);
                    }
                }
            }
        }
    }
}
