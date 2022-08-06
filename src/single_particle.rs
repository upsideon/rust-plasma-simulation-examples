use std::env;
use std::fs::File;
use std::io::Write;

use crate::constants::{ELECTRON_MASS, ELEMENTARY_CHARGE, PERMITTIVITY};

/// Maximum number of iterations for the potential solver.
const MAX_ITERATIONS: usize = 4000;

/// The number of iterations that the potential solver will wait before checking for convergence again.
const CONVERGENCE_CHECK_RATE: usize = 50;

/// The threshold of L2 norm residue by which potential convergence is defined.
const CONVERGENCE_TOLERANCE: f64 = 1e-6;

/// The change in simulation time per iteration.
const SIMULATION_TIMESTEP: f64 = 1e-10;

/// The number of timesteps executed by the simulation.
const NUM_SIMULATION_TIMESTEPS: usize = 5000;

/// Simulates a single electron oscillating in a 1-dimensional potential well.
pub fn simulate(num_mesh_nodes: usize) -> std::io::Result<()> {
    let mut potential = vec![0.0_f64; num_mesh_nodes];
    let mut charge_density = vec![ELEMENTARY_CHARGE * 1e12; num_mesh_nodes];
    let mut electric_field = vec![0.0_f64; num_mesh_nodes];

    let mesh_origin: f64 = 0.0;
    let mesh_end: f64 = 0.1;

    let dx = (mesh_end - mesh_origin) / (num_mesh_nodes - 1) as f64;

    // Computing potential on mesh based on charge density.
    solve_potential(&mut potential, &mut charge_density, dx);

    // Computing electric field on mesh based on the potential.
    compute_electric_field(&mut potential, &mut electric_field, dx, true);

    // Defining the single particle as an electron.
    let mass = ELECTRON_MASS;
    let charge = -ELEMENTARY_CHARGE;
    let mut position = 4.0 * dx;
    let mut velocity = 0.0;
    let dt = SIMULATION_TIMESTEP;

    // Rewinding velocity by half a timestep so that explicit averaging
    // of velocities is not required when using the Leapfrog method.
    let logical_coordinate = position_to_logical_coordinate(position, dx, mesh_origin);
    let interpolated_electric_field = gather(logical_coordinate, &mut electric_field);
    velocity -= 0.5 * (charge / mass) * interpolated_electric_field * dt;

    // Retrieving the maximum potential for use in the potential energy calculation.
    let mut maximum_potential = potential[0];
    for i in 1..num_mesh_nodes {
        if potential[i] > maximum_potential {
            maximum_potential = potential[i];
        }
    }

    // Opening a file for writing trace information.
    let current_directory = env::current_dir()?;
    let trace_filepath = current_directory.join("trace.csv");
    let mut trace_file = File::create(trace_filepath).unwrap();

    // Writing CSV columns.
    writeln!(
        &mut trace_file,
        "time,position,velocity,kinetic_energy,potential_energy"
    )
    .unwrap();

    // Simulating motion of a single particle through an electric field.
    for ts in 1..=NUM_SIMULATION_TIMESTEPS {
        // Sampling mesh data at particle position.
        let logical_coordinate = position_to_logical_coordinate(position, dx, mesh_origin);
        let interpolated_electric_field = gather(logical_coordinate, &mut electric_field);

        // Integrating velocity and position.
        let previous_position = position;
        velocity += (charge / mass) * interpolated_electric_field * dt;
        position += velocity * dt;

        // Interpolating the potential at the average position.
        let average_position = 0.5 * (position + previous_position);
        let logical_average_position =
            position_to_logical_coordinate(average_position, dx, mesh_origin);
        let interpolated_average_potential = gather(logical_average_position, &mut potential);

        // Kinetic and potential energy are given in electron volts.
        let kinetic_energy = 0.5 * mass * velocity * velocity / ELEMENTARY_CHARGE;
        let potential_energy =
            charge * (interpolated_average_potential - maximum_potential) / ELEMENTARY_CHARGE;

        // Writing particle trace information to file.
        writeln!(
            &mut trace_file,
            "{},{},{},{},{}",
            ts as f64 * dt,
            position,
            velocity,
            kinetic_energy,
            potential_energy
        )
        .unwrap();

        // Printing particle information every 1000 timesteps.
        if ts == 1 || ts % 1000 == 0 {
            println!(
                "ts: {}, x: {}, v: {}, phi: {}, ke: {}, pe: {}, ef: {}",
                ts,
                position,
                velocity,
                interpolated_average_potential,
                kinetic_energy,
                potential_energy,
                interpolated_electric_field,
            );
        }
    }

    Ok(())
}

