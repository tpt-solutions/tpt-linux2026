//! Python bindings for `tpt-l-journal-oracle`.

use pyo3::prelude::*;
use tpt_l_journal_oracle::ingest::{parse_entry, parse_stream};
use tpt_l_journal_oracle::oracle::build_prompt;

/// Parse a journald export blob (entries separated by blank lines) into a list of
/// `{field: value}` dicts.
#[pyfunction]
fn parse_journal(py: Python<'_>, text: &str) -> PyResult<Py<pyo3::types::PyList>> {
    let entries = parse_stream(text);
    let list = pyo3::types::PyList::empty_bound(py);
    for e in &entries {
        let dict = pyo3::types::PyDict::new_bound(py);
        for (k, v) in &e.fields {
            dict.set_item(k, v)?;
        }
        list.append(dict)?;
    }
    Ok(list.unbind())
}

/// Build the plain-English diagnostic prompt for a journald export blob.
#[pyfunction]
fn diagnostic_prompt(text: &str) -> String {
    let entries = parse_stream(text);
    build_prompt(&entries)
}

/// Count entries in a journald export blob (handy smoke check from Python).
#[pyfunction]
fn count_entries(text: &str) -> usize {
    parse_stream(text).len()
}

#[pymodule]
fn tpt_l_journal_oracle_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_journal, m)?)?;
    m.add_function(wrap_pyfunction!(diagnostic_prompt, m)?)?;
    m.add_function(wrap_pyfunction!(count_entries, m)?)?;
    Ok(())
}
