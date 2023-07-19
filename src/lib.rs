mod api;
mod error;
use pyo3::prelude::*;

/// Dump a single tracker
#[pyfunction]
fn dump(
    path: String,                 // Path to tracker module
    destination: String,          // Folder to put ripped samples
    index_raw: Option<bool>,      // Preserve sample number
    index_padding: Option<usize>, // Set sample number padding
    index_only: Option<bool>,     // Only name sample by their number
    with_folder: Option<bool>,    // Store ripped samples in a self-contained folder
    upper: Option<bool>,          // Name samples in upper case
    lower: Option<bool>,          // Name samples in lower case
    strict: Option<bool>,         // filter by extension
    format: Option<String>,       // Format of exported samples
) -> PyResult<()> {
    api::rip(
        &path,
        destination,
        index_raw,
        index_padding,
        index_only,
        with_folder,
        upper,
        lower,
        strict,
        format,
    )
    .map_err(PyErr::from)
}

/// XMODITS python library
#[pymodule]
fn xmodits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    // m.add_function(wrap_pyfunction!(dump_multiple, m)?)?;

    Ok(())
}
