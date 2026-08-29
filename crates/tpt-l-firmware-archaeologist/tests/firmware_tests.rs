#[cfg(test)]
mod tests {
    use tpt_l_firmware_archaeologist::acpi::{AcpiTable, Dsdt, Fadt, Ssdt};
    use tpt_l_firmware_archaeologist::intel_me::FptHeader;
    use tpt_l_firmware_archaeologist::uefi::{FvHeader, PeImage};

    // FFS file type RAW.
    const FFS_FILE_TYPE: u8 = 0x01;

    fn make_fadt() -> Vec<u8> {
        let mut buf = vec![0u8; 64];
        buf[0..4].copy_from_slice(b"FACP");
        buf[4..8].copy_from_slice(&64u32.to_le_bytes());
        buf[40..44].copy_from_slice(&0x1000u32.to_le_bytes());
        buf[44..48].copy_from_slice(&0x2000u32.to_le_bytes());
        buf[48] = 2;
        buf
    }

    #[test]
    fn parse_fadt() {
        let buf = make_fadt();
        let fadt = Fadt::parse(&buf, 0).unwrap();
        assert_eq!(fadt.firmware_ctrl, 0x1000);
        assert_eq!(fadt.dsdt, 0x2000);
        assert_eq!(fadt.preferred_pm_profile, 2);
        assert_eq!(&fadt.header.signature, b"FACP");
    }

    #[test]
    fn reject_wrong_acpi_signature() {
        let mut buf = make_fadt();
        buf[0..4].copy_from_slice(b"XXXX");
        assert!(Fadt::parse(&buf, 0).is_err());
    }

    #[test]
    fn parse_dsdt_ssdt() {
        // DSDT: header + a little AML body.
        let mut buf = vec![0u8; 48];
        buf[0..4].copy_from_slice(b"DSDT");
        buf[4..8].copy_from_slice(&48u32.to_le_bytes());
        let dsdt = Dsdt::parse(&buf, 0).unwrap();
        assert!(!dsdt.table.body.is_empty());

        let mut buf2 = vec![0u8; 48];
        buf2[0..4].copy_from_slice(b"SSDT");
        buf2[4..8].copy_from_slice(&48u32.to_le_bytes());
        let ssdt = Ssdt::parse(&buf2, 0).unwrap();
        assert_eq!(&ssdt.table.header.signature, b"SSDT");

        // A DSDT parser must reject an SSDT signature.
        assert!(Dsdt::parse(&buf2, 0).is_err());
    }

    #[test]
    fn acpi_table_length_bounds() {
        let mut buf = vec![0u8; 48];
        buf[0..4].copy_from_slice(b"TEST");
        buf[4..8].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        assert!(AcpiTable::parse(&buf, 0).is_err());
    }

    #[test]
    fn parse_firmware_volume_and_files() {
        let mut buf = vec![0u8; 256];
        // FV header.
        buf[32..40].copy_from_slice(&0x1000u64.to_le_bytes()); // fv_length
        buf[40..44].copy_from_slice(&0x4856_465Fu32.to_le_bytes()); // "_FVH"
        buf[48..50].copy_from_slice(&56u16.to_le_bytes()); // header_length
        // One FFS file right after the header.
        let file_off = 56;
        buf[file_off..file_off + 16].copy_from_slice(b"FILE123456789012"); // name
        buf[file_off + 18] = FFS_FILE_TYPE; // file_type
        buf[file_off + 20..file_off + 23].copy_from_slice(&24u32.to_le_bytes()[..3]); // size=24
        let fv = FvHeader::parse(&buf, 0).unwrap();
        assert_eq!(fv.header_length, 56);
        let files: Vec<_> = fv.files(&buf).collect();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_type, FFS_FILE_TYPE);
    }

    #[test]
    fn reject_bad_fv_signature() {
        let mut buf = vec![0u8; 64];
        buf[40..44].copy_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
        assert!(FvHeader::parse(&buf, 0).is_err());
    }

    #[test]
    fn parse_pe_coff() {
        let mut buf = vec![0u8; 0x100];
        buf[0..2].copy_from_slice(b"MZ");
        buf[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes()); // e_lfanew
        buf[0x80..0x84].copy_from_slice(&0x0000_4550u32.to_le_bytes()); // "PE\0\0"
        buf[0x84..0x86].copy_from_slice(&0x8664u16.to_le_bytes()); // X64
        buf[0x86..0x88].copy_from_slice(&1u16.to_le_bytes()); // 1 section
        buf[0x94..0x96].copy_from_slice(&0x00F0u16.to_le_bytes()); // opt header size
        buf[0x98..0x9A].copy_from_slice(&0x20Bu16.to_le_bytes()); // PE32+
        buf[0xA8..0xAC].copy_from_slice(&0x1000u32.to_le_bytes()); // entry point
        let pe = PeImage::parse(&buf).unwrap();
        assert_eq!(pe.machine, tpt_l_firmware_archaeologist::uefi::Machine::X64);
        assert!(pe.is_pe32_plus);
        assert_eq!(pe.entry_point, 0x1000);
        assert_eq!(pe.number_of_sections, 1);
    }

    #[test]
    fn reject_non_mz() {
        let buf = vec![0u8; 64];
        assert!(PeImage::parse(&buf).is_err());
    }

    #[test]
    fn parse_intel_me_fpt() {
        let mut buf = vec![0u8; 28 + 2 * 32];
        buf[0..4].copy_from_slice(&0x5F46_5054u32.to_le_bytes()); // "_FPT"
        buf[4..8].copy_from_slice(&2u32.to_le_bytes()); // num_entries
        buf[8..10].copy_from_slice(&1u16.to_le_bytes()); // header_version
        buf[12..14].copy_from_slice(&28u16.to_le_bytes()); // header_length
        // Entry 0
        buf[28..32].copy_from_slice(b"MEFS"); // name
        buf[28 + 8..28 + 12].copy_from_slice(&0x1000u32.to_le_bytes()); // offset
        buf[28 + 12..28 + 16].copy_from_slice(&0x2000u32.to_le_bytes()); // length
        // Entry 1
        let e1 = 28 + 32;
        buf[e1..e1 + 4].copy_from_slice(b"FTPR");
        buf[e1 + 8..e1 + 12].copy_from_slice(&0x3000u32.to_le_bytes());
        buf[e1 + 12..e1 + 16].copy_from_slice(&0x4000u32.to_le_bytes());

        let hdr = FptHeader::parse(&buf, 0).unwrap();
        assert_eq!(hdr.num_entries, 2);
        let entries = hdr.entries(&buf, 0).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "MEFS");
        assert_eq!(entries[0].offset, 0x1000);
        assert_eq!(entries[0].length, 0x2000);
        assert_eq!(entries[1].name, "FTPR");
    }

    #[test]
    fn reject_bad_fpt_signature() {
        let mut buf = vec![0u8; 28];
        buf[0..4].copy_from_slice(b"XXXX");
        assert!(FptHeader::parse(&buf, 0).is_err());
    }
}
