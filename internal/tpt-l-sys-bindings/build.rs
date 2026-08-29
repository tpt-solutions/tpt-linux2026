//! Build script: generate DRM/KMS/kernel bindings via `bindgen` on Linux targets.
//!
//! On non-Linux hosts (where kernel UAPI headers and `libclang` are typically
//! unavailable) we emit an empty stub so the crate still type-checks and can be
//! included in workspace-wide metadata operations.

use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(target_os = "linux")]
    {
        let _ = target_os;
        // A minimal UAPI header covering the DRM/KMS ioctls we wrap.
        let header = r#"
            #include <drm/drm.h>
            #include <drm/drm_mode.h>
            #include <linux/kvm.h>
        "#;
        let header_path = out_dir.join("tpt_bindings.h");
        std::fs::write(&header_path, header).expect("write header");

        let bindings = bindgen::Builder::default()
            .header(header_path.to_str().unwrap())
            .allowlist_type("drm_.*")
            .allowlist_type("drm_mode_.*")
            .allowlist_var("DRM_.*")
            .allowlist_var(".*_MODE_.*")
            .generate()
            .expect("unable to generate DRM/KMS bindings");

        bindings.write_to_file(out_dir.join("bindings.rs")).expect("write bindings");
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = target_os;
        // Fallback stub for non-Linux hosts.
        std::fs::write(out_dir.join("bindings.rs"), "// stub: bindings only generated on Linux\n")
            .expect("write stub bindings");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
