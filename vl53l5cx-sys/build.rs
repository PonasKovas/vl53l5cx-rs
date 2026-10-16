use std::{env, path::PathBuf};

// Feature `foo-bar` <-> macro VL53L5CX_DISABLE_FOO_BAR (defined when the feature is OFF).
const OUTPUTS: &[&str] = &[
    "AMBIENT_PER_SPAD",
    "NB_SPADS_ENABLED",
    "NB_TARGET_DETECTED",
    "SIGNAL_PER_SPAD",
    "RANGE_SIGMA_MM",
    "DISTANCE_MM",
    "REFLECTANCE_PERCENT",
    "TARGET_STATUS",
];

fn main() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let include = dir.join("include");
    let vendor = dir.join("vendor");
    let csrc = dir.join("csrc");

    let feature = |name: &str| env::var_os(format!("CARGO_FEATURE_{name}")).is_some();

    let mut defines: Vec<(String, Option<String>)> = Vec::new();
    for out in OUTPUTS {
        if !feature(out) {
            defines.push((format!("VL53L5CX_DISABLE_{out}"), None));
        }
    }
    // Motion indicator is a plugin; not part of core.
    defines.push(("VL53L5CX_DISABLE_MOTION_INDICATOR".into(), None));

    let targets = if feature("TARGETS_4") {
        4
    } else if feature("TARGETS_3") {
        3
    } else if feature("TARGETS_2") {
        2
    } else {
        1
    };
    defines.push((
        "VL53L5CX_NB_TARGET_PER_ZONE".into(),
        Some(format!("{targets}U")),
    ));

    // ST's ULD is compiled untouched; only our platform.h/platform.c are ours.
    let mut build = cc::Build::new();
    build
        .file(vendor.join("vl53l5cx_api.c"))
        .file(csrc.join("platform.c"))
        .include(&include)
        .include(&vendor)
        .warnings(false);
    for (k, v) in &defines {
        build.define(k, v.as_deref());
    }
    build.compile("vl53l5cx_uld");

    let mut bg = bindgen::Builder::default()
        .header(include.join("wrapper.h").to_str().unwrap())
        .clang_arg(format!("-I{}", include.display()))
        .clang_arg(format!("-I{}", vendor.display()))
        .clang_arg("-ffreestanding")
        .clang_arg(format!("--target={}", env::var("TARGET").unwrap()))
        .use_core()
        .ctypes_prefix("core::ffi")
        .layout_tests(false)
        .allowlist_function("vl53l5cx_.*")
        .allowlist_type("VL53L5CX_(Configuration|ResultsData)")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    for (k, v) in &defines {
        bg = bg.clang_arg(match v {
            Some(v) => format!("-D{k}={v}"),
            None => format!("-D{k}"),
        });
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bg.generate()
        .expect("bindgen failed")
        .write_to_file(out.join("bindings.rs"))
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=csrc");
    println!("cargo:rerun-if-changed=vendor");
}
