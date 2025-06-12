use pyo3::prelude::*;

use crate::functions::integration_functions;
use crate::utils::py_import_module;

/// Python binding for the "integrate" submodule.
pub fn register_integrate_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let integrate_module = PyModule::new(parent_module.py(), "integrate")?;

    // add module to python's sys.modules
    py_import_module("integrate");

    // add integrate submodule functions
    integrate_module.add_function(wrap_pyfunction!(
        integration_functions::integration_composite_simpson,
        &integrate_module
    )?)?;
    integrate_module.add_function(wrap_pyfunction!(
        integration_functions::integration_midpoint,
        &integrate_module
    )?)?;
    integrate_module.add_function(wrap_pyfunction!(
        integration_functions::integration_simpson,
        &integrate_module
    )?)?;

    // attach to parent module
    parent_module.add_submodule(&integrate_module)
}
