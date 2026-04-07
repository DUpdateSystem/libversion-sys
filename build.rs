use std::env;
use std::path::PathBuf;

fn main() {
    // Build libversion static library using cc.
    // The cmake-generated headers (config.h, export.h) are pre-committed under
    // generated/ — run `bash scripts/generate-headers.sh` to regenerate them
    // whenever the libversion submodule is updated.
    cc::Build::new()
        .file("libversion/libversion/compare.c")
        .file("libversion/libversion/private/compare.c")
        .file("libversion/libversion/private/parse.c")
        .include("libversion") // source headers (libversion/version.h, etc.)
        .include("generated") // pre-generated headers (libversion/config.h, libversion/export.h)
        .define("LIBVERSION_STATIC_DEFINE", None)
        .compile("version");

    // Generate FFI bindings via bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Igenerated") // pre-generated config.h, export.h
        .clang_arg("-Ilibversion") // source headers
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
