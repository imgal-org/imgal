use std::ffi::CString;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use super::functions;

/// Python binding for the "integrate" submodule.
pub fn register_integrate_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let integrate_module = PyModule::new(parent_module.py(), "integrate")?;

    // add module to python's sys.modules
    py_import_module("integrate");

    // add integrate submodule functions
    integrate_module.add_function(wrap_pyfunction!(
        functions::py_fn_integrate_composite_simpson,
        &integrate_module
    )?)?;
    integrate_module.add_function(wrap_pyfunction!(
        functions::py_fn_integrate_midpoint,
        &integrate_module
    )?)?;
    integrate_module.add_function(wrap_pyfunction!(
        functions::py_fn_integrate_simpson,
        &integrate_module
    )?)?;

    // attach to parent module
    parent_module.add_submodule(&integrate_module)
}

/// Python binding for the "parameters" submodule.
pub fn register_parameters_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let parameters_module = PyModule::new(parent_module.py(), "parameters")?;

    // add module to python's sys.modules
    py_import_module("parameters");

    // add parameters submodule functions
    parameters_module.add_function(wrap_pyfunction!(
        functions::py_fn_parameters_omega,
        &parameters_module
    )?)?;

    // attach to parent module
    parent_module.add_submodule(&parameters_module)
}

/// Python binding for the "phasor" submodule.
pub fn register_phasor_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let phasor_module = PyModule::new(parent_module.py(), "phasor")?;
    let plot_module = PyModule::new(parent_module.py(), "plot")?;
    let time_domain_module = PyModule::new(parent_module.py(), "time_domain")?;

    // add module to python's sys.modules
    py_import_module("phasor");
    py_import_module("phasor.plot");
    py_import_module("phasor.time_domain");

    // add phasor::time_domain submodule functions
    time_domain_module.add_function(wrap_pyfunction!(
        functions::py_fn_phasor_time_domain_imaginary,
        &time_domain_module
    )?)?;
    time_domain_module.add_function(wrap_pyfunction!(
        functions::py_fn_phasor_time_domain_real,
        &time_domain_module
    )?)?;

    // add phasor::plot submodule functions
    plot_module.add_function(wrap_pyfunction!(
        functions::py_fn_phasor_plot_calibrate_imaginary,
        &plot_module
    )?)?;
    plot_module.add_function(wrap_pyfunction!(
        functions::py_fn_phasor_plot_calibrate_real,
        &plot_module
    )?)?;
    plot_module.add_function(wrap_pyfunction!(
        functions::py_fn_phasor_plot_multi_component_modulation,
        &plot_module
    )?)?;
    plot_module.add_function(wrap_pyfunction!(
        functions::py_fn_phasor_plot_multi_component_phi,
        &plot_module
    )?)?;
    plot_module.add_function(wrap_pyfunction!(
        functions::py_fn_phasor_plot_single_component_modulation,
        &plot_module
    )?)?;
    plot_module.add_function(wrap_pyfunction!(
        functions::py_fn_phasor_plot_single_component_phi,
        &plot_module
    )?)?;

    // attach phasor submodule before attaching to the parent module
    phasor_module.add_submodule(&time_domain_module)?;
    phasor_module.add_submodule(&plot_module)?;
    parent_module.add_submodule(&phasor_module)
}

/// Python binding for the "statistics" submodule.
pub fn register_statistics_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let statistics_module = PyModule::new(parent_module.py(), "statistics")?;

    // add module to python's sys.modules
    py_import_module("statistics");

    // add statistics submodule functions
    statistics_module.add_function(wrap_pyfunction!(
        functions::py_fn_statistics_sum,
        &statistics_module
    )?)?;

    // attach to parent module
    parent_module.add_submodule(&statistics_module)
}

/// Add a child module to Python's sys.modules dict.
///
/// # Description
///
/// This function manually adds a given module to Python's sys.modules
/// dict. This enables imports like `import imgal_python.parameters as params`.
///
/// # Arguments
///
/// * `module_name` - The name of the module to add to sys.modules.
fn py_import_module(module_name: &str) {
    let import_cmd = format!(
        "import sys; sys.modules['imgal_python.{}'] = '{}'",
        module_name, module_name
    );
    let c_str_cmd =
        CString::new(import_cmd).expect("Failed to create 'CString' module import command.");
    Python::with_gil(|py| {
        py.run(c_str_cmd.as_c_str(), None, None).unwrap();
    });
}
