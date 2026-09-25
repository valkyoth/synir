# Tool and source review — 2026-09-25

| Tool | Pin | Primary source |
| --- | --- | --- |
| Rust stable | 1.98.1 | [release announcements](https://blog.rust-lang.org/releases/) and [distribution manifest](https://static.rust-lang.org/dist/channel-rust-stable.toml) |
| cargo-deny | 0.20.2 | [official release](https://github.com/EmbarkStudios/cargo-deny/releases/tag/0.20.2) |
| cargo-audit | 0.22.2 | [official release](https://github.com/rustsec/rustsec/releases/tag/cargo-audit/v0.22.2) |
| actions/checkout | 7.0.1, full commit in workflows | [official release](https://github.com/actions/checkout/releases/tag/v7.0.1) |

Rust distribution metadata was fetched and compared with installed rustc.
Tool versions are rechecked by `cargo xtask freshness` using official Git tags
(the RustSec repository's generic latest release can name another product).
Direct crates.io API requests returned HTTP 403 during setup; do not infer crate
freshness from that unavailable endpoint. Synir admits no external crate versions.

Run freshness weekly (scheduled CI) and before tool changes or any release.
Review upstream changelogs, MSRV, action permissions and pin changes; rerun all
relevant checks and commit updated evidence. A network failure is not freshness.
Dependabot checks Cargo and GitHub Actions weekly; normal dependency proposals
remain forbidden by the first-party graph gate. cargo-deny/audit are installed
executables, not Synir dependencies. Their graphs remain external tool trust.

Architecture sources rechecked during setup:
[procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html),
[native delimiter behavior](https://doc.rust-lang.org/proc_macro/enum.Delimiter.html),
[Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
Before source parsing/Unicode implementation, pin exact Rust Reference and
Unicode data revisions, checksums and redistribution notices. No Unicode tables
are currently vendored. Avoid attributing behavioral guarantees to mere no_std
headers, or treating compiler-native spans as serializable source offsets.
