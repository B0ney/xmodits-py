mod api;
mod error;
use error::Error;
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
    format: Option<String>,       // Format of exported samples
) -> PyResult<()> {
    api::rip_multiple(
        &[path],
        destination,
        index_raw,
        index_padding,
        index_only,
        with_folder,
        upper,
        lower,
        format,
    )
    .map_err(Error::py_err)
}

/// Dump multiple trackers
#[pyfunction]
fn dump_multiple(
    path: Vec<String>,
    destination: String,
    index_raw: Option<bool>,
    index_padding: Option<usize>,
    index_only: Option<bool>,
    with_folder: Option<bool>,
    upper: Option<bool>,
    lower: Option<bool>,
    format: Option<String>,
) -> PyResult<()> {
    api::rip_multiple(
        &path,
        destination,
        index_raw,
        index_padding,
        index_only,
        with_folder,
        upper,
        lower,
        format,
    )
    .map_err(Error::py_err)
}

/// XMODITS python library
#[pymodule]
fn xmodits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(dump_multiple, m)?)?;

    Ok(())
}
