//! Rec. 2100 tone-mapping helpers built on [`tpt-l-math-core`].

use tpt_l_math_core::color::{self};

/// Re-export the linear-light color type so callers can use
/// `tpt_l_wayland_hdr_vrr_kit::tonemap::Rgb`.
pub use tpt_l_math_core::color::Rgb;

/// A tone-mapping operator that converts HDR scene-referred linear light into a
/// display-encoded signal (PQ or HLG) for a target EOTF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToneCurve {
    /// Scene-linear -> PQ (ST.2084) signal.
    Pq,
    /// Scene-linear -> HLG (BT.2100) signal.
    Hlg,
}

/// Map a scene-referred linear-light color to a display signal.
pub fn encode(curve: ToneCurve, linear: Rgb) -> Rgb {
    match curve {
        ToneCurve::Pq => linear.to_pq(),
        ToneCurve::Hlg => linear.to_hlg(),
    }
}

/// Inverse: display signal -> scene-linear light.
pub fn decode(curve: ToneCurve, signal: Rgb) -> Rgb {
    match curve {
        ToneCurve::Pq => signal.from_pq(),
        ToneCurve::Hlg => signal.from_hlg(),
    }
}

/// Apply a simple per-channel highlight roll-off (Reinhard-ish) to keep values
/// within a `[0, 1]` display range before EOTF encoding.
pub fn roll_off(linear: Rgb, peak: f64) -> Rgb {
    let f = |v: f64| -> f64 {
        let x = v / peak;
        (x / (1.0 + x)).clamp(0.0, 1.0)
    };
    Rgb::new(f(linear.r), f(linear.g), f(linear.b))
}

/// Convenience: linear scene color -> chosen display signal in one step.
pub fn to_display(curve: ToneCurve, linear: Rgb, peak: f64) -> Rgb {
    encode(curve, roll_off(linear, peak))
}

/// Name of the EOTF for diagnostics.
pub fn curve_name(curve: ToneCurve) -> &'static str {
    match curve {
        ToneCurve::Pq => "PQ (SMPTE ST.2084)",
        ToneCurve::Hlg => "HLG (BT.2100)",
    }
}

/// Re-export the underlying color math for downstream crates.
pub use color::PQ_PEAK_NITS;
