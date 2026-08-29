//! DRM (Direct Rendering Manager) constant tables and safe helpers.

/// DRM mode object types as used by `drm_mode_object` and atomic states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DrmObjectType {
    None = 0,
    Crtc = 0xAAAAAAAA,
    Connector = 0xBBBBBBBB,
    Encoder = 0xCCCCCCCC,
    Mode = 0xDDDDDDDD,
    Property = 0xEEEEEEEE,
    Fb = 0xFFFFFFFF,
    Blob = 0xB0B0B0B0,
    Plane = 0xB1B1B1B1,
}

/// DRM plane types (DRM_PLANE_TYPE_*).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PlaneType {
    Primary = 1,
    Cursor = 2,
    Overlay = 0,
}

/// Well-known connector type encodings (subset of `drm_connector_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ConnectorType {
    Unknown = 0,
    Vga = 1,
    DviI = 2,
    DviD = 3,
    DviA = 4,
    Composite = 5,
    Component = 6,
    HDMIA = 11,
    HDMIB = 12,
    EDP = 14,
    Dsi = 16,
}

/// Symbolic names for [`ConnectorType`].
pub fn connector_type_name(t: ConnectorType) -> &'static str {
    match t {
        ConnectorType::Unknown => "Unknown",
        ConnectorType::Vga => "VGA",
        ConnectorType::DviI => "DVI-I",
        ConnectorType::DviD => "DVI-D",
        ConnectorType::DviA => "DVI-A",
        ConnectorType::Composite => "Composite",
        ConnectorType::Component => "Component",
        ConnectorType::HDMIA => "HDMI-A",
        ConnectorType::HDMIB => "HDMI-B",
        ConnectorType::EDP => "eDP",
        ConnectorType::Dsi => "DSI",
    }
}

#[cfg(target_os = "linux")]
pub use crate::ffi;
