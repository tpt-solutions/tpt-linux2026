# tpt-l-sys-bindings

Shared, low-level C bindings for the Linux DRM/KMS and kernel UAPI surfaces used
by the hardware crates in this workspace.

- [`ffi`] — raw `bindgen`-generated bindings (generated on Linux hosts only).
- [`drm`] / [`kms`] — symbolic constant tables and safe helpers available on all
  platforms.

This crate is `publish = false`.

## License

MIT OR Apache-2.0.
