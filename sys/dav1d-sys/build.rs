// Build rust library and bindings for dav1d.

use std::env;
use std::path::Path;
use std::path::PathBuf;

fn path_buf(inputs: &[&str]) -> PathBuf {
    let path: PathBuf = inputs.iter().collect();
    path
}

pub fn generate_bindings(header_file: PathBuf, version_dir: PathBuf, outfile: PathBuf) {
    let extra_includes_str = format!("-I{}", version_dir.display());
    let mut bindings = bindgen::Builder::default()
        .header(header_file.into_os_string().into_string().unwrap())
        .clang_arg(extra_includes_str)
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
}

fn add_native_library(
    library_dir: &str,
    library_path: PathBuf,
    header_file: PathBuf,
    extra_include_dir: PathBuf,
    bindings_path: PathBuf,
) {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let abs_library_dir = PathBuf::from(&project_root).join(library_dir);
    let abs_object_dir = PathBuf::from(&abs_library_dir).join(library_path);
    let object_file = format!("libdav1d.a");
    let library_file = PathBuf::from(&abs_object_dir).join(object_file);
    if !Path::new(&library_file).exists() {
        panic!("dav1d not found. Run dav1d.cmd.");
    }
    println!("cargo:rustc-link-search={}", abs_object_dir.display());
    println!("cargo:rustc-link-lib=static=dav1d");

    let abs_header_file = PathBuf::from(&abs_library_dir).join(header_file);
    let extra_includes = PathBuf::from(&abs_library_dir).join(extra_include_dir);
    generate_bindings(abs_header_file, extra_includes, bindings_path);
}

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

    add_native_library(
        "dav1d",
        path_buf(&[build_dir, "src"]),
        path_buf(&["include", "dav1d", "dav1d.h"]),
        path_buf(&[build_dir, "include", "dav1d"]),
        path_buf(&["src", "dav1d.rs"]),
    );
}
