use std::env;
use std::path::{Path, PathBuf};

/// When cross-compiling for Android, the `cmake` crate (cmake-rs) sets
/// `CMAKE_SYSTEM_NAME=Android` but does **not** set `CMAKE_ANDROID_NDK`.
/// CMake ≥ 3.21's `Platform/Android-Determine.cmake` then fails because it
/// cannot locate the NDK.
///
/// Detection strategy:
///   1. Check well-known environment variables (`ANDROID_NDK_ROOT`, etc.).
///   2. Infer from the C compiler path that `cc` selects for this target.
///      NDK compilers live at `<NDK>/toolchains/llvm/prebuilt/<host>/bin/…`,
///      so we walk up from the compiler looking for the `toolchains` dir.
fn detect_android_ndk() -> Option<PathBuf> {
    // 1. Prefer explicit env vars (same ones CMake itself checks)
    for var in ["ANDROID_NDK_ROOT", "ANDROID_NDK_HOME", "ANDROID_NDK"] {
        if let Ok(val) = env::var(var) {
            let p = PathBuf::from(&val);
            if p.is_dir() {
                return Some(p);
            }
        }
    }

    // 2. Infer from the C compiler path
    let compiler = cc::Build::new()
        .cargo_metadata(false)
        .opt_level(0)
        .warnings(false)
        .try_get_compiler()
        .ok()?;
    let cc_path = compiler.path().canonicalize().ok()?;
    let mut dir: &Path = cc_path.parent()?;
    loop {
        if dir.file_name().and_then(|n| n.to_str()) == Some("toolchains")
            && dir.join("llvm").is_dir()
        {
            return dir.parent().map(|p| p.to_path_buf());
        }
        dir = dir.parent()?;
    }
}

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // Build libversion static library using cmake
    let mut cmake_cfg = cmake::Config::new("libversion");
    cmake_cfg.build_target("libversion_static");

    // Work around cmake-rs not setting CMAKE_ANDROID_NDK for Android targets.
    if target_os == "android" {
        if let Some(ndk_root) = detect_android_ndk() {
            cmake_cfg.define("CMAKE_ANDROID_NDK", &ndk_root);
        }
    }

    let dst = cmake_cfg.build();

    let build_dir = dst.join("build").join("libversion");
    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=version");

    // Generate FFI bindings via bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // cmake-generated headers (config.h, export.h) are in build/libversion/
        .clang_arg(format!("-I{}", dst.join("build").display()))
        // source headers are under libversion/ (the submodule root)
        .clang_arg("-Ilibversion")
        // static build: LIBVERSION_EXPORT expands to nothing
        .clang_arg("-DLIBVERSION_STATIC_DEFINE")
        .default_enum_style(bindgen::EnumVariation::Consts)
        .allowlist_function("version_compare.*")
        .allowlist_var("VERSIONFLAG_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