/// Solves the potential field.
fn solve_potential(potential: &mut Vec<f64>, charge_density: &mut Vec<f64>, dx: f64) {
    let dx2 = dx * dx;
    let relaxation_parameter: f64 = 1.4;
    let num_mesh_nodes = potential.len();

    // Iterating until convergence or the maximum number of iterations have been applied.
    for iteration in 0..MAX_ITERATIONS {
        // Specifying Dirichlet boundary conditions on edges of mesh.
        potential[0] = 0.0;
        potential[num_mesh_nodes - 1] = 0.0;

        // Applying the Gauss-Seidel method with Successive Over Relaxation (SOR).
        for i in 1..num_mesh_nodes - 1 {
            let gauss_seidel_approx = 0.5
                * (potential[i - 1] + potential[i + 1] + dx2 * charge_density[i] / PERMITTIVITY);
            potential[i] =
                potential[i] + relaxation_parameter * (gauss_seidel_approx - potential[i]);
        }

        // Checking for convergence every so many iterations based on check rate.
        if iteration != 0 && iteration % CONVERGENCE_CHECK_RATE == 0 {
            let mut residue_sum = 0.0;

            for i in 1..num_mesh_nodes - 1 {
                let residue_component = -charge_density[i] / PERMITTIVITY
                    - (potential[i - 1] - 2.0 * potential[i] + potential[i + 1]) / dx2;
                residue_sum += residue_component * residue_component;
            }

            let residue_l2_norm = (residue_sum).sqrt() / num_mesh_nodes as f64;

            // Convergence implies we've found a solution, so we return.
            if residue_l2_norm < CONVERGENCE_TOLERANCE {
                println!(
                    "Gauss-Seidel solver converged after {} iterations.",
                    iteration
                );

                return;
            }
        }
    }

    println!(
        "Gauss-Seidel solver failed to converge after {} iterations.",
        MAX_ITERATIONS
    );
}

/// Computes the electric field.
fn compute_electric_field(
    potential: &mut Vec<f64>,
    electric_field: &mut Vec<f64>,
    dx: f64,
    second_order_boundary_approx: bool,
) {
    let num_mesh_nodes = potential.len();

    // Applying the central finite difference method to internal nodes.
    for i in 1..num_mesh_nodes - 1 {
        electric_field[i] = -(potential[i + 1] - potential[i - 1]) / (2.0 * dx);
    }

    // Applying a one sided first or second order difference on boundaries.
    if second_order_boundary_approx {
        electric_field[0] = (3.0 * potential[0] - 4.0 * potential[1] + potential[2]) / (2.0 * dx);
        electric_field[num_mesh_nodes - 1] = (-potential[num_mesh_nodes - 3]
            + 4.0 * potential[num_mesh_nodes - 2]
            - 3.0 * potential[num_mesh_nodes - 1])
            / (2.0 * dx);
    } else {
        electric_field[0] = (potential[0] - potential[1]) / dx;
        electric_field[num_mesh_nodes - 1] =
            (potential[num_mesh_nodes - 2] - potential[num_mesh_nodes - 1]) / dx;
    }
}

/// Converts a position to a logical coordinate.
fn position_to_logical_coordinate(position: f64, dx: f64, mesh_origin: f64) -> f64 {
    return (position - mesh_origin) / dx;
}

/// Interpolates field values at points between mesh nodes.
fn gather(logical_coordinate: f64, field: &mut Vec<f64>) -> f64 {
    let left_node_index = logical_coordinate.trunc() as usize;
    let right_node_index = left_node_index + 1;
    let fractional_distance = logical_coordinate.fract();

    return field[left_node_index] * (1.0 - fractional_distance)
        + field[right_node_index] * fractional_distance;
}

/// Outputs the simulation state.
fn _output_simulation_state(
    potential: &mut Vec<f64>,
    charge_density: &mut Vec<f64>,
    electric_field: &mut Vec<f64>,
    dx: f64,
) {
    println!("position,potential,charge_density,electric_field");

    for i in 0..potential.len() {
        let position = i as f64 * dx;
        println!(
            "{},{},{},{}",
            position, potential[i], charge_density[i], electric_field[i]
        );
    }
}
