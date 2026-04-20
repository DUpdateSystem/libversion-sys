# libversion-sys

[![Crates.io](https://img.shields.io/crates/v/libversion-sys.svg)](https://crates.io/crates/libversion-sys)
[![docs.rs](https://docs.rs/libversion-sys/badge.svg)](https://docs.rs/libversion-sys)
[![License: MIT](https://img.shields.io/crates/l/libversion-sys.svg)](LICENSE)

Rust FFI bindings to [libversion](https://github.com/repology/libversion), an advanced version string comparison library.

By default the crate vendors the C source and builds it directly, so no system-level installation of libversion is required. If you prefer linking an installed system copy, disable default features and make sure `pkg-config` can find `libversion`.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
libversion-sys = "0.2"
```

Use the default vendored build:

```toml
[dependencies]
libversion-sys = "0.2"
```

Or link a system-installed `libversion`:

```toml
[dependencies]
libversion-sys = { version = "0.2", default-features = false }
```

### Safe API

```rust
use std::cmp::Ordering;
use libversion_sys::{compare, compare_with_flags, VERSIONFLAG_P_IS_PATCH};

assert_eq!(compare("1.0", "1.1"), Ordering::Less);
assert_eq!(compare("1.0", "1.0.0"), Ordering::Equal);
assert_eq!(compare("1.0alpha1", "1.0"), Ordering::Less);

// "p" as patch (post-release) instead of pre-release
assert_eq!(
    compare_with_flags("1.0p1", "1.0", VERSIONFLAG_P_IS_PATCH, 0),
    Ordering::Greater,
);
```

### Raw FFI

```rust
use std::ffi::CString;
use libversion_sys::ffi;

let v1 = CString::new("1.0").unwrap();
let v2 = CString::new("2.0").unwrap();
let result = unsafe { ffi::version_compare2(v1.as_ptr(), v2.as_ptr()) };
assert_eq!(result, -1);
```

### Version metadata

```rust
assert!(libversion_sys::version_atleast(3, 0, 0));
assert!(!libversion_sys::version_string().is_empty());
```

## Flags

| Flag | Description |
|------|-------------|
| `VERSIONFLAG_P_IS_PATCH` | Treat `p` as patch (post-release) instead of pre-release |
| `VERSIONFLAG_ANY_IS_PATCH` | Treat any letter sequence as post-release |
| `VERSIONFLAG_LOWER_BOUND` | Derive lowest possible version with the given prefix |
| `VERSIONFLAG_UPPER_BOUND` | Derive highest possible version with the given prefix |

## Build requirements

- Rust (stable)
- C compiler (gcc/clang) for the default vendored build
- libclang (for bindgen)
- `pkg-config` and a system `libversion` installation when building with `default-features = false`

On Ubuntu/Debian:

```sh
sudo apt-get install libclang-dev
```

For system linking:

```sh
sudo apt-get install pkg-config libversion-dev
```

`cmake` is only needed by maintainers when regenerating `generated/libversion/config.h` and `generated/libversion/export.h` after updating the vendored libversion source.

## License

MIT -- see [LICENSE](LICENSE).

The bundled [libversion](https://github.com/repology/libversion) is also MIT licensed.
