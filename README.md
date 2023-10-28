# Cargo
To properly use `clippy` and execute tests for this library, you need to use a couple of important flags.

```
cargo clippy --all-features --all-targets -- -D warnings
```

This ensures that all compilation targets are checked, and that warnings present as errors - which is how the CI handles warnings.
As such, if you only run `cargo clippy` and get no warnings, there may still be warnings, which becomes errors when the CI checks the code.

```
cargo test --all-targets --all-features
```

Similarly, this ensures that all compilation targets and paths are tested - again, this is what the CI does when it tests the code, so it should also be the default for any developer.

# Branching
## Naming
Branches should be named so that they can lead back to the issue they relate to, and so as to indicate the purpose of the branch.
This is done by using the following template `<issue-type>/<issue-number>`, for example:
- `feature/225`
- `bug/102`
- `documentation/155`

## Merging
Merges to main should only happen once all actions are completed and the issue is solved in its entirety.

# Safety
The entirety of the crate is written in safe rust, enforced with `#![forbid(unsafe_code)]`.

# Convention
The product should be developed with the [API guidelines](https://rust-lang.github.io/api-guidelines/about.html) in mind.