use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::statistics::sum;

/// The imgal parent module.
#[pymodule(name = "imgal")]
fn py_mod_imgal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // register children modules
    reg_py_mod_statistics(m)?;
    Ok(())
}

/// Register function for the statistics submodule.
fn reg_py_mod_statistics(parent_mod: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_mod = PyModule::new(parent_mod.py(), "statistics")?;
    // add submodule functions
    child_mod.add_function(wrap_pyfunction!(py_fn_sum, &child_mod)?)?;
    // attach to parent module
    parent_mod.add_submodule(&child_mod)
}

/// Python binding for statistics::sum.
#[pyfunction]
#[pyo3(name = "sum")]
fn py_fn_sum(input: Vec<f64>) -> f64 {
    sum(&input)
}
