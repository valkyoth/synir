#!/usr/bin/env sh
set -eu
# Host/proc-macro crates are deliberately not compiled for the target OS.
for target in x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-msvc aarch64-apple-darwin x86_64-apple-darwin x86_64-unknown-freebsd aarch64-linux-android aarch64-apple-ios thumbv7em-none-eabihf riscv32imac-unknown-none-elf; do
    cargo check --locked --offline -p synir --no-default-features --target "$target"
    cargo check --locked --offline -p synir --no-default-features --features alloc --target "$target"
done
