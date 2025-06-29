use pyo3::prelude::*;

use crate::functions::simulation_functions;
use crate::utils::py_import_module;

/// Python bindings for the "simulation" submodule.
pub fn register_simulation_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let simulation_module = PyModule::new(parent_module.py(), "simulation")?;
    let decay_module = PyModule::new(parent_module.py(), "decay")?;
    let instrument_module = PyModule::new(parent_module.py(), "instrument")?;

    // add module to python's sys.modules
    py_import_module("simulation");
    py_import_module("simulation.decay");
    py_import_module("simulation.instrument");

    // add simulation::decay submodule functions
    decay_module.add_function(wrap_pyfunction!(
        simulation_functions::decay_gaussian_fluorescence_1d,
        &decay_module
    )?)?;
    decay_module.add_function(wrap_pyfunction!(
        simulation_functions::decay_gaussian_fluorescence_3d,
        &decay_module
    )?)?;
    decay_module.add_function(wrap_pyfunction!(
        simulation_functions::decay_ideal_fluorescence_1d,
        &decay_module
    )?)?;
    decay_module.add_function(wrap_pyfunction!(
        simulation_functions::decay_ideal_fluorescence_3d,
        &decay_module
    )?)?;

    // add simulation::instrument submodule functions
    instrument_module.add_function(wrap_pyfunction!(
        simulation_functions::instrument_gaussian_irf_1d,
        &instrument_module
    )?)?;

    // attach simulation submodule before attaching to the parent module
    simulation_module.add_submodule(&decay_module)?;
    simulation_module.add_submodule(&instrument_module)?;
    parent_module.add_submodule(&simulation_module)
}
