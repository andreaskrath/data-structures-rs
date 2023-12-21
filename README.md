# Features
**json**: derives the serde `Serialize` and `Deserialize` on the provided data structures.

# Safety
The entirety of the crate is written in safe rust, enforced with:
```toml
[lints.rust]
unsafe_code = "forbid"
```

# Convention
The product should be developed with the [API guidelines](https://rust-lang.github.io/api-guidelines/about.html) in mind.

# Toolchain
Developers of this project should use stable rust, with a minimum version of 1.74 as lints are managed by the [Cargo.toml](Cargo.toml) file.
