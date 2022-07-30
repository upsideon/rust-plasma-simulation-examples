const ELEMENTARY_CHARGE: f64 = 1.602176565e-19;
const PERMITTIVITY: f64 = 8.85418782e-12;
const ELECTRON_MASS: f64 = 9.10938215e-31;

const MAX_ITERATIONS: usize = 1000;
const CONVERGENCE_CHECK_RATE: usize = 50;
const CONVERGENCE_TOLERANCE: f64 = 1e-6;

pub fn simulate(num_mesh_nodes: usize) {
    let mut potential = vec![0.0_f64; num_mesh_nodes];
    let mut charge_density = vec![ELEMENTARY_CHARGE; num_mesh_nodes];
    let mut electric_field = vec![0.0_f64; num_mesh_nodes];

    let mesh_origin: f64 = 0.0;
    let mesh_end: f64 = 0.1;

    let node_spacing = (mesh_end - mesh_origin) / (num_mesh_nodes - 1) as f64;

    solve_potential(
        &mut potential,
        &mut charge_density,
        &mut electric_field,
        node_spacing,
    );

    compute_electric_field(&mut potential, &mut electric_field, node_spacing, true);

    output_simulation_state(
        &mut potential,
        &mut charge_density,
        &mut electric_field,
        node_spacing,
    );
}

fn solve_potential(
    potential: &mut Vec<f64>,
    charge_density: &mut Vec<f64>,
    electric_field: &mut Vec<f64>,
    dx: f64,
) {
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

            let residue_l2_norm = (residue_sum / num_mesh_nodes as f64).sqrt();

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

fn compute_electric_field(
    potential: &mut Vec<f64>,
    electric_field: &mut Vec<f64>,
    dx: f64,
    first_order_boundary_approx: bool,
) {
    let num_mesh_nodes = potential.len();

    // Applying the central finite difference method to internal nodes.
    for i in 1..num_mesh_nodes - 1 {
        electric_field[i] = -(potential[i + 1] - potential[i - 1]) / 2.0 * dx;
    }

    // Applying a one sided first or second order difference on boundaries.
    if first_order_boundary_approx {
        electric_field[0] = (potential[0] - potential[1]) / dx;
        electric_field[num_mesh_nodes - 1] =
            (potential[num_mesh_nodes - 2] - potential[num_mesh_nodes - 1]) / dx;
    } else {
        electric_field[0] = (3.0 * potential[0] - 4.0 * potential[1] + potential[2]) / 2.0 * dx;
        electric_field[num_mesh_nodes - 1] = (-potential[num_mesh_nodes - 3]
            + 4.0 * potential[num_mesh_nodes - 2]
            - 3.0 * potential[num_mesh_nodes - 1])
            / 2.0
            * dx;
    }
}

fn output_simulation_state(
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
