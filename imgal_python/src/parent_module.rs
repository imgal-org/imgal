use pyo3::prelude::*;

use super::child_modules::{
    distribution_module, filters_module, integration_module, parameters_module, phasor_module,
    simulation_module, statistics_module,
};

/// Python binding for the imgal parent module.
#[pymodule(name = "imgal_python")]
fn imgal_parent_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // register child modules
    distribution_module::register_distribution_module(m)?;
    filters_module::register_filters_module(m)?;
    integration_module::register_integration_module(m)?;
    parameters_module::register_parameters_module(m)?;
    phasor_module::register_phasor_module(m)?;
    simulation_module::register_simulation_module(m)?;
    statistics_module::register_statistics_module(m)?;
    Ok(())
}
