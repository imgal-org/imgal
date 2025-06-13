use pyo3::prelude::*;

use crate::functions::parameters_functions;
use crate::utils::py_import_module;

/// Python binding for the "parameters" submodule.
pub fn register_parameters_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let parameters_module = PyModule::new(parent_module.py(), "parameters")?;

    // add module to python's sys.modules
    py_import_module("parameters");

    // add parameters submodule functions
    parameters_module.add_function(wrap_pyfunction!(
        parameters_functions::parameters_abbe_diffraction_limit,
        &parameters_module
    )?)?;
    parameters_module.add_function(wrap_pyfunction!(
        parameters_functions::parameters_omega,
        &parameters_module
    )?)?;

    // attach to parent module
    parent_module.add_submodule(&parameters_module)
}
