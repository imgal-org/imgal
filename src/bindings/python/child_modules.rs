use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use super::functions;

/// Python binding for the "integrate" submodule.
pub fn register_integrate_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let integrate_module = PyModule::new(parent_module.py(), "integrate")?;
    // add integrate submodule functions
    integrate_module.add_function(wrap_pyfunction!(functions::py_fn_integrate_composite_simpson, &integrate_module)?)?;
    integrate_module.add_function(wrap_pyfunction!(functions::py_fn_integrate_midpoint, &integrate_module)?)?;
    integrate_module.add_function(wrap_pyfunction!(functions::py_fn_integrate_simpson, &integrate_module)?)?;
    // attach to parent module
    parent_module.add_submodule(&integrate_module)
}

/// Python binding for the "phasor" submodule.
pub fn register_phasor_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let phasor_module = PyModule::new(parent_module.py(), "phasor")?;
    let time_domain_module = PyModule::new(parent_module.py(), "time_domain")?;
    // add phasor::time_domain submodule functions
    time_domain_module.add_function(wrap_pyfunction!(functions::py_fn_phasor_time_domain_imaginary, &time_domain_module)?)?;
    time_domain_module.add_function(wrap_pyfunction!(functions::py_fn_phasor_time_domain_real, &time_domain_module)?)?;
    // attach phasor submodule before attaching to the parent module
    phasor_module.add_submodule(&time_domain_module)?;
    parent_module.add_submodule(&phasor_module)
}

/// Python binding for the "statistics" submodule.
pub fn register_statistics_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let statistics_module = PyModule::new(parent_module.py(), "statistics")?;
    // add statistics submodule functions
    statistics_module.add_function(wrap_pyfunction!(functions::py_fn_statistics_sum, &statistics_module)?)?;
    // attach to parent module
    parent_module.add_submodule(&statistics_module)
}
