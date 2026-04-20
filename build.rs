use std::env;
use std::path::PathBuf;

fn main() {
    let include_paths = if env::var_os("CARGO_FEATURE_VENDORED").is_some() {
        build_vendored();
        vec![PathBuf::from("generated"), PathBuf::from("libversion")]
    } else {
        find_system_lib()
    };

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Consts)
        .allowlist_function("version_compare.*")
        .allowlist_var("VERSIONFLAG_.*")
        .allowlist_var("LIBVERSION_VERSION.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(
            include_paths
                .iter()
                .map(|path| format!("-I{}", path.display())),
        );

    if env::var_os("CARGO_FEATURE_VENDORED").is_some() {
        bindings = bindings.clang_arg("-DLIBVERSION_STATIC_DEFINE");
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_vendored() {
    // Build libversion from vendored C sources. The cmake-generated headers
    // (config.h, export.h) are pre-committed under generated/ so end users do
    // not need cmake installed for the default build.
    cc::Build::new()
        .file("libversion/libversion/compare.c")
        .file("libversion/libversion/private/compare.c")
        .file("libversion/libversion/private/parse.c")
        .include("libversion")
        .include("generated")
        .define("LIBVERSION_STATIC_DEFINE", None)
        .compile("version");
}

fn find_system_lib() -> Vec<PathBuf> {
    let library = pkg_config::Config::new()
        .atleast_version("3.0.0")
        .probe("libversion")
        .expect("failed to find system libversion via pkg-config; enable the vendored feature or install libversion.pc");

    library.include_paths
}
