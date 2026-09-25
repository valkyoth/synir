# Contributing to Synir

Contributions are licensed under MIT OR Apache-2.0, at the recipient's option.
Use the pinned stable compiler and run `scripts/checks.sh`,
`scripts/check-rust-version-matrix.sh`, and `scripts/check-platforms.sh`.
The host tooling is Rust with no external Cargo dependencies.

All code files must remain at most 500 lines. Split responsibilities into modules;
use separate crates for portability and trust boundaries. No unsafe code, build
scripts, third-party normal/build/dev dependencies, or networked code generation.
Every new behavior needs positive, negative, boundary, and regression tests.
Security-sensitive gate changes need tests that show rejection as well as success.

Follow [the implementation plan](../docs/IMPLEMENTATION_PLAN.md) and
[release runbook](../docs/RELEASE_RUNBOOK.md). Each release stops for an exact
candidate pentest. Local tests do not constitute independent review.
Report vulnerabilities privately via [SECURITY.md](../SECURITY.md).
