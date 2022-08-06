use std::fs;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;
use std::string::String;

use crate::mesh::BoxMesh;
use crate::species::Species;

pub fn diagnostic(mesh: &BoxMesh, species: &Vec<Species>, iteration: usize) {
    let filepath = String::from("runtime_diags.csv");
    let path = Path::new(&filepath);

    let mut diagnostic_file = File::create(path).unwrap();
}

pub fn vtk_output(mesh: &BoxMesh, species: &Vec<Species>, file_index: usize) -> Result<()> {
    // Creating the results directory, if it doesn't exist.
    fs::create_dir_all("results")?;

    let filepath = format!("results/field_{:05}.vti", file_index);
    let path = Path::new(&filepath);

    let mut vti_file = File::create(path).unwrap();

    let dimensions = mesh.dimensions();
    let origin = mesh.origin();
    let cell_spacings = mesh.cell_spacings();

    // ImageData is the VTK format for structured Cartesian meshes.
    writeln!(&mut vti_file, "<VTKFile type=\"ImageData\">")?;

    write!(&mut vti_file, "<ImageData Origin=\"{}\" ", origin)?;
    write!(
        &mut vti_file,
        "Spacing=\"{} {} {}\" ",
        cell_spacings[0], cell_spacings[1], cell_spacings[2]
    )?;
    write!(
        &mut vti_file,
        "WholeExtent=\"0 {} 0 {} 0 {}\">\n",
        dimensions.x - 1,
        dimensions.y - 1,
        dimensions.z - 1
    )?;

    // Output data is stored on nodes (point data).
    writeln!(&mut vti_file, "<PointData>")?;

    // Writing node volumes.
    writeln!(
        &mut vti_file,
        "<DataArray Name=\"NodeVol\" NumberOfComponents=\"1\" format=\"ascii\" type=\"Float64\">"
    )?;
    writeln!(&mut vti_file, "{}", mesh.node_volumes())?;
    writeln!(&mut vti_file, "</DataArray>")?;

    // Writing potential.
    writeln!(
        &mut vti_file,
        "<DataArray Name=\"phi\" NumberOfComponents=\"1\" format=\"ascii\" type=\"Float64\">"
    )?;
    writeln!(&mut vti_file, "{}", mesh.potential())?;
    writeln!(&mut vti_file, "</DataArray>")?;

    // Writing charge density.
    writeln!(
        &mut vti_file,
        "<DataArray Name=\"rho\" NumberOfComponents=\"1\" format=\"ascii\" type=\"Float64\">"
    )?;
    writeln!(&mut vti_file, "{}", mesh.charge_density())?;
    writeln!(&mut vti_file, "</DataArray>")?;

    // Writing species number densities.
    for s in species {
        writeln!(
            &mut vti_file,
            "<DataArray Name=\"{}\" NumberOfComponents=\"1\" format=\"ascii\" type=\"Float64\">",
            s.name()
        )?;
        writeln!(&mut vti_file, "{}", s.number_density())?;
        writeln!(&mut vti_file, "</DataArray>")?;
    }

    // Writing electric field.
    writeln!(
        &mut vti_file,
        "<DataArray Name=\"ef\" NumberOfComponents=\"3\" format=\"ascii\" type=\"Float64\">"
    )?;
    writeln!(&mut vti_file, "{}", mesh.electric_field())?;
    writeln!(&mut vti_file, "</DataArray>")?;

    // Closing tags.
    writeln!(&mut vti_file, "</PointData>")?;
    writeln!(&mut vti_file, "</ImageData>")?;
    writeln!(&mut vti_file, "</VTKFile>")?;

    Ok(())
}
