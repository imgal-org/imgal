use pyo3::prelude::*;

use crate::functions::filters_functions;
use crate::utils::py_import_module;

// Python bindings for the "filters" submodule
pub fn register_filters_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let filters_module = PyModule::new(parent_module.py(), "filters")?;

    // add module to Python's sys.modules
    py_import_module("filters");

    // add filters submodule functions
    filters_module.add_function(wrap_pyfunction!(
        filters_functions::filters_fft_convolve,
        &filters_module
    )?)?;

    // attach to parent module
    parent_module.add_submodule(&filters_module)
}
