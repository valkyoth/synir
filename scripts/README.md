# Verification commands

Run from the repository root. Shell scripts orchestrate Rust tools and the Brynja-derived Git report gate;
`tools/xtask` implements policy and online freshness review with no dependencies.

| Command | Scope |
| --- | --- |
| `scripts/checks.sh` | Offline local gate, tool tests, docs, core packaging |
| `scripts/check-rust-version-matrix.sh` | Checks and workspace tests on every compiler 1.90.0–1.98.1 |
| `scripts/check-platforms.sh` | Portable minimal and alloc target checks |
| `scripts/install-ci-tools.sh` | Explicit installation of pinned external executables |
| `scripts/release/validate-current-pentest.sh --required` | Require current committed passing report before release |
| `scripts/release/test-release-readiness.sh` | Disposable Git/signature/report regression cases |
| `cargo xtask policy` | Source size, graph, safety markers, local doc links |
| `cargo xtask freshness` | Online Rust/tag review; fails on drift/unavailable data |

Run `rustup toolchain install VERSION --profile minimal` and
`rustup target add TARGET --toolchain 1.98.1` for missing prerequisites.
The checks never silently install or skip a missing compiler/target.
Freshness uses `curl` and `git`; builds and ordinary local gates need no network.
The package allowlist in xtask is intentionally reviewed when adding a package
or advancing the coordinated workspace version.

Release gate tests require Bash, Git and ssh-keygen; they create only temporary
repositories and test keys. Ordinary CI exercises these fixtures without requiring
a report in Synir itself. See [the runbook](../docs/RELEASE_RUNBOOK.md).
