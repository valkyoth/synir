#!/usr/bin/env sh
set -eu
# Missing compilers fail. Install prerequisites explicitly with rustup first.
for compiler in 1.90.0 1.91.0 1.91.1 1.92.0 1.93.0 1.93.1 1.94.0 1.94.1 1.95.0 1.96.0 1.96.1 1.97.0 1.97.1 1.98.0 1.98.1; do
    cargo "+$compiler" check --locked --offline --workspace --no-default-features
    cargo "+$compiler" check --locked --offline --workspace --all-features
    cargo "+$compiler" test --locked --offline --workspace --no-default-features
    cargo "+$compiler" test --locked --offline --workspace --all-features
done
