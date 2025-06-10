use pyo3::prelude::*;

use super::child_modules;

/// Python binding for the imgal parent module.
#[pymodule(name = "imgal_python")]
fn imgal_parent_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // register child modules
    child_modules::register_integrate_module(m)?;
    child_modules::register_parameters_module(m)?;
    child_modules::register_phasor_module(m)?;
    child_modules::register_statistics_module(m)?;
    child_modules::register_simulation_module(m)?;
    Ok(())
}
