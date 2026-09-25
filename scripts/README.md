# Verification commands

Run from the repository root. Shell scripts only orchestrate Rust/rustup tools;
`tools/xtask` implements policy and online freshness review with no dependencies.

| Command | Scope |
| --- | --- |
| `scripts/checks.sh` | Offline local gate, tool tests, docs, core packaging |
| `scripts/check-rust-version-matrix.sh` | Every released compiler 1.90.0–1.98.1 |
| `scripts/check-platforms.sh` | Portable minimal and alloc target checks |
| `scripts/install-ci-tools.sh` | Explicit installation of pinned external executables |
| `cargo xtask policy` | Source size, graph, safety markers, local doc links |
| `cargo xtask freshness` | Online Rust/tag review; fails on drift/unavailable data |

Run `rustup toolchain install VERSION --profile minimal` and
`rustup target add TARGET --toolchain 1.98.1` for missing prerequisites.
The checks never silently install or skip a missing compiler/target.
Freshness uses `curl` and `git`; builds and ordinary local gates need no network.
The package allowlist in xtask is intentionally reviewed when adding a package
or advancing the coordinated workspace version.
