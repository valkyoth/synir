# Verification strategy

## Baseline commands

`scripts/checks.sh` runs format, policy, strict Clippy, minimal/all-feature tests,
feature wiring checks, rustdoc and the self-contained core package verification.
`scripts/check-rust-version-matrix.sh` checks all released compilers from 1.90.0
through 1.98.1. `scripts/check-platforms.sh` checks portable profiles on all named
targets, including bare metal without std. None silently skips missing tools.

`cargo deny check` and `cargo audit --deny warnings` consume current advisory data.
`cargo xtask freshness` checks official Rust distribution metadata and stable
Git tags for installed-tool pins; it reports drift or unavailable sources and
never installs updates automatically. Dependency graph enforcement covers
normal/build/dev edges, all features and all targets, not just the default graph.

The Rust xtask checks first-party package paths, no registry/git lock entries,
source size, mandatory unsafe/no_std markers, shared README headers and local
Markdown file links, plus complete numbered roadmap fields and pentest exits. These are repository guardrails, not proofs of semantic
correctness or a malicious maintainer's inability to modify the guards.

## Evidence grows with implemented behavior

| Area | Required tests |
| --- | --- |
| Tokens/storage | Wrong-origin ranges, delimiter mismatches, exact/short capacity, checked arithmetic |
| Parsing | Complete consumption, parser progress, invalid tails, every field/variant, where/generic regressions |
| Budgets | Zero/exact/one-over limits; monotonic rollback; exhaustion during errors and emission |
| Attributes/literals | Unknown/duplicate/conflicting keys, raw/escaped strings, wrong forms, suffix/overflow |
| Unicode | Official XID and normalization vectors; bounded combining sequences; table regeneration |
| Emission/quote | Data injection, token fusion, precedence, punctuation, mismatched repetition, no partial success |
| Native integration | Real proc macros and rustc, macro_rules forwarding, spans, renamed crates, no_std consumers |
| Full grammar/edits | Explicit supported productions; stale revisions; overlap; UTF-8; no-op byte identity |
| Resource behavior | Deep clone/drop/debug, process limits, visits/bytes/allocation counts, deterministic hostile corpus |

Create a Rust/std fixture runner; no trybuild, proptest or libfuzzer dependency.
Test compile-pass and deliberate compile-fail consumers using structured/stable
error markers and locations. A direct call to proc_macro outside a macro is not
a native integration test. Use external independent parser/compiler executables
for differentials; store tool/version/input/expected-result provenance. A
semantic rustc error alone does not establish a syntax-parser disagreement.

Use bounded deterministic generators in ordinary tests, longer adversarial runs
in scheduled jobs, and replay every discovered issue. Tool availability is
reported explicitly; optional skipped experiments are never passed evidence.
Miri and external sanitizers/formal tools may be added with separately checked
pins when there is meaningful code to exercise; no proof claim comes from policy.

## First adversarial corpus

Before token parsing implementation, pin these ten named cases: nested generic
commas; fn arrows inside generic bounds; where without generics; tuple-struct
where placement; impl defaults; malformed helper value; duplicate key; escaped
and raw string mismatch; invisible-group precedence; rollback fuel exhaustion.
Each gets an owner release and runnable assertion as its API becomes available.
No corpus descriptor alone counts as an executed test.

## Claims

Compile success, runtime tests, fuzzing, coverage and independent review are
separate evidence classes. Publish compiler/profile/corpus/command/result,
resource thresholds and limitations. Benchmarks record cold/warm builds, wall
and CPU time, host memory, native import, emitted tokens and profile features.
No numeric coverage or speedup is currently established.
