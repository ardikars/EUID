#!/usr/bin/sh

# Clean Project
cargo clean

cargo fmt
cargo clippy

# Code Coverage
rustup component add llvm-tools-preview
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="target/euid-%p-%m.profraw"
cargo test --lib
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

# Generate Docs
cargo doc

# Package Lib
# cargo package