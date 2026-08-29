#[cfg(test)]
mod tests {
    use tpt_l_wayland_hdr_vrr_kit::edid::{Edid, HdrStaticMetadata};
    use tpt_l_wayland_hdr_vrr_kit::tonemap::{self, Rgb, ToneCurve};

    /// Build a minimal valid EDID base block with one CEA extension carrying an
    /// HDR static metadata descriptor.
    fn make_edid() -> Vec<u8> {
        let mut buf = vec![0u8; 256];
        // Header.
        buf[0..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);
        // Manufacturer "ABC" -> 3 letters. Encode: A=1, B=2, C=3 => bits.
        let a = 1u16;
        let b = (b'B' - b'A' + 1) as u16;
        let c = (b'C' - b'A' + 1) as u16;
        let m: u16 = (a << 10) | (b << 5) | c;
        buf[8..10].copy_from_slice(&m.to_be_bytes());
        buf[10..12].copy_from_slice(&0x1234u16.to_le_bytes()); // product
        buf[12..16].copy_from_slice(&1u32.to_le_bytes()); // serial
        buf[18] = 1; // version major
        buf[19] = 4; // version minor

        // One extension: CEA-861 with HDR descriptor.
        buf[126] = 1; // extension count
        // Extension block at offset 128.
        buf[128] = 0x02; // CEA extension tag
        let dtd_start = 4 + 13; // header(4) + one 13-byte data block
        buf[130] = dtd_start as u8; // dtd start
        buf[131] = 0; // flags
        // Data block: extended tag (0x07) length 7, extended tag 0x87 (HDR),
        // eotf=2 (PQ), max lum 0x000F, favg 0x0005, min 0x00.
        let db = 128 + 4;
        buf[db] = (0x07 << 5) | 7; // tag=extended(7), length=7
        buf[db + 1] = 0x87; // HDR static metadata
        buf[db + 2] = 0x02; // eotf PQ
        buf[db + 3] = 0x0F; // max luminance LSB
        buf[db + 4] = 0x00; // max luminance MSB nibble
        buf[db + 5] = 0x05; // frame-avg luminance LSB
        buf[db + 6] = 0x00; // frame-avg luminance MSB nibble
        buf[db + 7] = 0x00; // min luminance

        // Fixup checksums (sum of each 128-byte block must be 0 mod 256).
        for blk in 0..2usize {
            let start = blk * 128;
            let mut sum: i32 = 0;
            for b in &buf[start..start + 128] {
                sum = sum.wrapping_add(*b as i32);
            }
            let adj = (-sum).rem_euclid(256) as u8;
            buf[start + 127] = adj;
        }
        buf
    }

    #[test]
    fn parse_base_block() {
        let buf = make_edid();
        let edid = Edid::parse(&buf).unwrap();
        assert_eq!(&edid.manufacturer, b"ABC");
        assert_eq!(edid.product_code, 0x1234);
        assert_eq!(edid.version_major, 1);
        assert_eq!(edid.version_minor, 4);
    }

    #[test]
    fn parses_hdr_metadata() {
        let buf = make_edid();
        let edid = Edid::parse(&buf).unwrap();
        assert_eq!(
            edid.hdr,
            Some(HdrStaticMetadata {
                eotf: 0x02,
                desired_max_luminance: 0x000F,
                desired_max_frame_avg_luminance: 0x0005,
                desired_min_luminance: 0x00,
            })
        );
    }

    #[test]
    fn reject_bad_header() {
        let mut buf = make_edid();
        buf[0] = 0x01;
        assert!(Edid::parse(&buf).is_err());
    }

    #[test]
    fn tonemap_pq_roundtrip() {
        let lin = Rgb::new(0.5, 0.25, 0.1);
        let sig = tonemap::encode(ToneCurve::Pq, lin);
        let back = tonemap::decode(ToneCurve::Pq, sig);
        assert!((back.r - lin.r).abs() < 1e-5);
        assert_eq!(tonemap::curve_name(ToneCurve::Pq), "PQ (SMPTE ST.2084)");
    }

    #[test]
    fn roll_off_clamps() {
        let lin = Rgb::new(5.0, 5.0, 5.0);
        let out = tonemap::roll_off(lin, 1.0);
        assert!(out.r <= 1.0 + 1e-9);
    }
}
