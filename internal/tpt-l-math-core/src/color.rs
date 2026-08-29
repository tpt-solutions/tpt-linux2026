//! Rec. 2100 (BT.2100) high-dynamic-range transfer functions and color helpers.
//!
//! All functions operate on signal values normalized to `[0.0, 1.0]` unless
//! otherwise noted. No standard library is required; the transcendental
//! functions come from `libm` so the crate stays `no_std`.

use libm::{exp, log, pow, sqrt};

/// PQ peak luminance in candela per square metre (ST.2084 reference white).
pub const PQ_PEAK_NITS: f64 = 10_000.0;

// ST.2084 (PQ) constants.
const PQ_M1: f64 = 2610.0 / 16384.0;
const PQ_M2: f64 = (2523.0 / 4096.0) * 128.0;
const PQ_C1: f64 = 3424.0 / 4096.0;
const PQ_C2: f64 = (2413.0 / 4096.0) * 32.0;
const PQ_C3: f64 = (2392.0 / 4096.0) * 32.0;

// BT.2100 (HLG) constants.
const HLG_A: f64 = 0.17883277;
const HLG_B: f64 = 0.28466892;
const HLG_C: f64 = 0.55991073;

/// Perceptual Quantization (PQ, SMPTE ST.2084) Electro-Optical Transfer
/// Function: converts a normalized code value in `[0, 1]` to a normalized
/// linear light value in `[0, 1]` (1.0 ≙ [`PQ_PEAK_NITS`]).
#[inline]
pub fn pq_eotf(code: f64) -> f64 {
    let c = code.clamp(0.0, 1.0);
    let p = pow(c, PQ_M1);
    pow((PQ_C1 + PQ_C2 * p) / (1.0 + PQ_C3 * p), PQ_M2)
}

/// PQ Optical-Electronic Transfer Function (inverse of [`pq_eotf`]).
#[inline]
pub fn pq_oetf(linear: f64) -> f64 {
    let l = linear.clamp(0.0, 1.0);
    let lp = pow(l, 1.0 / PQ_M2);
    let p = (lp - PQ_C1) / (PQ_C2 - PQ_C3 * lp);
    if p <= 0.0 { 0.0 } else { pow(p, 1.0 / PQ_M1) }
}

/// Hybrid Log-Gamma (HLG, BT.2100) OETF: linear light `[0, 1]` → display
/// signal `[0, 1]`.
#[inline]
pub fn hlg_oetf(linear: f64) -> f64 {
    let l = linear.clamp(0.0, 1.0);
    if l <= 1.0 / 12.0 { sqrt(3.0 * l) } else { HLG_A * log(12.0 * l - HLG_B) + HLG_C }
}

/// HLG EOTF: display signal `[0, 1]` → linear light `[0, 1]` (inverse of
/// [`hlg_oetf`]).
#[inline]
pub fn hlg_eotf(signal: f64) -> f64 {
    let o = signal.clamp(0.0, 1.0);
    if o <= 0.5 { o * o / 3.0 } else { (exp((o - HLG_C) / HLG_A) + HLG_B) / 12.0 }
}

/// A tri-stimulus linear-light (or signal) color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    /// Construct a color.
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Rgb { r, g, b }
    }

    /// Apply `f` to each channel, returning a new `Rgb`.
    pub fn map(self, f: impl Fn(f64) -> f64) -> Self {
        Rgb::new(f(self.r), f(self.g), f(self.b))
    }

    /// Apply the PQ OETF to each channel (linear → PQ signal).
    pub fn to_pq(self) -> Self {
        self.map(pq_oetf)
    }

    /// Apply the PQ EOTF to each channel (PQ signal → linear).
    pub fn from_pq(self) -> Self {
        self.map(pq_eotf)
    }

    /// Apply the HLG OETF to each channel.
    pub fn to_hlg(self) -> Self {
        self.map(hlg_oetf)
    }

    /// Apply the HLG EOTF to each channel.
    pub fn from_hlg(self) -> Self {
        self.map(hlg_eotf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pq_roundtrip() {
        for v in [1e-4, 0.05, 0.18, 0.5, 0.999, 1.0] {
            let code = pq_oetf(v);
            assert!((0.0..=1.0).contains(&code), "code out of range: {code}");
            let back = pq_eotf(code);
            assert!((back - v).abs() < 1e-6, "pq roundtrip {v} -> {back}");
        }
    }

    #[test]
    fn pq_peak() {
        assert!((pq_eotf(1.0) - 1.0).abs() < 1e-12);
        // PQ maps code 0 to a tiny but non-zero normalized value (~7e-7).
        assert!(pq_eotf(0.0) < 1e-5);
    }

    #[test]
    fn hlg_roundtrip() {
        for v in [0.0, 0.05, 1.0 / 12.0, 0.5, 0.8, 1.0] {
            let sig = hlg_oetf(v);
            let back = hlg_eotf(sig);
            assert!((back - v).abs() < 1e-9, "hlg roundtrip {v} -> {back}");
        }
    }

    #[test]
    fn hlg_boundary_continuous() {
        let lo = hlg_oetf(1.0 / 12.0 - 1e-6);
        let hi = hlg_oetf(1.0 / 12.0 + 1e-6);
        assert!((lo - hi).abs() < 1e-4, "hlg not continuous at 1/12");
    }

    #[test]
    fn rgb_channel_apply() {
        let c = Rgb::new(0.5, 0.25, 1.0).to_pq();
        assert!((pq_eotf(c.g) - 0.25).abs() < 1e-9);
    }
}
