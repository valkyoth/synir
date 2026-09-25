# Synir release plan to 1.0

Status: planning document; 0.1.0 is an unpublished foundation candidate.
Every numbered release is a small implementation or assurance pass. No dates,
maximum minor count, or production claims are implied. Split an oversized pass
before implementation; do not use patches to conceal new scope.

This plan expands the compressed ranges in [IDEA.md](IDEA.md). The final idea
clarification is binding: capability equivalence, not API/source compatibility.
The complete admitted toolkit includes full syntax and source transformations
before 1.0. Useful macro workflows arrive earlier as explicitly scoped pre-1.0
releases. Aesynx readiness preserves portability but waits for a real target.

## Mandatory setup and exit for every milestone

Each entry below explicitly includes Setup, Goal, Deliverables, Verification and
Exit criteria. Its predecessor is the dependency baseline, not a claim that a
future subsystem already exists. If an implementation discovers a missing
prerequisite, split/reorder unpublished passes before using it. All entries
inherit the following requirements:

- Start with a scope manifest: owning crate/modules, APIs, input/output/error
  contracts, exclusions, numeric work/memory bounds, fixture IDs and commands.
- Recheck latest stable Rust/tool pins and authoritative grammar/API sources.
  Preserve Rust 1.90.0 compatibility and all day-one portability boundaries.
- No third-party Cargo dependencies (even dev/build), no unsafe, no build
  scripts, and no code file over 500 lines. Check minimal/combined profiles.
- Deliver runnable positive, negative, exact-limit, one-over-limit and regression
  tests as applicable. Plans and mock-only interfaces are not implementations.
- Update README capability claims, support/source matrices, changelog and notes.
- Run `scripts/checks.sh`, compiler/platform matrices, security/freshness checks
  and milestone-specific tests. Record exact candidate and evidence.
- **Stop for a pentest of that exact candidate.** Fix findings, rerun checks and
  obtain retest evidence before any release. Every patch and RC also needs one.
  The setup has no completed pentest; no PASS is inferred from CI or policy tests.
- Follow [the release runbook](RELEASE_RUNBOOK.md). Signed tag and publication
  require the actual evidence and maintainer release authorization.

## Version map

| Versions | Detailed implementation passes |
| --- | --- |
| 0.1.0–0.10.0 | [Foundation and accounting](roadmap/foundation.md) |
| 0.11.0–0.20.0 | [Native tokens and portable views](roadmap/native.md) |
| 0.21.0–0.30.0 | [Declaration recognition and generic projections](roadmap/declarations.md) |
| 0.31.0–0.40.0 | [Literal decoding and typed attributes](roadmap/schemas.md) |
| 0.41.0–0.50.0 | [Fallible emission and usable macro profile](roadmap/emission.md) |
| 0.51.0–0.60.0 | [Quotation and generated schemas](roadmap/quotation.md) |
| 0.61.0–0.70.0 | [Source frontend and Unicode](roadmap/source.md) |
| 0.71.0–0.80.0 | [Expanded types, expressions and patterns](roadmap/syntax.md) |
| 0.81.0–0.90.0 | [Full items and traversal](roadmap/full.md) |
| 0.91.0–0.100.0 | [Owned syntax and lossless edits](roadmap/editing.md) |
| 0.101.0–0.110.0 | [Security, performance and production admission](roadmap/admission.md) |
| 1.0.0-rc.N | Exact stable candidate validation and retest below |
| 1.0.0 | First serious production-ready release below |

## 1.0.0-rc.N — exact candidate

**Setup:** Complete 0.110.0 and freeze the 1.0 manifests, admitted API/grammar,
Unicode data, adapter versions and artifacts. Additional RCs replace changed
candidates; review evidence cannot silently transfer across implementation changes.

**Goal:** Prove the exact intended stable release meets the declared scope.

**Deliverables:** Complete evidence bundle, package archives, documentation,
independent review closure and published support policy.

**Verification:** Re-run complete compiler/profile/platform and downstream tests,
archive rehearsal, source/Unicode reproduction, security and hostile-input gates.
Compare generated behavior and preserve benchmark/resource regression limits.

**Exit criteria:** Exact-candidate pentest and independent remediation retest
pass; CI/CodeQL are green; no open production blocker. If anything changes,
produce another RC and rerun affected evidence. Obtain release authorization.

## 1.0.0 — production admission

**Setup:** Use the admitted RC implementation unchanged; review any evidence-only
metadata difference and exact artifact relationship.

**Goal:** First serious production-ready Synir crate family for its explicit scope.

**Deliverables:** Stable no_std core, usable native macro profile, source/Unicode,
full grammar, transformations, validated docs, signed provenance and support policy.

**Verification:** Exact artifacts correspond to the reviewed candidate; package
installation and public examples work; all capability claims have linked evidence.

**Exit criteria:** Final pentest scope confirmation and release authorization;
no unreviewed code delta, unresolved release blocker, or fabricated platform claim.
Publish the signed release and verify registry contents. Ongoing maintenance
retains freshness checks, regression tests and per-version pentests.
