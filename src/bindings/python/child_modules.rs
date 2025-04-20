use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use super::functions;

/// Python binding for the "integrate" submodule.
pub fn register_integrate_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "integrate")?;
    // add integrate submodule functions
    child_module.add_function(wrap_pyfunction!(functions::py_fn_integrate_composite_simpson, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(functions::py_fn_integrate_simpson, &child_module)?)?;
    // attach to parent module
    parent_module.add_submodule(&child_module)
}

/// Python binding for the "statistics" submodule.
pub fn register_statistics_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "statistics")?;
    // add statistics submodule functions
    child_module.add_function(wrap_pyfunction!(functions::py_fn_statistics_sum, &child_module)?)?;
    // attach to parent module
    parent_module.add_submodule(&child_module)
}
