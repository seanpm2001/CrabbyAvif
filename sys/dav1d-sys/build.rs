// Build rust library and bindings for dav1d.

use std::env;
use std::path::PathBuf;

extern crate pkg_config;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let build_target = std::env::var("TARGET").unwrap();
    let build_dir = if build_target.contains("android") {
        if build_target.contains("x86_64") {
            "build.android/x86_64"
        } else if build_target.contains("x86") {
            "build.android/x86"
        } else if build_target.contains("aarch64") {
            "build.android/aarch64"
        } else if build_target.contains("arm") {
            "build.android/arm"
        } else {
            panic!("Unknown target_arch for android. Must be one of x86, x86_64, arm, aarch64.");
        }
    } else {
        "build"
    };

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Prefer locally built dav1d if available.
    env::set_var(
        "PKG_CONFIG_PATH",
        format!("dav1d/{build_dir}/meson-uninstalled"),
    );
    let library = pkg_config::Config::new().probe("dav1d");
    if library.is_err() {
        println!(
            "dav1d could not be found with pkg-config. Install the system library or run dav1d.cmd"
        );
    }
    let library = library.unwrap();
    for lib in &library.libs {
        println!("cargo:rustc-link-lib={lib}");
    }
    for link_path in &library.link_paths {
        println!("cargo:rustc-link-search={}", link_path.display());
    }
    let mut include_str = String::new();
    for include_path in &library.include_paths {
        include_str.push_str("-I");
        include_str.push_str(include_path.to_str().unwrap());
    }

    // Generate bindings.
    let header_file = PathBuf::from(&project_root).join("wrapper.h");
    let outfile = PathBuf::from(&project_root).join("dav1d.rs");
    let mut bindings = bindgen::Builder::default()
        .header(header_file.into_os_string().into_string().unwrap())
        .clang_arg(include_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false)
        .generate_comments(false);
    let allowlist_items = &[
        "dav1d_close",
        "dav1d_data_unref",
        "dav1d_data_wrap",
        "dav1d_default_settings",
        "dav1d_error",
        "dav1d_get_picture",
        "dav1d_open",
        "dav1d_picture_unref",
        "dav1d_send_data",
    ];
    for allowlist_item in allowlist_items {
        bindings = bindings.allowlist_item(allowlist_item);
    }
    let bindings = bindings
        .generate()
        .unwrap_or_else(|_| panic!("Unable to generate bindings for dav1d."));
    bindings
        .write_to_file(outfile.as_path())
        .unwrap_or_else(|_| panic!("Couldn't write bindings for dav1d"));
    println!(
        "cargo:rustc-env=CRABBYAVIF_DAV1D_BINDINGS_RS={}",
        outfile.display()
    );
}
