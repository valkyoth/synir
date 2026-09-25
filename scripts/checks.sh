#!/usr/bin/env sh
set -eu
cargo fmt --all --check
cargo xtask policy
cargo clippy --locked --offline --workspace --all-targets --all-features -- -D warnings
cargo test --locked --offline --workspace --no-default-features
cargo test --locked --offline --workspace --all-features
for features in alloc proc-macro quote; do
    cargo check --locked --offline -p synir --no-default-features --features "$features"
done
RUSTDOCFLAGS="-D warnings" cargo doc --locked --offline --workspace --all-features --no-deps
# Core is independently packageable; sibling publication is intentionally not assumed.
cargo package --locked --offline --allow-dirty -p synir-core
