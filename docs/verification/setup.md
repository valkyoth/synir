# Setup verification — 2026-09-25

Scope: local working-tree 0.1.0 foundation candidate on x86_64 Linux, verified
before the foundation commit. No release tag was created. Results establish scaffold/tooling behavior,
not syntax implementation or independent security verification.

| Command / check | Observed result |
| --- | --- |
| `scripts/checks.sh` | PASS: format, policy, strict Clippy, minimal/all-feature tests, feature wiring, rustdoc, core package verification |
| `cargo test -p synir-xtask` through the local gate | PASS: 10 tests; dependency rejection, source-size boundary, build-script rejection, links, version parsing and roadmap completeness fields |
| `scripts/check-rust-version-matrix.sh` | PASS: minimal/all-feature workspace compilation on all 15 compiler releases listed below |
| `scripts/check-platforms.sh` | PASS: minimal and reserved alloc profiles on all 10 targets below |
| `cargo deny check` | PASS: advisories, bans, licenses and sources |
| `cargo audit --deny warnings` | PASS: five first-party packages; 1,271 advisory records loaded |
| `cargo xtask freshness` | PASS: official stable manifest and Git tags match Rust 1.98.1, cargo-deny 0.20.2, cargo-audit 0.22.2, checkout 7.0.1 |
| Original idea and existing image | Preserved; idea relocated to docs/IDEA.md |

Compilers: 1.90.0, 1.91.0, 1.91.1, 1.92.0, 1.93.0, 1.93.1, 1.94.0,
1.94.1, 1.95.0, 1.96.0, 1.96.1, 1.97.0, 1.97.1, 1.98.0, 1.98.1.
Full-check rustc reports `1.98.1 (48a229cea 2026-09-01)`.

Targets: x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu,
x86_64-pc-windows-msvc, aarch64-apple-darwin, x86_64-apple-darwin,
x86_64-unknown-freebsd, aarch64-linux-android, aarch64-apple-ios,
thumbv7em-none-eabihf, riscv32imac-unknown-none-elf.

The libraries contain no algorithm tests because no syntax APIs are implemented.
The ten tests exercise actual tooling behavior; compilation proves only the
current scaffold boundaries. Bare-metal builds do not prove future parser
boundedness. All-feature host builds intentionally include optional compiler crates.

## Outstanding release evidence

- Pentest of a frozen, committed candidate and any required remediation/retest.
- Actual GitHub CI/CodeQL results and confirmation of Default setup, private
  reporting and repository branch protections.
- Native Windows/macOS/FreeBSD and mobile device/runtime evidence; cross checks
  alone do not establish runtime support.
- Whole-family registry/archive rehearsal and publication. Core alone was
  packaged and verified locally; siblings are unpublished and no registry
  publication was attempted.

Crates.io metadata API access returned HTTP 403 during setup. Freshness used
successful official distribution/Git release sources instead; no third-party
crate version is admitted. No independent audit or production readiness is claimed.
