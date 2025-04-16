use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::integrate;
use crate::statistics::sum;

/// The imgal parent module.
#[pymodule(name = "imgal")]
fn py_mod_imgal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // register children modules
    reg_py_mod_integrate(m)?;
    reg_py_mod_statistics(m)?;
    Ok(())
}

/// Register function for the integrate submodule
fn reg_py_mod_integrate(parent_mod: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_mod = PyModule::new(parent_mod.py(), "integrate")?;
    // add submodule functions
    child_mod.add_function(wrap_pyfunction!(
        py_fn_integrate_composite_simpson,
        &child_mod
    )?)?;
    child_mod.add_function(wrap_pyfunction!(py_fn_integrate_simpson, &child_mod)?)?;
    // attach to parent module
    parent_mod.add_submodule(&child_mod)
}

/// Register function for the statistics submodule.
fn reg_py_mod_statistics(parent_mod: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_mod = PyModule::new(parent_mod.py(), "statistics")?;
    // add submodule functions
    child_mod.add_function(wrap_pyfunction!(py_fn_sum, &child_mod)?)?;
    // attach to parent module
    parent_mod.add_submodule(&child_mod)
}

/// Python binding for integrate::compsite_simpson
#[pyfunction]
#[pyo3(name = "composite_simpson")]
fn py_fn_integrate_composite_simpson(y: Vec<f64>, delta_x: f64) -> f64 {
    integrate::composite_simpson(&y, delta_x)
}

/// Python binding for integrate::simpson
#[pyfunction]
#[pyo3(name = "simpson")]
fn py_fn_integrate_simpson(y: Vec<f64>, delta_x: f64) -> f64 {
    integrate::simpson(&y, delta_x).unwrap()
}

/// Python binding for statistics::sum.
#[pyfunction]
#[pyo3(name = "sum")]
fn py_fn_sum(input: Vec<f64>) -> f64 {
    sum(&input)
}
