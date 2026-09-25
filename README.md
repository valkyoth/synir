<p align="center">
  <b>Security-first Rust syntax and macro tooling with a no_std core.</b><br>
  First-party crates, explicit validation, bounded work, and small reviewed releases.
</p>

<div align="center">
  <a href="https://crates.io/crates/synir">Crates.io</a> |
  <a href="https://docs.rs/synir">Docs.rs</a> |
  <a href="https://github.com/valkyoth/synir/blob/main/docs/RELEASE_PLAN.md">Release Plan</a> |
  <a href="https://github.com/valkyoth/synir/blob/main/docs/threat-model.md">Threat Model</a> |
  <a href="https://github.com/valkyoth/synir/blob/main/SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/synir">
    <img src="https://raw.githubusercontent.com/valkyoth/synir/main/.github/images/synir.webp" alt="Synir Rust syntax and macro toolkit">
  </a>
</p>

# Synir

Synir is a clean-slate Rust syntax and code-generation toolkit in development.
It aims to cover declaration parsing, typed attributes, fallible token emission,
quotation, full syntax, traversal, and source transformations through a unified
API. Compatibility with existing parser or macro-library APIs is not a goal.

**Current status: unpublished 0.1.0 foundation candidate.** Workspace boundaries,
checks and plans exist. No lexer, parser, attribute decoder, quotation macro,
or source transformer is implemented. No production-readiness or independent
audit claim is made. The registry links above are future package destinations.

## Workspace

| Crate | Boundary | Current state |
| --- | --- | --- |
| `synir` | Small no_std facade; optional host integration | Scaffold |
| `synir-core` | no_std, allocator-independent foundation | Scaffold |
| `synir-host` | Host/compiler invocation and span ownership | Scaffold |
| `synir-macros` | Optional proc-macro helpers; no bootstrap cycle | Scaffold |
| `synir-xtask` | Unpublished Rust repository checks | Working tooling |

Zero third-party Cargo dependencies applies to normal, build, dev and optional
dependencies, including tooling and fixtures. Compiler-provided `core`, `alloc`,
`std`, and `proc_macro` belong to the Rust toolchain. No unsafe implementation
code or build scripts are permitted. Code files may not exceed 500 lines.

The crate-facing introduction lives in [crates/synir/README.md](crates/synir/README.md).
Features `alloc`, `proc-macro`, and `quote` currently establish dependency
boundaries only; they do not expose the planned capabilities.

## Rust version support

MSRV: **1.90.0**. Development and full checks: **1.98.1** (latest stable
verified 2026-09-25). Edition 2024; resolver 3. Compiler compatibility is
separate from the source grammar and Unicode versions Synir will understand.

| Rust compiler | Required verification |
| --- | --- |
| 1.90.0 | MSRV, minimal and all-feature workspace checks |
| 1.91.0, 1.91.1 | Minimal and all-feature workspace checks |
| 1.92.0 | Minimal and all-feature workspace checks |
| 1.93.0, 1.93.1 | Minimal and all-feature workspace checks |
| 1.94.0, 1.94.1 | Minimal and all-feature workspace checks |
| 1.95.0 | Minimal and all-feature workspace checks |
| 1.96.0, 1.96.1 | Minimal and all-feature workspace checks |
| 1.97.0, 1.97.1 | Minimal and all-feature workspace checks |
| 1.98.0 | Minimal and all-feature workspace checks |
| **1.98.1** | **Full lint, tests, docs, policy, package, platform and security checks** |

The upper version is a tested baseline, not a Cargo upper bound. Weekly review
and every release preflight check newer stable tools while preserving the MSRV.

## Platforms

Linux, Windows, BSD, macOS, Android and iOS are included in the day-one
portability contract. Core builds are OS-independent; host tooling and compiler
macros run on the compiler host. Cross compilation does not prove runtime support.
See [the platform matrix](docs/PLATFORMS.md) for exact targets and evidence.
Aesynx is a future target: no current toolchain or runtime support is claimed.

## Develop and verify

```sh
scripts/checks.sh
scripts/check-rust-version-matrix.sh
scripts/check-platforms.sh
cargo deny check
cargo audit --deny warnings
cargo xtask freshness
```

Rustup must have the listed toolchains/targets installed; missing prerequisites
fail explicitly. Standard local checks work offline. Online security and freshness
checks fail on unavailable upstream services rather than treating missing data
as a pass. CI includes Linux, Windows and macOS execution plus cross-target checks.
GitHub CodeQL uses **Default setup**, configured in repository settings.

## Project documents

- [Implementation plan](docs/IMPLEMENTATION_PLAN.md): architecture and work sequence.
- [Release plan](docs/RELEASE_PLAN.md): small numbered passes with pentest exits.
- [Version policy](docs/VERSION_PLAN.md) and [release runbook](docs/RELEASE_RUNBOOK.md).
- [Verification strategy](docs/VERIFICATION.md) and [current status](docs/current-status.md).
- [Original idea](docs/IDEA.md), preserved verbatim, including the final API clarification.
- [Changelog](CHANGELOG.md), [release notes](release-notes/README.md), and [security policy](SECURITY.md).

## License

MIT OR Apache-2.0, at your option. See [MIT](LICENSE-MIT) and
[Apache-2.0](LICENSE-APACHE).
