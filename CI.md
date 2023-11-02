# Cargo
To properly use `clippy` and execute tests for this library, you need to use a couple of important flags.

```
cargo clippy --all-features -- -W clippy:all -D warnings
```

This ensures that all compilation targets are checked, and that warnings present as errors - which is how the CI handles warnings.
As such, if you only run `cargo clippy` and get no warnings, there may still be warnings, which becomes errors when the CI checks the code.

It may also be a good idea to use the pedantic version, `clippy::pedantic` before opening a pull request, to get some last pointers. However, the intention of pedantic is not for all warnings to be solved.

```
cargo test --all-features
```

Similarly, this ensures that features are tested - again, this is what the CI does when it tests the code, so it should also be the default for any developer.
