# libversion-sys

Rust FFI bindings to [libversion](https://github.com/repology/libversion), an advanced version string comparison library.

The C source is included as a Git submodule and compiled from source via CMake -- no system-level installation of libversion is required.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
libversion-sys = { git = "https://github.com/DUpdateSystem/libversion-sys" }
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

## Flags

| Flag | Description |
|------|-------------|
| `VERSIONFLAG_P_IS_PATCH` | Treat `p` as patch (post-release) instead of pre-release |
| `VERSIONFLAG_ANY_IS_PATCH` | Treat any letter sequence as post-release |
| `VERSIONFLAG_LOWER_BOUND` | Derive lowest possible version with the given prefix |
| `VERSIONFLAG_UPPER_BOUND` | Derive highest possible version with the given prefix |

## Build requirements

- Rust (stable)
- CMake
- C compiler (gcc/clang)
- libclang (for bindgen)

On Ubuntu/Debian:

```sh
sudo apt-get install cmake libclang-dev
```

## License

MIT -- see [LICENSE](LICENSE).

The bundled [libversion](https://github.com/repology/libversion) is also MIT licensed.
