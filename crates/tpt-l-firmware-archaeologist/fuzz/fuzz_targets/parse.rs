#![no_main]

use libfuzzer_sys::fuzz_target;
use tpt_l_firmware_archaeologist::acpi::{AcpiTable, Fadt};
use tpt_l_firmware_archaeologist::intel_me::FptHeader;
use tpt_l_firmware_archaeologist::uefi::{FvHeader, PeImage};

fuzz_target!(|data: &[u8]| {
    // The parsers must never panic on arbitrary input; they should return
    // `Err` (or a best-effort value) instead.
    let _ = AcpiTable::parse(data, 0);
    let _ = Fadt::parse(data, 0);
    let _ = FvHeader::parse(data, 0);
    let _ = PeImage::parse(data);
    let _ = FptHeader::parse(data, 0);
});
