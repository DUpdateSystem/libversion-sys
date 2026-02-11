use std::env;
use std::path::PathBuf;

fn main() {
    // Build libversion static library using cmake
    let dst = cmake::Config::new("libversion")
        .build_target("libversion_static")
        .build();

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
