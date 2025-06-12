use pyo3::prelude::*;

use crate::functions::simulation_functions;
use crate::utils::py_import_module;


/// Python bindings for the "simulation" submodule.
pub fn register_simulation_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let simulation_module = PyModule::new(parent_module.py(), "simulation")?;
    let decay_module = PyModule::new(parent_module.py(), "decay")?;

    // add module to python's sys.modules
    py_import_module("simulation");
    py_import_module("simulation.decay");

    // add simulation::decay submodule functions
    decay_module.add_function(wrap_pyfunction!(
        simulation_functions::decay_fluorescence_decay_1d,
        &decay_module
    )?)?;

    // attach simulation submodule before attaching to the parent module
    simulation_module.add_submodule(&decay_module)?;
    parent_module.add_submodule(&simulation_module)
}
