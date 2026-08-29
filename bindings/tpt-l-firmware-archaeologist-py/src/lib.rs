//! Python bindings for `tpt-l-firmware-archaeologist`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tpt_l_firmware_archaeologist::acpi::{Dsdt, Fadt, Ssdt};
use tpt_l_firmware_archaeologist::intel_me::FptHeader;
use tpt_l_firmware_archaeologist::uefi::{FvHeader, Machine, PeImage};

/// Parse an ACPI FADT and return its key fields as a dict.
#[pyfunction]
fn parse_fadt(py: Python<'_>, data: &[u8]) -> PyResult<Py<pyo3::types::PyDict>> {
    let fadt = Fadt::parse(data, 0).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("signature", String::from_utf8_lossy(&fadt.header.signature).to_string())?;
    dict.set_item("length", fadt.header.length)?;
    dict.set_item("firmware_ctrl", fadt.firmware_ctrl)?;
    dict.set_item("dsdt", fadt.dsdt)?;
    dict.set_item("preferred_pm_profile", fadt.preferred_pm_profile)?;
    Ok(dict.unbind())
}

/// Parse a DSDT/SSDT table, returning its signature and body length.
#[pyfunction]
fn parse_acpi_table(py: Python<'_>, data: &[u8], kind: &str) -> PyResult<Py<pyo3::types::PyDict>> {
    let dict = pyo3::types::PyDict::new(py);
    match kind {
        "dsdt" => {
            let t = Dsdt::parse(data, 0).map_err(|e| PyValueError::new_err(e.to_string()))?;
            dict.set_item("signature", "DSDT")?;
            dict.set_item("body_len", t.table.body.len())?;
        }
        "ssdt" => {
            let t = Ssdt::parse(data, 0).map_err(|e| PyValueError::new_err(e.to_string()))?;
            dict.set_item("signature", "SSDT")?;
            dict.set_item("body_len", t.table.body.len())?;
        }
        other => return Err(PyValueError::new_err(format!("unknown table: {other}"))),
    }
    Ok(dict.unbind())
}

/// Parse a UEFI Firmware Volume header.
#[pyfunction]
fn parse_firmware_volume(py: Python<'_>, data: &[u8]) -> PyResult<Py<pyo3::types::PyDict>> {
    let fv = FvHeader::parse(data, 0).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("fv_length", fv.fv_length)?;
    dict.set_item("header_length", fv.header_length)?;
    dict.set_item("revision", fv.revision)?;
    Ok(dict.unbind())
}

/// Parse a PE/COFF image header.
#[pyfunction]
fn parse_pe_image(py: Python<'_>, data: &[u8]) -> PyResult<Py<pyo3::types::PyDict>> {
    let pe = PeImage::parse(data).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let machine = match pe.machine {
        Machine::X64 => "x86_64",
        Machine::Aarch64 => "aarch64",
        Machine::Ia32 => "ia32",
        Machine::Unknown(_) => "unknown",
    };
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("machine", machine)?;
    dict.set_item("entry_point", pe.entry_point)?;
    dict.set_item("number_of_sections", pe.number_of_sections)?;
    dict.set_item("is_pe32_plus", pe.is_pe32_plus)?;
    Ok(dict.unbind())
}

/// Parse an Intel ME Flash Partition Table.
#[pyfunction]
fn parse_fpt(py: Python<'_>, data: &[u8]) -> PyResult<Py<pyo3::types::PyDict>> {
    let hdr = FptHeader::parse(data, 0).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let entries = hdr.entries(data, 0).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("num_entries", hdr.num_entries)?;
    let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    dict.set_item("partitions", names)?;
    Ok(dict.unbind())
}

#[pymodule]
fn tpt_l_firmware_archaeologist_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_fadt, m)?)?;
    m.add_function(wrap_pyfunction!(parse_acpi_table, m)?)?;
    m.add_function(wrap_pyfunction!(parse_firmware_volume, m)?)?;
    m.add_function(wrap_pyfunction!(parse_pe_image, m)?)?;
    m.add_function(wrap_pyfunction!(parse_fpt, m)?)?;
    Ok(())
}
