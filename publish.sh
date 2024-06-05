#!/bin/sh
set -ex
cargo build --workspace
cargo test --workspace
cargo fmt -- --check
cargo doc --workspace --all-features

cd core
cargo publish
cd ..
cd macros
cargo publish
cd ..
cargo publish
