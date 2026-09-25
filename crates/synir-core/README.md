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

# synir-core

The portable, allocation-independent foundation for Synir.

**Unpublished 0.1.0 foundation candidate. No usable syntax toolkit yet.**
This package is a scaffold, not a production release. No independent audit has
been completed. The registry links are reserved documentation destinations.

Always `no_std`. The empty-default and reserved `alloc` profiles compile independently of compiler-native APIs. Future modules own bounded tokens, parsing, diagnostics, schemas and emission. No syntax APIs exist yet.

## Dependency and security contract

Zero third-party Cargo dependencies, including build and development dependencies.
All first-party implementation code forbids unsafe Rust. Source files are at most
500 lines. Resource, completeness and provenance guarantees will be documented
per implemented API; no such parser guarantees are claimed by this scaffold.

## Use during development

Point a local workspace dependency at `crates/synir-core` to inspect the scaffold.
Do not use `cargo add` expecting an already published implementation.
The package will be published only after its release evidence and pentest pass.

## Rust version support

MSRV: **1.90.0**. Development and full checks: **1.98.1** (latest stable
verified 2026-09-25). Edition 2024; resolver 3. Compiler compatibility is
separate from the source grammar and Unicode versions Synir will understand.

| Rust compiler | Required verification |
| --- | --- |
| 1.90.0 | MSRV, minimal and all-feature workspace checks |
| 1.91.0–1.98.0 | Minimal and all-feature workspace checks |
| **1.98.1** | **Full lint, tests, docs, policy, package, platform and security checks** |

The upper version is a tested baseline, not a Cargo upper bound. Weekly review
and every release preflight check newer stable tools while preserving the MSRV.

## Scope and documentation

See the [project README](https://github.com/valkyoth/synir),
[capability status](https://github.com/valkyoth/synir/blob/main/docs/current-status.md),
[roadmap](https://github.com/valkyoth/synir/blob/main/docs/RELEASE_PLAN.md), and
[platform evidence](https://github.com/valkyoth/synir/blob/main/docs/PLATFORMS.md).
Linux, Windows, BSD, macOS, Android and iOS are planned/tested at the exact
profile scope recorded there; Aesynx remains a future portability target.

Licensed MIT OR Apache-2.0. See [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE).
