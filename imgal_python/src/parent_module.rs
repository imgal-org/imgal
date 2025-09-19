use pyo3::prelude::*;

use super::child_modules::{
    distribution_module, filter_module, integration_module, kernel_module, parameter_module,
    phasor_module, simulation_module, statistics_module,
};

/// Python binding for the imgal parent module.
#[pymodule(name = "imgal_python")]
fn imgal_parent_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // register child modules
    distribution_module::register_distribution_module(m)?;
    filter_module::register_filter_module(m)?;
    integration_module::register_integration_module(m)?;
    kernel_module::register_kernel_module(m)?;
    parameter_module::register_parameter_module(m)?;
    phasor_module::register_phasor_module(m)?;
    simulation_module::register_simulation_module(m)?;
    statistics_module::register_statistics_module(m)?;
    Ok(())
}
