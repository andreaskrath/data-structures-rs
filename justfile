# TO USE ANY OF THE FOLLOWING COMMANDS, YOU MUST INSTALL THE just PROGRAM.

# run test cases for all features
test:
    cargo test --all-features

# build a debug solution with all features
build:
    cargo build --all-features

# build a debug solution with all features
release:
    cargo build --all-features --release

# run the CI clippy command
clippy:
    cargo clippy --all-features -- -W clippy::all -D warnings

# run the pedantic clippy command
pedantic:
    cargo clippy --all-features -- -W clippy::pedantic
