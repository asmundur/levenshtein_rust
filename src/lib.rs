use pyo3::prelude::*;
use edit_distance::edit_distance;

#[pyfunction]
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    edit_distance(s1, s2)
}

#[pymodule]
fn levenshtein_rust(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein_distance, m)?)?;
    Ok(())
}
