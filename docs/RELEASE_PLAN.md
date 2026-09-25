# Synir release plan to 1.0

Status: planning document; 0.1.0 is an unpublished foundation candidate.
The audit revision expands the original 110 passes into 137 implementation or assurance passes.
The original-to-current version map is at the end of this document. No dates,
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
  This single roadmap is documentation and has no 500-line limit.
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

## Continuous evidence requirements

The grammar/edition baseline is pinned before the first declaration parser. Every
foundation primitive starts with in-crate positive, negative and budget tests.
The early runner then registers those fixtures; every native/parser family
registers its corpus and work/storage counters before its capability is admitted. Quick replay
runs on each change; extended seeded generation and external coverage-guided runs
have separately recorded budgets and a scheduled cadence. An oracle disagreement
must be triaged and replayable. Missing tools, empty required corpora and skipped
measurements cannot count as a pass. The later campaigns broaden this evidence.

The runner commands introduced by the early assurance passes are planned until
those passes land: `scripts/check-assurance.sh quick`,
`scripts/check-assurance.sh extended`, and
`scripts/check-native-fixtures.sh --toolchain VERSION`. On native API admission,
the last command runs real macro consumers on every compiler listed in the Rust
matrix, with positive/negative diagnostics, provenance and expansion assertions.
Core tests and native consumers are distinct; cargo check alone proves neither.

Each ordinary implementation pass should add no more than three tightly coupled
new behaviors. A grammar family must name its productions; generic “remaining
forms” language cannot hide unresolved implementation. Re-split before starting
if an inventory exposes more work. Opaque regions remain explicitly partial until
the integrating deep-validation milestone succeeds. Recovery is inspection-only.

## Version map

| Versions | Detailed implementation passes |
| --- | --- |
| 0.1.0–0.14.0 | [Foundation and accounting](#foundation-and-accounting) |
| 0.15.0–0.24.0 | [Native tokens and portable views](#native-tokens-and-portable-views) |
| 0.25.0–0.34.0 | [Declaration recognition and generic projections](#declaration-recognition-and-generic-projections) |
| 0.35.0–0.45.0 | [Literal decoding and typed attributes](#literal-decoding-and-typed-attributes) |
| 0.46.0–0.57.0 | [Fallible emission and usable macro profile](#fallible-emission-and-usable-macro-profile) |
| 0.58.0–0.67.0 | [Quotation and generated schemas](#quotation-and-generated-schemas) |
| 0.68.0–0.80.0 | [Source frontend and Unicode](#source-frontend-and-unicode) |
| 0.81.0–0.98.0 | [Expanded types, expressions and patterns](#expanded-types-expressions-and-patterns) |
| 0.99.0–0.115.0 | [Full items and traversal](#full-items-and-traversal) |
| 0.116.0–0.127.0 | [Owned syntax and lossless edits](#owned-syntax-and-lossless-edits) |
| 0.128.0–0.137.0 | [Security, performance and production admission](#security-performance-and-production-admission) |
| 1.0.0-rc.N | [Exact candidate](#100-rcn--exact-candidate) |
| 1.0.0 | [Production admission](#100--production-admission) |

## Foundation and accounting

### 0.1.0 — Workspace and assurance setup

**Setup:** Start from initial repository and supplied idea; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Workspace and assurance setup with explicit validation and resource contracts.

**Deliverables:** Establish the four-crate DAG, licenses, separate READMEs, tool pins, CI, policies and complete roadmap. Adopt the Brynja committed-report release gate with regression fixtures; ordinary CI tests the gate, while release validation requires a passing report.

**Verification:** Run local, compiler and target gates; review dependency rejection tests and package contents. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.2.0 — Contract inventory and errors

**Setup:** Start from completed 0.1.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Contract inventory and errors with explicit validation and resource contracts.

**Deliverables:** Define typed portable error codes and the inspect/validate/recover/opaque contract inventory; pin the ten adversarial fixture inputs. Maintain requirement-to-owner-and-evidence rows in this roadmap; mark every evidence item planned until an executable test is linked.

**Verification:** Check deterministic error identity, no input disclosure, and each fixture owner; no parser success claims. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.3.0 — Language baseline and support matrix

**Setup:** Start from completed 0.2.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Language baseline and support matrix with explicit validation and resource contracts.

**Deliverables:** Before grammar implementation, pin Rust Reference and compiler source revisions, retrieval checksums and supported editions; inventory declaration/type/expression/item productions and unsupported/opaque policies. Keep build MSRV, understood grammar, generated-code minimum compiler and Unicode data as separate axes.

**Verification:** Validate the source manifest and edition/production matrix offline; a missing baseline blocks the first declaration parser. Rust 2015/2018/2021/2024 each get an explicit supported/planned decision and native-token limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.4.0 — Explicit resource limits

**Setup:** Start from completed 0.3.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Explicit resource limits with explicit validation and resource contracts.

**Deliverables:** Define units and checked limits for bytes, tokens, depth, nodes, decoded data, storage, work, output and diagnostics.

**Verification:** Test zero/exact/one-over boundaries and usize/u64 overflow on 32-bit and 64-bit targets. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.5.0 — Monotonic work budget

**Setup:** Start from completed 0.4.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Monotonic work budget with explicit validation and resource contracts.

**Deliverables:** Implement one invocation fuel ledger shared by child operations; charge before action.

**Verification:** Exhaust fuel during nested operations; failed work cannot wrap, reset or regain credit. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.6.0 — Caller-owned storage

**Setup:** Start from completed 0.5.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Caller-owned storage with explicit validation and resource contracts.

**Deliverables:** Implement safe initialized-slot storage with fallible insertion and occupancy accounting.

**Verification:** Test empty/exact/full buffers, failed insertion, reuse and no allocator linkage. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.7.0 — Token identities and provenance

**Setup:** Start from completed 0.6.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Token identities and provenance with explicit validation and resource contracts.

**Deliverables:** Introduce private store-bound token/source identities and checked range constructors.

**Verification:** Reject foreign IDs, inverted ranges, overflow and stale source identities; compile-fail forging attempts. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.8.0 — Flat portable token tape

**Setup:** Start from completed 0.7.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Flat portable token tape with explicit validation and resource contracts.

**Deliverables:** Define token kinds, punctuation spacing and checked delimiter links in flat caller storage.

**Verification:** Reject unmatched/crossed groups and invalid indices; traverse deepest allowed tape iteratively. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.9.0 — Cursor and checkpoints

**Setup:** Start from completed 0.8.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Cursor and checkpoints with explicit validation and resource contracts.

**Deliverables:** Add bounded lookahead, checkpoints and explicit complete versus prefix consumption.

**Verification:** Rollback restores position but never fuel; trailing input and out-of-range seeks fail. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.10.0 — Progress-safe parser combinators

**Setup:** Start from completed 0.9.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Progress-safe parser combinators with explicit validation and resource contracts.

**Deliverables:** Implement alternatives, optional and separated repetition with explicit trailing-separator rules.

**Verification:** Zero-progress success fails repetition; adversarial failed alternatives exhaust the shared budget. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.11.0 — Compiler fixture runner

**Setup:** Start from completed 0.10.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Compiler fixture runner with explicit validation and resource contracts.

**Deliverables:** Build a Rust/std subprocess runner for compile-pass/fail fixtures with explicit compiler, edition, profile, timeout and expected diagnostic markers. Test the runner using first-party fixture macros; no Synir adapter functionality is claimed yet.

**Verification:** Exercise success, deliberate failure, crash, missing compiler, timeout and bounded output capture; commands must execute the requested compiler, not silently use the default. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.12.0 — Deterministic adversarial runner

**Setup:** Start from completed 0.11.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Deterministic adversarial runner with explicit validation and resource contracts.

**Deliverables:** Add seeded input generators, corpus replay, failure minimization and work/storage/stack/output measurements through test-only observation hooks. Record per-subsystem thresholds in the scope manifest.

**Verification:** Replay exact seeds, exhaust every available budget, distinguish missing instrumentation from a pass, and verify a deliberately over-budget fixture is rejected. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.13.0 — Independent comparison protocol

**Setup:** Start from completed 0.12.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Independent comparison protocol with explicit validation and resource contracts.

**Deliverables:** Implement a versioned external-executable oracle protocol and first-party adapter runner. Pin executable provenance and corpus revisions; classify syntax disagreements separately from semantic rustc failures.

**Verification:** Inject wrong oracle output, mismatched versions, missing executable and timeouts; differences produce replayable triage records. No comparison crates enter Cargo manifests. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.14.0 — Continuous assurance scheduling

**Setup:** Start from completed 0.13.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Continuous assurance scheduling with explicit validation and resource contracts.

**Deliverables:** Wire bounded replay into scripts/checks.sh and an extended scheduled CI job. Introduce scripts/check-assurance.sh quick/extended with numeric case, time and memory budgets; add corpus/owner registration before admitting each new algorithm.

**Verification:** Test nonempty registered coverage, seed replay, untriaged disagreement failure, and missing-runner failure. Archive failing cases safely; extended runs and coverage-guided external tools remain distinct evidence. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Native tokens and portable views

### 0.15.0 — Host invocation ownership

**Setup:** Start from completed 0.14.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Host invocation ownership with explicit validation and resource contracts.

**Deliverables:** Define invocation-local handle storage with explicit compiler-thread and lifetime restrictions.

**Verification:** Compile-fail escaped/foreign handles; test contracts through real compiler fixtures. Register real Synir native fixtures with the compiler runner when the first native API lands; then run them on every advertised compiler, not merely cargo check. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.16.0 — Native leaf import

**Setup:** Start from completed 0.15.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native leaf import with explicit validation and resource contracts.

**Deliverables:** Import identifiers, literals and punctuation preserving original handles, spans and spacing.

**Verification:** Macro fixtures compare joint operators, lifetimes, negative literals and raw identifiers; charge spelling extraction. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.17.0 — Native group import

**Setup:** Start from completed 0.16.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native group import with explicit validation and resource contracts.

**Deliverables:** Traverse native groups into the flat tape with explicit depth/token/storage accounting.

**Verification:** Deep and wide groups exhaust each limit cleanly; no recursive clone/drop path introduced. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.18.0 — Native spelling cache

**Setup:** Start from completed 0.17.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native spelling cache with explicit validation and resource contracts.

**Deliverables:** Cache per-token spelling under byte/work/storage quotas without stringify-and-reparse.

**Verification:** Repeated cache hits/misses preserve identity and charge correctly; long spellings fail boundedly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.19.0 — Native span policies

**Setup:** Start from completed 0.18.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native span policies with explicit validation and resource contracts.

**Deliverables:** Separate source offsets from invocation spans, opening/closing spans and resolution policy.

**Verification:** Real diagnostic locations and hygiene fixtures; no behavior depends on optional span source_text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.20.0 — Opaque forwarding

**Setup:** Start from completed 0.19.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Opaque forwarding with explicit validation and resource contracts.

**Deliverables:** Preserve original groups and token provenance with an explicit reason for opacity.

**Verification:** Compile macro_rules forwarding with Delimiter::None; compare precedence and original-token identity. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.21.0 — Bounded diagnostic rendering

**Setup:** Start from completed 0.20.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Bounded diagnostic rendering with explicit validation and resource contracts.

**Deliverables:** Render portable errors through caller sinks and native compile_error tokens; cap and redact data.

**Verification:** Overflow during error reporting still yields bounded failure; escape control characters and suppress literals/paths. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.22.0 — Inspection views

**Setup:** Start from completed 0.21.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Inspection views with explicit validation and resource contracts.

**Deliverables:** Expose header-only views distinct from validated shapes and opaque/recovered regions.

**Verification:** Malformed tails never acquire validated types; consuming a header cannot authorize high-level generation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.23.0 — Borrowing ergonomics

**Setup:** Start from completed 0.22.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Borrowing ergonomics with explicit validation and resource contracts.

**Deliverables:** Separate immutable input from mutable work/output and prototype nested user parser contexts.

**Verification:** Compile realistic borrowed views through decoding/emission contexts; callbacks cannot reset internal counters. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.24.0 — Native foundation acceptance

**Setup:** Start from completed 0.23.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native foundation acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate import, views, diagnostic failures and unchanged forwarding in real consumer fixtures.

**Verification:** Run MSRV/current stable, no_std target consumers, host OS tests and hostile group corpus. Include diagnostic marker/location, renamed-crate, opaque forwarding and expansion-behavior fixtures on all supported compilers; missing fixtures or compilers fail the native gate. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Declaration recognition and generic projections

### 0.25.0 — Item prefixes and visibility

**Setup:** Start from completed 0.24.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Item prefixes and visibility with explicit validation and resource contracts.

**Deliverables:** Recognize ordered outer attributes, visibility and item introducers; never scan past unexpected prefixes. Consume the already pinned language/edition inventory; register this first grammar corpus with quick replay, extended generation and the independent oracle before admission.

**Verification:** Reject skipped leading syntax; test pub, restricted visibility, raw names and preserved unrelated attributes. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.26.0 — Path and generic-argument boundaries

**Setup:** Start from completed 0.25.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Path and generic-argument boundaries with explicit validation and resource contracts.

**Deliverables:** Recognize qualified paths, nested arguments and associated bindings without comma scanning shortcuts.

**Verification:** Result<T,E>, qualified paths, turbofish and joint punctuation have exact ranges or explicit unsupported errors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.27.0 — Reference pointer and tuple type boundaries

**Setup:** Start from completed 0.26.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Reference pointer and tuple type boundaries with explicit validation and resource contracts.

**Deliverables:** Recognize references, raw pointers, tuple/group types and slices over token views.

**Verification:** Nested commas/lifetimes, invalid mutability and malformed delimiters have precise complete-consumption errors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.28.0 — Function and bounded type regions

**Setup:** Start from completed 0.27.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Function and bounded type regions with explicit validation and resource contracts.

**Deliverables:** Recognize function pointers, higher-ranked binders, trait-object bounds and array const regions with explicit opacity.

**Verification:** Fn(T)->Result<T,E> arrows never close generic lists; unsupported const internals cannot become validated expressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.29.0 — Named structs

**Setup:** Start from completed 0.28.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Named structs with explicit validation and resource contracts.

**Deliverables:** Parse every named field and attached attributes using grammar-aware type boundaries.

**Verification:** Nested type commas, missing colons, duplicates as syntax policy, malformed last field and early iterator stop. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.30.0 — Tuple and unit structs

**Setup:** Start from completed 0.29.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Tuple and unit structs with explicit validation and resource contracts.

**Deliverables:** Add tuple/unit forms and their item-specific terminators/where positions.

**Verification:** Tuple where clauses before semicolon, empty tuples, forbidden separators and trailing junk. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.31.0 — Enum variants

**Setup:** Start from completed 0.30.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Enum variants with explicit validation and resource contracts.

**Deliverables:** Recognize unit/tuple/record variants and explicit discriminant regions with bounded shape validation.

**Verification:** Every variant/field counted; malformed final variant rejected even if iteration stops early. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.32.0 — Union shapes

**Setup:** Start from completed 0.31.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Union shapes with explicit validation and resource contracts.

**Deliverables:** Parse union declarations with an explicit caller derive policy and no implicit field access.

**Verification:** Reject invalid union shapes; acceptance example policy rejects unsupported derive operations clearly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.33.0 — Generic parameter and where models

**Setup:** Start from completed 0.32.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generic parameter and where models with explicit validation and resource contracts.

**Deliverables:** Parse lifetime/type/const parameters, bounds, defaults and grammar-aware where predicates.

**Verification:** Where without generics, function trait bounds, raw parameters, empty lists, const defaults and tuple placement. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.34.0 — Validated impl projections

**Setup:** Start from completed 0.33.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Validated impl projections with explicit validation and resource contracts.

**Deliverables:** Produce declaration/impl/type-argument projections and explicit bound-policy plans from complete shapes.

**Verification:** Defaults removed only for impl parameters; predicates/spans/order preserved; generated minimal impls compile. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Literal decoding and typed attributes

### 0.35.0 — Literal classification

**Setup:** Start from completed 0.34.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Literal classification with explicit validation and resource contracts.

**Deliverables:** Distinguish all admitted Rust literal token forms and preserve original spellings and suffixes.

**Verification:** Reject wrong literal kind and malformed prefix/suffix; document unsupported forms explicitly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.36.0 — String decoding

**Setup:** Start from completed 0.35.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** String decoding with explicit validation and resource contracts.

**Deliverables:** Decode escaped and raw strings into caller buffers with decoded-byte/work accounting.

**Verification:** Escape, continuation, raw delimiter and UTF-8 boundary vectors; exact output capacity and one-over failure. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.37.0 — Byte and character decoding

**Setup:** Start from completed 0.36.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Byte and character decoding with explicit validation and resource contracts.

**Deliverables:** Decode byte strings/bytes/chars with form-specific Unicode and length checks.

**Verification:** Reject multi-scalar char, out-of-range byte and invalid escapes; no silent fallback to text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.38.0 — Checked numeric decoding

**Setup:** Start from completed 0.37.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Checked numeric decoding with explicit validation and resource contracts.

**Deliverables:** Implement checked integer schema conversion with explicit sign, base and suffix policies. Preserve float spelling; conversion outside the admitted policy returns an error.

**Verification:** Test representable endpoints, one-over overflow, malformed separators/exponents and suffix rejection without successful prefix conversion. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.39.0 — C-string decoding

**Setup:** Start from completed 0.38.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** C-string decoding with explicit validation and resource contracts.

**Deliverables:** Implement C-string literal decoding and validation under decoded-byte and work limits, consuming the earlier literal classification and caller-buffer interfaces.

**Verification:** Reject interior NUL, invalid escape/encoding and short output buffers; preserve original spelling separately from decoded bytes. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.40.0 — Attribute syntax

**Setup:** Start from completed 0.39.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Attribute syntax with explicit validation and resource contracts.

**Deliverables:** Parse paths, flags, assignments, nested lists and explicit opaque payloads separately from schema policy.

**Verification:** Unexpected values never become flags; trailing tokens and nesting exhaustion fail; preserve unrelated attributes. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.41.0 — Strict scalar schemas

**Setup:** Start from completed 0.40.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Strict scalar schemas with explicit validation and resource contracts.

**Deliverables:** Add handwritten schemas for booleans, strings, checked integers, paths and choices.

**Verification:** Wrong type, unknown key, misspelling, missing required value and malformed optional values all error. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.42.0 — Duplicate and conflict policies

**Setup:** Start from completed 0.41.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Duplicate and conflict policies with explicit validation and resource contracts.

**Deliverables:** Enforce singleton/repeatable cardinality, defaults and mutually exclusive keys.

**Verification:** No last-write-wins accident; explicit repeat ordering; duplicate/conflict diagnostics retain both locations. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.43.0 — Nested context schemas

**Setup:** Start from completed 0.42.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Nested context schemas with explicit validation and resource contracts.

**Deliverables:** Support bounded nested records and separate container/variant/field/generic contexts.

**Verification:** Misplaced keys, deep records and too many repeated values fail with bounded deterministic output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.44.0 — Custom validation and diagnostics

**Setup:** Start from completed 0.43.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Custom validation and diagnostics with explicit validation and resource contracts.

**Deliverables:** Add caller validators with documented external-work boundary and bounded independent error aggregation.

**Verification:** Callbacks cannot forge validated state; source-order diagnostics, truncation and error-budget exhaustion. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.45.0 — Schema acceptance

**Setup:** Start from completed 0.44.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Schema acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate literal decoding, strict schemas and derive shapes with validated attribute values.

**Verification:** Run hostile configuration corpus and context matrix; no malformed attribute activates behavior. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Fallible emission and usable macro profile

### 0.46.0 — Transactional token sink

**Setup:** Start from completed 0.45.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Transactional token sink with explicit validation and resource contracts.

**Deliverables:** Implement explicit start/commit/discard output with token/byte limits and reserved error capacity.

**Verification:** Any mid-output failure returns error without publishing a truncated successful stream. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.47.0 — Checked token builders

**Setup:** Start from completed 0.46.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Checked token builders with explicit validation and resource contracts.

**Deliverables:** Construct identifiers, supported punctuation, groups and typed literals; reject untrusted invalid names.

**Verification:** Invalid Ident input is rejected before compiler constructors; strings emit data, not executable text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.48.0 — Source-text sink and lexical separation

**Setup:** Start from completed 0.47.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source-text sink and lexical separation with explicit validation and resource contracts.

**Deliverables:** Add a bounded source-text sink alongside portable/native token sinks. Insert lexical separators to avoid token fusion; distinguish preserved raw token forwarding from typed expression emission. Stage atomic output, or explicitly document caller streaming sinks as potentially partially written on error.

**Verification:** Compiler fixtures cover adjacent identifiers, numeric suffixes, lifetimes, joint punctuation and literals; verify lexical structure, exact output limits and failed-sink behavior. No partial output may be reported as a successful artifact. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.49.0 — Native export

**Setup:** Start from completed 0.48.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native export with explicit validation and resource contracts.

**Deliverables:** Export portable constructions with explicit span policy while forwarding original native objects unchanged.

**Verification:** Group/spacing/span fixtures and output exhaustion through real proc macros; document compiler allocation boundary. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.50.0 — Generic impl emitter

**Setup:** Start from completed 0.49.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generic impl emitter with explicit validation and resource contracts.

**Deliverables:** Emit validated impl plans with projected generics, original predicates and explicit trait/dependency paths.

**Verification:** Compile lifetime/default/const/where examples; no incidental Clone/Debug bounds; renamed runtime paths work. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.51.0 — Describe structs acceptance

**Setup:** Start from completed 0.50.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Describe structs acceptance with explicit validation and resource contracts.

**Deliverables:** Implement a real metadata derive for named, tuple and unit structs using the manual pipeline.

**Verification:** Compile and execute metadata tests; no_std target contains only intended trait/data, no parser runtime. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.52.0 — Describe enum and union policy

**Setup:** Start from completed 0.51.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Describe enum and union policy with explicit validation and resource contracts.

**Deliverables:** Extend Describe to enums and document explicit union behavior and field options.

**Verification:** Wrong attributes and unsupported union behavior emit deliberate errors; no omitted variants/fields. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.53.0 — Declaration-preserving attribute macro

**Setup:** Start from completed 0.52.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Declaration-preserving attribute macro with explicit validation and resource contracts.

**Deliverables:** Provide a compiled function attribute example retaining opaque bodies and unrelated attributes.

**Verification:** Body/provenance unchanged; malformed owned options and unsupported signatures fail without body truncation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.54.0 — Error derive example

**Setup:** Start from completed 0.53.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Error derive example with explicit validation and resource contracts.

**Deliverables:** Provide a compiled error-type derive using the manual pipeline, explicit dependency paths and documented bound policy.

**Verification:** Compile generics with necessary and unnecessary bounds, renamed runtime paths, invalid options and no_std-capable output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.55.0 — Custom DSL example

**Setup:** Start from completed 0.54.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Custom DSL example with explicit validation and resource contracts.

**Deliverables:** Provide a custom-language macro built from the portable combinators and fallible sinks, with an explicit grammar and completeness contract.

**Verification:** Exercise zero-progress alternatives, trailing input, malformed DSL tokens and output failures without bypassing shared invocation budgets. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.56.0 — Macro-development profile

**Setup:** Start from completed 0.55.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro-development profile with explicit validation and resource contracts.

**Deliverables:** Expose a documented additive macro-dev preset for implemented derive/attribute/manual emission capabilities.

**Verification:** Feature-isolated builds, renamed facade/runtime dependencies, namespace shadowing and no_std consumers. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.57.0 — Macro workflow acceptance

**Setup:** Start from completed 0.56.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro workflow acceptance with explicit validation and resource contracts.

**Deliverables:** Measure and review end-to-end native import to validated plan to output on several real use cases.

**Verification:** Publish boundedness and cold/warm benchmark methodology; run independent compiler fixtures and pentest. Compare the same workload with both ordinary and selective upstream parsing; include native import. Record per-profile size/latency/memory baselines without inventing speedup targets. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Quotation and generated schemas

### 0.58.0 — Quotation language specification

**Setup:** Start from completed 0.57.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Quotation language specification with explicit validation and resource contracts.

**Deliverables:** Specify a clean-slate template grammar, marker escaping and builder lowering independent of Quote API compatibility.

**Verification:** Ambiguous templates, literal markers and unsupported constructs have explicit diagnostics and bounded parsing. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.59.0 — Scalar interpolation

**Setup:** Start from completed 0.58.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Scalar interpolation with explicit validation and resource contracts.

**Deliverables:** Lower scalar token/literal/view interpolation into fallible manual builder calls.

**Verification:** Data/code distinction, nested groups and runtime errors preserve transactional output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.60.0 — Renamed-crate hygiene

**Setup:** Start from completed 0.59.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Renamed-crate hygiene with explicit validation and resource contracts.

**Deliverables:** Implement tested framework-path plumbing and explicit span selection for generated builder code.

**Verification:** Rename Synir, shadow common names and combine multiple dependency aliases in real compilation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.61.0 — Simple repetition

**Setup:** Start from completed 0.60.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Simple repetition with explicit validation and resource contracts.

**Deliverables:** Implement bounded zero-or-more/nonempty/optional repetitions and separators.

**Verification:** Empty, one, many, missing item and trailing-separator cases; work/output counters cover every iteration. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.62.0 — Nested and multiple repetition

**Setup:** Start from completed 0.61.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Nested and multiple repetition with explicit validation and resource contracts.

**Deliverables:** Define lexical repetition scopes and exact finite sequence length matching.

**Verification:** Mismatched lengths error, never zip-truncate; deeply nested output growth exhausts shared limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.63.0 — Punctuation and lifetime quotation

**Setup:** Start from completed 0.62.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Punctuation and lifetime quotation with explicit validation and resource contracts.

**Deliverables:** Preserve jointness, lifetimes, negative literals and delimiter semantics through templates.

**Verification:** Native compilation distinguishes operator token fusion and invisible-group precedence regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.64.0 — Quotation resource acceptance

**Setup:** Start from completed 0.63.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Quotation resource acceptance with explicit validation and resource contracts.

**Deliverables:** Bound template compilation as well as generated execution and eliminate recursive expansion blowups.

**Verification:** Large templates, escaped markers and pathological repetition have measured bounded failures. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.65.0 — Generated schema decoder core

**Setup:** Start from completed 0.64.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generated schema decoder core with explicit validation and resource contracts.

**Deliverables:** Generate the same strict scalar decoder semantics as handwritten schemas without self-bootstrap dependencies.

**Verification:** Compare handwritten/generated behavior for valid and malformed options; helper crate never depends on facade. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.66.0 — Generated nested schema support

**Setup:** Start from completed 0.65.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generated nested schema support with explicit validation and resource contracts.

**Deliverables:** Extend schema helpers to contexts, nested/repeated values, forwarding and custom validation hooks.

**Verification:** Parity for duplicate/unknown/conflict diagnostics and locations; code-size and budget regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.67.0 — Helper workflow acceptance

**Setup:** Start from completed 0.66.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Helper workflow acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate quotation and generated schemas as optional conveniences over the working manual path.

**Verification:** Run all helper feature combinations and real fixtures; publish manual-versus-helper compile cost. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Source frontend and Unicode

### 0.68.0 — Source contract and trivia

**Setup:** Start from completed 0.67.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source contract and trivia with explicit validation and resource contracts.

**Deliverables:** Extend the existing pinned grammar/edition contract for source input; introduce borrowed UTF-8 source and lossless trivia ranges.

**Verification:** Invalid byte input is rejected explicitly; no-op traversal preserves bytes and source identity. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.69.0 — ASCII identifiers and punctuation

**Setup:** Start from completed 0.68.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** ASCII identifiers and punctuation with explicit validation and resource contracts.

**Deliverables:** Implement an explicitly restricted ASCII lexer with raw/keyword/edition rules and punctuation jointness.

**Verification:** Reject non-ASCII identifiers as unsupported; no complete-Rust lexer claim; reserved prefixes and lifetimes tested. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.70.0 — Comments and groups

**Setup:** Start from completed 0.69.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Comments and groups with explicit validation and resource contracts.

**Deliverables:** Lex nested block/line/doc comments and validated delimiters with iterative depth accounting.

**Verification:** Unclosed/deep comments, delimiters inside comments and trivia boundaries consume bounded work. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.71.0 — Source literal scanning

**Setup:** Start from completed 0.70.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source literal scanning with explicit validation and resource contracts.

**Deliverables:** Recognize strings/raw strings, chars, byte/C strings and numeric tokens into borrowed ranges.

**Verification:** Escapes, raw hashes, lifetime/char ambiguity, malformed exponent/suffix and oversized literals. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.72.0 — Unicode data generation

**Setup:** Start from completed 0.71.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unicode data generation with explicit validation and resource contracts.

**Deliverables:** Pin official XID and normalization data, checksums, licenses and an offline Rust generator; shard tables.

**Verification:** Regeneration byte equality, input checksum mismatch rejection and every generated file at most 500 lines. Require the exact Unicode data version and its relationship to each admitted grammar/compiler profile in the existing source manifest. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.73.0 — Unicode identifier classification

**Setup:** Start from completed 0.72.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unicode identifier classification with explicit validation and resource contracts.

**Deliverables:** Implement XID_Start/XID_Continue with ASCII fast path and edition/raw identifier restrictions.

**Verification:** Exhaustive scalar classification against pinned data; invalid scalar boundaries and table edge vectors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.74.0 — Canonical decomposition

**Setup:** Start from completed 0.73.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Canonical decomposition with explicit validation and resource contracts.

**Deliverables:** Implement bounded canonical decomposition, including algorithmic Hangul decomposition, using pinned tables and caller-owned storage. Preserve original spelling.

**Verification:** Run pinned decomposition vectors; exact-capacity and long expansion cases exhaust storage/work correctly. No complete NFC claim yet. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.75.0 — Canonical combining order

**Setup:** Start from completed 0.74.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Canonical combining order with explicit validation and resource contracts.

**Deliverables:** Implement stable canonical ordering within combining sequences over the decomposition output; account for every comparison/move.

**Verification:** Test equal combining classes, starter boundaries and reversed long sequences; prove observed work stays within the charged budget even for quadratic candidates. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.76.0 — Canonical composition

**Setup:** Start from completed 0.75.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Canonical composition with explicit validation and resource contracts.

**Deliverables:** Implement canonical composition and blocking rules, including Hangul and composition exclusions, over ordered decompositions.

**Verification:** Test blocked/unblocked composition, excluded pairs, Hangul edges and exact output capacity against pinned official vectors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.77.0 — NFC pipeline and conformance

**Setup:** Start from completed 0.76.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** NFC pipeline and conformance with explicit validation and resource contracts.

**Deliverables:** Integrate decomposition, ordering and composition into no-allocator and optional alloc NFC APIs with shared limits and an ASCII fast path.

**Verification:** Run the complete pinned normalization suite and independent oracle comparisons; test idempotence and exhaustion in every phase. Only this integrated pass admits an NFC capability. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.78.0 — Canonical identifier identity

**Setup:** Start from completed 0.77.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Canonical identifier identity with explicit validation and resource contracts.

**Deliverables:** Keep original spelling separate from normalized identity and compiler-normalized tokens.

**Verification:** Canonically equivalent source identifiers compare as defined; source bytes unchanged; native/source differences triaged. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.79.0 — Source/native conformance

**Setup:** Start from completed 0.78.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source/native conformance with explicit validation and resource contracts.

**Deliverables:** Run the same supported declaration/schema grammar against source and native token frontends.

**Verification:** Compare normalized structure, never lost whitespace; grammar/Unicode/compiler version axes recorded. Include doc-comment-to-attribute lowering, punctuation, normalized identifiers and explicit unsupported production cases. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.80.0 — Standalone and no-allocator acceptance

**Setup:** Start from completed 0.79.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Standalone and no-allocator acceptance with explicit validation and resource contracts.

**Deliverables:** Ship source inspection and caller-storage parser examples with optional fallible owned storage.

**Verification:** Bare-metal and mobile target checks; tiny/exact buffers; alloc reservation errors and no implicit file loading. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Expanded types, expressions and patterns

### 0.81.0 — Complete type grammar inventory

**Setup:** Start from completed 0.80.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Complete type grammar inventory with explicit validation and resource contracts.

**Deliverables:** Close supported Rust type-production gaps and record opaque macro/unstable policies against pinned reference.

**Verification:** Matrix names every type form; unsupported syntax cannot be mislabeled as a validated type. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.82.0 — Qualified types and associated constraints

**Setup:** Start from completed 0.81.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Qualified types and associated constraints with explicit validation and resource contracts.

**Deliverables:** Complete typed views for qualified paths and associated equality/bound constraints using the earlier boundary recognizers.

**Verification:** Test nested qualified projections and associated binding ambiguity, invalid tails and every newly admitted production against compiler/oracle fixtures. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.83.0 — Bare function types and binders

**Setup:** Start from completed 0.82.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Bare function types and binders with explicit validation and resource contracts.

**Deliverables:** Complete typed bare-function, ABI and higher-ranked binder views; preserve qualifiers and lifetimes with bounded recognition.

**Verification:** Check arrows versus angle closures, nested fn bounds, malformed binder lists and complete consumption across compiler versions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.84.0 — Trait object and impl-trait types

**Setup:** Start from completed 0.83.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Trait object and impl-trait types with explicit validation and resource contracts.

**Deliverables:** Complete dyn/impl-trait and remaining inventoried stable type forms, with explicit contextual restrictions and opaque macro policy.

**Verification:** Reconcile the type-production inventory; positive/negative vectors cover bounds and contexts, with no remaining form hidden by a generic supported claim. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.85.0 — Expression atoms and paths

**Setup:** Start from completed 0.84.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Expression atoms and paths with explicit validation and resource contracts.

**Deliverables:** Recognize expression literals, paths, groups and explicit opaque macro invocations.

**Verification:** No semantic resolution claims; distinguish malformed paths from opaque macro bodies. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.86.0 — Postfix expressions

**Setup:** Start from completed 0.85.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Postfix expressions with explicit validation and resource contracts.

**Deliverables:** Add calls, method calls, field/index/await/try operators under explicit edition rules.

**Verification:** Chained syntax, turbofish and malformed argument separators; complete consumption and limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.87.0 — Unary cast and range expressions

**Setup:** Start from completed 0.86.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unary cast and range expressions with explicit validation and resource contracts.

**Deliverables:** Add unary operations, casts and range forms with explicit contextual restrictions, using the established expression atoms/postfix nodes.

**Verification:** Check precedence boundaries, open/closed ranges, invalid casts and complete-input rejection; bounded nesting and operator ambiguity. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.88.0 — Binary operator precedence

**Setup:** Start from completed 0.87.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Binary operator precedence with explicit validation and resource contracts.

**Deliverables:** Implement binary operators with a declared precedence and associativity table over admitted operands.

**Verification:** Assert exact tree structure for mixed operators and contextual punctuation; compare generated compiler behavior and charge all speculation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.89.0 — Assignments and compound assignments

**Setup:** Start from completed 0.88.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Assignments and compound assignments with explicit validation and resource contracts.

**Deliverables:** Implement assignment forms and their precedence/context rules using the completed operator recognizer.

**Verification:** Test chained forms, invalid places as syntactic versus semantic failures, compound tokens and complete consumption. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.90.0 — Block and conditional expression shapes

**Setup:** Start from completed 0.89.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Block and conditional expression shapes with explicit validation and resource contracts.

**Deliverables:** Recognize block boundaries and if/else contexts. Preserve statement regions explicitly as opaque until statement parsing is integrated; do not label opaque interiors fully validated.

**Verification:** Test dangling else, let-condition boundaries and invalid delimiters. Record partial validity explicitly; full block admission waits for statement and full-grammar acceptance. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.91.0 — Loops labels and control expressions

**Setup:** Start from completed 0.90.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Loops labels and control expressions with explicit validation and resource contracts.

**Deliverables:** Implement loop/while/for and break/continue/return shape recognition under the same explicit block/pattern completeness policy.

**Verification:** Check labels, value-bearing control forms, malformed headers and limits; deferred inner syntax remains opaque and cannot enter a deep-validation plan. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.92.0 — Closure expressions

**Setup:** Start from completed 0.91.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Closure expressions with explicit validation and resource contracts.

**Deliverables:** Recognize closure parameters, qualifiers and body boundaries under the admitted expression/pattern grammar.

**Verification:** Test parameter/type ambiguity, move forms and malformed closures; unsupported body syntax reports explicit partial/unsupported state. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.93.0 — Async const and unsafe expressions

**Setup:** Start from completed 0.92.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Async const and unsafe expressions with explicit validation and resource contracts.

**Deliverables:** Implement the remaining inventoried stable special expression forms, preserving explicit block completeness and edition requirements.

**Verification:** Test edition-dependent forms and invalid qualifier combinations; complete grammar admission requires statement integration. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.94.0 — Atomic and binding patterns

**Setup:** Start from completed 0.93.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Atomic and binding patterns with explicit validation and resource contracts.

**Deliverables:** Implement wildcard, literal/path, binding and reference pattern forms with explicit binding-mode syntax.

**Verification:** Test mut/ref combinations, raw names, invalid binding forms and contextual restrictions without assuming name resolution. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.95.0 — Destructuring patterns

**Setup:** Start from completed 0.94.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Destructuring patterns with explicit validation and resource contracts.

**Deliverables:** Add tuple, tuple-struct, slice/array and record patterns using validated atom/binding patterns.

**Verification:** Test rest placement, nested destructuring, separators, missing fields and depth/work exhaustion; no silent pattern omission. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.96.0 — Pattern alternatives and ranges

**Setup:** Start from completed 0.95.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Pattern alternatives and ranges with explicit validation and resource contracts.

**Deliverables:** Add range and or-pattern forms with explicit precedence and contextual restrictions.

**Verification:** Test nested alternatives, invalid range bounds as syntax versus semantics, and malformed trailing alternatives. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.97.0 — Match expressions and guards

**Setup:** Start from completed 0.96.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Match expressions and guards with explicit validation and resource contracts.

**Deliverables:** Integrate admitted patterns and expressions into match arms and guards with explicit separator rules.

**Verification:** Test arm commas, guarded alternatives, nested matches and incomplete arms; all children must meet the requested validation level. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.98.0 — Typed expression emission

**Setup:** Start from completed 0.97.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Typed expression emission with explicit validation and resource contracts.

**Deliverables:** Emit validated expressions with conservative precedence/associativity grouping.

**Verification:** Compile precedence-sensitive generated programs; raw token interpolation makes no typed-expression promise. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Full items and traversal

### 0.99.0 — Local binding statements

**Setup:** Start from completed 0.98.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Local binding statements with explicit validation and resource contracts.

**Deliverables:** Implement let and let-else statements using admitted pattern and expression parsers; distinguish syntactic validity from semantic divergence/type rules.

**Verification:** Test initializer boundaries, required else blocks, semicolons and malformed patterns; exact error spans and bounded rollback. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.100.0 — Statement lists and block completion

**Setup:** Start from completed 0.99.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Statement lists and block completion with explicit validation and resource contracts.

**Deliverables:** Implement item/expression statements and tail-expression rules; replace earlier opaque block interiors only after successful deep validation.

**Verification:** Test semicolon distinctions, nested controls, full block consumption and failure atomicity. Re-run all earlier block/closure/async/control fixtures as deeply validated cases. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.101.0 — Function qualifiers and ABI

**Setup:** Start from completed 0.100.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Function qualifiers and ABI with explicit validation and resource contracts.

**Deliverables:** Complete function prefix/ABI grammar against the early pinned inventory, consuming the earlier declaration-preserving subset.

**Verification:** Reject illegal ordering and combinations; compare edition/compiler fixtures and retain exact signature/body boundaries. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.102.0 — Parameters receivers and signatures

**Setup:** Start from completed 0.101.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Parameters receivers and signatures with explicit validation and resource contracts.

**Deliverables:** Complete parameter, receiver, variadic and return-type signature grammar, with explicit item/trait/impl context.

**Verification:** Test self forms, ABI restrictions, unsupported contexts and generic/where placement; no implicit body parsing required for signature-only operations. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.103.0 — Trait declarations and members

**Setup:** Start from completed 0.102.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Trait declarations and members with explicit validation and resource contracts.

**Deliverables:** Implement trait item/member syntax and bounds against the pinned stable inventory, retaining explicit opacity for unrecognized unstable forms.

**Verification:** Test associated type/function/constant contexts, supertraits, where placement and malformed members. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.104.0 — Implementation blocks and members

**Setup:** Start from completed 0.103.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Implementation blocks and members with explicit validation and resource contracts.

**Deliverables:** Implement inherent/trait impl blocks and associated items using admitted signatures/types; do not silently enable unstable negative impls.

**Verification:** Test unsafe/default/modifier contexts and stable-versus-unstable policy, malformed members and complete block consumption. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.105.0 — Module declarations

**Setup:** Start from completed 0.104.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Module declarations with explicit validation and resource contracts.

**Deliverables:** Implement inline/external module syntax while leaving file loading to explicit application interfaces.

**Verification:** Test inline bodies, semicolon modules, attributes and malformed declarations with no filesystem reads. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.106.0 — Use trees

**Setup:** Start from completed 0.105.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Use trees with explicit validation and resource contracts.

**Deliverables:** Implement nested import trees, globs and renames over bounded path/token views.

**Verification:** Test nested groups, separators and alias forms, invalid trees and exact-input consumption. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.107.0 — Foreign modules and items

**Setup:** Start from completed 0.106.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Foreign modules and items with explicit validation and resource contracts.

**Deliverables:** Implement admitted extern module/item productions and their ABI/context restrictions.

**Verification:** Test foreign members and invalid contexts across editions; preserve spans and reject unknown stable claims. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.108.0 — Type aliases

**Setup:** Start from completed 0.107.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Type aliases with explicit validation and resource contracts.

**Deliverables:** Implement type-alias item contexts, generics and where forms using the validated type grammar.

**Verification:** Test alias contexts and malformed types; syntactic acceptance never claims alias/name resolution. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.109.0 — Constants and statics

**Setup:** Start from completed 0.108.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Constants and statics with explicit validation and resource contracts.

**Deliverables:** Implement const/static item declarations using admitted expression/type grammar.

**Verification:** Test mutability/context rules, initializer boundaries and invalid tails; no evaluation or implicit execution. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.110.0 — Remaining stable item inventory closure

**Setup:** Start from completed 0.109.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Remaining stable item inventory closure with explicit validation and resource contracts.

**Deliverables:** Implement any remaining named stable item productions from the early inventory in bounded review scope; split again if more than three new behaviors remain.

**Verification:** Every inventory row has a concrete parser and negative fixture or an explicit unsupported status that blocks the full-profile claim. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.111.0 — Macro syntax representation

**Setup:** Start from completed 0.110.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro syntax representation with explicit validation and resource contracts.

**Deliverables:** Represent macro_rules definitions and invocations with validated outer structure and explicit opaque DSL bodies.

**Verification:** Metavariables/repetition delimiters preserved; no expansion or fabricated validation of arbitrary DSL contents. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.112.0 — Full grammar acceptance

**Setup:** Start from completed 0.111.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Full grammar acceptance with explicit validation and resource contracts.

**Deliverables:** Reconcile edition/production matrix, grammar revision and native/source support for all admitted stable Rust syntax.

**Verification:** Pinned compiler/independent-parser corpus triage; no missing production hidden in a percentage claim. Discharge every earlier opaque block/pattern restriction before a deep/full validation claim. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.113.0 — Borrowed visits

**Setup:** Start from completed 0.112.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Borrowed visits with explicit validation and resource contracts.

**Deliverables:** Add typed iterative traversal with explicit limits over validated syntax views and opaque regions.

**Verification:** Every node visited as specified, cancellation/exhaustion bounded, deep traversal no stack overflow. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.114.0 — Structural mapping and folds

**Setup:** Start from completed 0.113.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Structural mapping and folds with explicit validation and resource contracts.

**Deliverables:** Add checked transformation plans with validation before mutation and provenance-aware replacements.

**Verification:** Failure atomicity, foreign ranges, generated syntax validity and unchanged input on rejected edits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.115.0 — Full-profile acceptance

**Setup:** Start from completed 0.114.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Full-profile acceptance with explicit validation and resource contracts.

**Deliverables:** Expose full grammar as an additive optional profile and publish source-analysis/transformation workflows.

**Verification:** Minimal macro profile cost unchanged; full profile production matrix and independent consumer fixtures pass. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Owned syntax and lossless edits

### 0.116.0 — Owned arena storage

**Setup:** Start from completed 0.115.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned arena storage with explicit validation and resource contracts.

**Deliverables:** Add flat owned syntax arenas with fallible reservation and private source/node identities.

**Verification:** Allocation-size overflow, exhaustion, failed insertion and cross-arena IDs; no recursive destruction. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.117.0 — Owned view conversion

**Setup:** Start from completed 0.116.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned view conversion with explicit validation and resource contracts.

**Deliverables:** Convert validated borrowed views into owned nodes under shared limits without losing opacity/provenance.

**Verification:** Round-trip normalized structure and span origin; stale native handles cannot be serialized. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.118.0 — Deep operation safety

**Setup:** Start from completed 0.117.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Deep operation safety with explicit validation and resource contracts.

**Deliverables:** Audit clone/drop/debug/equality/hash of all owned token and syntax structures.

**Verification:** Adversarial depth on small-stack worker processes; every operation has bounded traversal/output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.119.0 — Revision-bound edit plans

**Setup:** Start from completed 0.118.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Revision-bound edit plans with explicit validation and resource contracts.

**Deliverables:** Define source identity/revision, UTF-8 ranges and deterministic insertion order.

**Verification:** Reject wrong source/revision even at matching length; invalid UTF-8 boundary, inverted and overflow ranges. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.120.0 — Edit conflict validation

**Setup:** Start from completed 0.119.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Edit conflict validation with explicit validation and resource contracts.

**Deliverables:** Check overlap, duplicate replacement and equal-offset insertion policies before commit.

**Verification:** Permutation/conflict corpus and failure atomicity; no container-order-dependent winners. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.121.0 — Lossless edit application

**Setup:** Start from completed 0.120.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Lossless edit application with explicit validation and resource contracts.

**Deliverables:** Apply validated edits in one pass to caller sinks or budgeted owned output.

**Verification:** No-op byte identity, untouched comments/trivia, exact output limit and no partial successful artifact. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.122.0 — Bounded source recovery for inspection

**Setup:** Start from completed 0.121.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Bounded source recovery for inspection with explicit validation and resource contracts.

**Deliverables:** Implement an explicit source-inspection recovery profile with synchronization points, bounded skipped-token ranges and structured errors. Return RecoveredSyntax, never validated plans; incomplete source editing remains opt-in.

**Verification:** Test repeated malformed tokens, recovery progress, work/error exhaustion and stable source ranges. Compile-fail/API tests prevent recovered nodes from reaching strict high-level emission without successful revalidation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.123.0 — Explicit configuration context

**Setup:** Start from completed 0.122.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Explicit configuration context with explicit validation and resource contracts.

**Deliverables:** Add application-supplied cfg contexts while preserving unresolved alternatives unless the caller explicitly requests evaluation.

**Verification:** Test unknown options, source/native expansion-order differences and no implicit environment reads. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.124.0 — Explicit file-loading boundary

**Setup:** Start from completed 0.123.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Explicit file-loading boundary with explicit validation and resource contracts.

**Deliverables:** Define optional application-provided source loading with origin/path/resource policy; portable parsing never follows mod/include automatically.

**Verification:** Test denied loads, hostile paths, missing sources and exhausted budgets; loader work outside library control is documented. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.125.0 — Owned transformation acceptance

**Setup:** Start from completed 0.124.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned transformation acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate full traversal, owned plans and lossless edits in standalone source tools.

**Verification:** Compiler-valid transformed fixtures, error propagation, caller-storage and alloc variants. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.126.0 — Workflow migration guides

**Setup:** Start from completed 0.125.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Workflow migration guides with explicit validation and resource contracts.

**Deliverables:** Publish task-oriented guides for derive, schema, quote, custom DSL, source analysis and transformations.

**Verification:** Compile every example with dependency renames/no_std where relevant; no API compatibility promises. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.127.0 — Capability replacement acceptance

**Setup:** Start from completed 0.126.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Capability replacement acceptance with explicit validation and resource contracts.

**Deliverables:** Reconcile all idea sections and practical replacement workflows with real downstream acceptance projects.

**Verification:** No descriptor/mock-only claims; parser, builder, helper and full/edit profiles each have independent consumers. All requirement rows below must link actual implementation, test command and evidence; any planned entry blocks production admission. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Security, performance and production admission

### 0.128.0 — Hostile-input campaign

**Setup:** Start from completed 0.127.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Hostile-input campaign with explicit validation and resource contracts.

**Deliverables:** Expand deterministic mutation/generation across tokens, grammar, literals, schemas, quoting and edits. Extend the continuously running early harness; this is not its introduction.

**Verification:** Run with external CPU/memory/time limits; replay seeds and record worst-case work/storage growth. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.129.0 — Independent differential campaign

**Setup:** Start from completed 0.128.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Independent differential campaign with explicit validation and resource contracts.

**Deliverables:** Compare admitted syntax and emitted behavior against pinned independent compiler/parser tools. Build on the early per-parser oracle protocol and triage records.

**Verification:** Triage every disagreement; semantic errors separated from syntax; save reproducible regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.130.0 — Platform runtime acceptance

**Setup:** Start from completed 0.129.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Platform runtime acceptance with explicit validation and resource contracts.

**Deliverables:** Run real consumers on Linux/Windows/macOS/FreeBSD and Android/iOS device or emulator/simulator setups.

**Verification:** Record exact hosts/targets/SDKs and results; cross compile alone cannot satisfy a runtime claim; Aesynx remains future. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.131.0 — Performance and footprint review

**Setup:** Start from completed 0.130.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Performance and footprint review with explicit validation and resource contracts.

**Deliverables:** Measure complete equivalent workloads with minimal/manual/helper/full profiles, including native import.

**Verification:** Publish hardware/cache/tool versions, cold/warm CPU/wall/RSS, allocations and malformed-input behavior. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.132.0 — API and profile stabilization

**Setup:** Start from completed 0.131.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** API and profile stabilization with explicit validation and resource contracts.

**Deliverables:** Review ergonomics, error stability, ownership, feature additivity and all four version axes for 1.0.

**Verification:** Independent consumer migrations; additive-feature matrix; documentation examples and semantic API snapshot. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.133.0 — Supply-chain and package rehearsal

**Setup:** Start from completed 0.132.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Supply-chain and package rehearsal with explicit validation and resource contracts.

**Deliverables:** Produce verified archives, first-party dependency graph/SBOM, provenance and checksums using isolated staging.

**Verification:** Rebuild from archives without sibling repos; install every package in dependency order; verify all pins and metadata. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.134.0 — Independent security review

**Setup:** Start from completed 0.133.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Independent security review with explicit validation and resource contracts.

**Deliverables:** Obtain named external review of parser completeness, native spans, Unicode, emission and release process. Earlier resource-contract and native-provenance reviews are requested at their trust-boundary milestones; this final review integrates the complete product.

**Verification:** Publish scope, evidence and findings; no automation substitute or unverified independence claim. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.135.0 — Remediation and regression release

**Setup:** Start from completed 0.134.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Remediation and regression release with explicit validation and resource contracts.

**Deliverables:** Resolve review/campaign findings with focused fixes, split further whenever fixes are not reviewable together.

**Verification:** Each issue has a reproducer, regression, exact fixed revision and reviewer retest; no open release blockers. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.136.0 — Stable documentation and support contract

**Setup:** Start from completed 0.135.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Stable documentation and support contract with explicit validation and resource contracts.

**Deliverables:** Finalize capability/grammar/Unicode/platform matrices, security response policy and stable API guarantees.

**Verification:** Every public claim links to current evidence; all examples compile; unsupported/opaque boundaries remain explicit. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

### 0.137.0 — Production candidate freeze

**Setup:** Start from completed 0.136.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Production candidate freeze with explicit validation and resource contracts.

**Deliverables:** Freeze the complete admitted scope and prepare exact 1.0.0-rc.N source/artifact candidates.

**Verification:** All gates, independent findings closure, package rehearsal, native acceptance and candidate pentest must be current. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for a
candidate **pentest**; update the one permanent report through remediation and
retest. Commit the report with the final candidate, including later CI fixes.
Require PASS/PASS, zero open findings and user-confirmed green GitHub before tagging.

## Production release

### 1.0.0-rc.N — exact candidate

**Setup:** Complete 0.137.0 and freeze the 1.0 manifests, admitted API/grammar,
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

### 1.0.0 — production admission

**Setup:** Use the admitted RC implementation unchanged; keep the final pentest report committed with the reviewed candidate.
Any later candidate change requires the corresponding report update and retest.

**Goal:** First serious production-ready Synir crate family for its explicit scope.

**Deliverables:** Stable no_std core, usable native macro profile, source/Unicode,
full grammar, transformations, validated docs, signed provenance and support policy.

**Verification:** Exact artifacts correspond to the reviewed candidate; package
installation and public examples work; all capability claims have linked evidence.

**Exit criteria:** Final pentest scope confirmation and release authorization;
no unreviewed code delta, unresolved release blocker, or fabricated platform claim.
Publish the signed release and verify registry contents. Ongoing maintenance
retains freshness checks, regression tests and per-version pentests.

## Requirement coverage

Every row is a requirement, not an implementation claim. Owners are exact
implementation/acceptance passes. Before an owner exits, replace its planned
evidence with the test path, runnable command and report link in its scope
manifest. At capability acceptance every row must resolve to actual evidence.
Incremental IDE trees, macro expansion and semantic resolution are outside the
admitted product scope; bounded recovery for source inspection is explicitly owned.

| ID | Idea section | Owner versions | Acceptance evidence required | Evidence state |
| --- | --- | --- | --- | --- |
| REQ-01 | 1–2, final clarification | 0.1.0, 0.126.0, 0.127.0 | Workflow-based capability equivalence; no API compatibility promise. | Planned; foundation repository checks only are currently executable. |
| REQ-02 | 3, 9 | 0.2.0, 0.3.0, 0.4.0, 0.5.0, 0.6.0, 0.118.0 | Reject cross-budget resets and storage exhaustion; include deep clone/drop/debug. | Planned; foundation repository checks only are currently executable. |
| REQ-03 | 4–6 | 0.1.0, 0.23.0, 0.56.0, 0.67.0, 0.115.0 | Acyclic core/host/helper graph; no_std/no-alloc and additive profiles; compile-fail lifetime boundaries. | Planned; foundation repository checks only are currently executable. |
| REQ-04 | 7 | 0.7.0, 0.8.0, 0.9.0, 0.16.0, 0.17.0, 0.18.0 | Foreign ranges/delimiters fail; import preserves original tokens under explicit quotas. | Planned; foundation repository checks only are currently executable. |
| REQ-05 | 8 | 0.19.0, 0.20.0, 0.24.0 | Native spans/hygiene/None groups survive real compiler consumers; source offsets are separate. | Planned; foundation repository checks only are currently executable. |
| REQ-06 | 10–11 | 0.9.0, 0.10.0, 0.22.0, 0.25.0, 0.29.0, 0.30.0, 0.31.0, 0.32.0 | Complete parsing, progress and every field/variant; inspection/opaque types cannot imply deep validity. | Planned; foundation repository checks only are currently executable. |
| REQ-07 | 12 | 0.33.0, 0.34.0, 0.50.0 | Generic defaults removed only from impl projection; no unnecessary bounds or misplaced where clauses. | Planned; foundation repository checks only are currently executable. |
| REQ-08 | 13 | 0.40.0, 0.41.0, 0.42.0, 0.43.0, 0.44.0, 0.45.0 | Unknown/duplicate/conflicting or wrongly placed helper attributes fail with bounded deterministic errors. | Planned; foundation repository checks only are currently executable. |
| REQ-09 | 14.1 | 0.35.0, 0.36.0, 0.37.0, 0.38.0, 0.39.0 | Literal form/escape/overflow/suffix/C-string vectors with caller storage and decoded-byte limits. | Planned; foundation repository checks only are currently executable. |
| REQ-10 | 14.2–14.3 | 0.72.0, 0.73.0, 0.74.0, 0.75.0, 0.76.0, 0.77.0, 0.78.0 | Pinned licensed data; reproducible tables, XID classification and full bounded NFC conformance. | Planned; foundation repository checks only are currently executable. |
| REQ-11 | 15.1–15.2 | 0.46.0, 0.47.0, 0.48.0, 0.49.0 | Portable, native and source-text sinks; lexical separation, validated builders and explicit failed-sink semantics. | Planned; foundation repository checks only are currently executable. |
| REQ-12 | 15.3–15.5 | 0.50.0, 0.60.0, 0.98.0 | Explicit paths/spans, renamed crates, transactional high-level generation and precedence-safe typed expressions. | Planned; foundation repository checks only are currently executable. |
| REQ-13 | 16 | 0.58.0, 0.59.0, 0.61.0, 0.62.0, 0.63.0, 0.64.0 | Quotation escaping/scalars/groups/repetition/spans/jointness; mismatched sequence lengths fail. | Planned; foundation repository checks only are currently executable. |
| REQ-14 | 13.3, 16 | 0.65.0, 0.66.0, 0.67.0 | Generated schemas match handwritten semantics; manual builder remains independently usable. | Planned; foundation repository checks only are currently executable. |
| REQ-15 | 17 | 0.2.0, 0.3.0, 0.21.0, 0.44.0 | Redacted bounded compile_error output and primary/secondary locations; failure never emits empty success. | Planned; foundation repository checks only are currently executable. |
| REQ-16 | 18, 26 | 0.81.0, 0.82.0, 0.83.0, 0.84.0, 0.85.0, 0.86.0, 0.87.0, 0.88.0, 0.89.0, 0.90.0, 0.91.0, 0.92.0, 0.93.0, 0.94.0, 0.95.0, 0.96.0, 0.97.0, 0.99.0, 0.100.0, 0.101.0, 0.102.0, 0.103.0, 0.104.0, 0.105.0, 0.106.0, 0.107.0, 0.108.0, 0.109.0, 0.110.0, 0.111.0, 0.112.0 | Every admitted production/edition has positive and negative fixtures; unknown/unstable syntax stays explicit. | Planned; foundation repository checks only are currently executable. |
| REQ-17 | 18 | 0.113.0, 0.114.0, 0.115.0, 0.116.0, 0.117.0, 0.118.0 | Bounded visits/folds, safe owned conversion and failure-atomic transformations; deep operations tested. | Planned; foundation repository checks only are currently executable. |
| REQ-18 | 19 | 0.119.0, 0.120.0, 0.121.0, 0.122.0, 0.125.0 | Revision/UTF-8/overlap checks, no-op byte fidelity, explicit recovery and revalidation barriers. | Planned; foundation repository checks only are currently executable. |
| REQ-19 | 19 | 0.123.0, 0.124.0 | Explicit cfg and source loading; no hidden filesystem/environment authority. | Planned; foundation repository checks only are currently executable. |
| REQ-20 | 20 | 0.51.0, 0.52.0, 0.53.0, 0.54.0, 0.55.0, 0.56.0, 0.80.0, 0.126.0 | Compiled Describe/error/attribute/DSL/source/no-alloc examples, renames and no unintended runtime parser link. | Planned; foundation repository checks only are currently executable. |
| REQ-21 | 21–23 | 0.11.0, 0.12.0, 0.13.0, 0.14.0, 0.24.0, 0.25.0, 0.79.0, 0.128.0, 0.129.0 | Early fixture/generation/oracle/replay gates, all-compiler native behavior and continuous adversarial evidence. | Planned; foundation repository checks only are currently executable. |
| REQ-22 | 24 | 0.57.0, 0.67.0, 0.131.0 | Equivalent ordinary/selective upstream workloads; cold/warm CPU/wall/memory including import; no unsupported speedup. | Planned; foundation repository checks only are currently executable. |
| REQ-23 | 25 | 0.1.0, 0.133.0 | Committed-report gate before tags; archives, graph/provenance/checksums before publication; no independent-audit fabrication. | Planned; foundation repository checks only are currently executable. |
| REQ-24 | 26 | 0.2.0, 0.3.0, 0.68.0, 0.132.0, 0.136.0 | API, compiler, grammar and Unicode are separate contracts; security rejections documented. | Planned; foundation repository checks only are currently executable. |
| REQ-25 | 27–29 | 0.127.0, 0.134.0, 0.135.0, 0.137.0 | Actual complete capability evidence, independent final review, remediation and frozen production candidate. | Planned; foundation repository checks only are currently executable. |
| REQ-26 | User platform requirements | 0.1.0, 0.24.0, 0.80.0, 0.130.0 | Day-one portable checks, all-compiler native fixtures and actual platform runtime acceptance; Aesynx future only. | Planned; foundation repository checks only are currently executable. |

## Audit version mapping

All pre-1.0 milestones were unpublished when this audit split was made. The
workspace remains the 0.1.0 foundation candidate. Each original owner maps to
the complete replacement range below; original capability commitments are
retained. This map preserves references in the earlier audit discussion.

| Original version | Current owner versions |
| --- | --- |
| 0.1.0 | 0.1.0 |
| 0.2.0 | 0.2.0, 0.3.0 |
| 0.3.0 | 0.4.0 |
| 0.4.0 | 0.5.0 |
| 0.5.0 | 0.6.0 |
| 0.6.0 | 0.7.0 |
| 0.7.0 | 0.8.0 |
| 0.8.0 | 0.9.0 |
| 0.9.0 | 0.10.0 |
| 0.10.0 | 0.11.0, 0.12.0, 0.13.0, 0.14.0 |
| 0.11.0 | 0.15.0 |
| 0.12.0 | 0.16.0 |
| 0.13.0 | 0.17.0 |
| 0.14.0 | 0.18.0 |
| 0.15.0 | 0.19.0 |
| 0.16.0 | 0.20.0 |
| 0.17.0 | 0.21.0 |
| 0.18.0 | 0.22.0 |
| 0.19.0 | 0.23.0 |
| 0.20.0 | 0.24.0 |
| 0.21.0 | 0.25.0 |
| 0.22.0 | 0.26.0 |
| 0.23.0 | 0.27.0 |
| 0.24.0 | 0.28.0 |
| 0.25.0 | 0.29.0 |
| 0.26.0 | 0.30.0 |
| 0.27.0 | 0.31.0 |
| 0.28.0 | 0.32.0 |
| 0.29.0 | 0.33.0 |
| 0.30.0 | 0.34.0 |
| 0.31.0 | 0.35.0 |
| 0.32.0 | 0.36.0 |
| 0.33.0 | 0.37.0 |
| 0.34.0 | 0.38.0, 0.39.0 |
| 0.35.0 | 0.40.0 |
| 0.36.0 | 0.41.0 |
| 0.37.0 | 0.42.0 |
| 0.38.0 | 0.43.0 |
| 0.39.0 | 0.44.0 |
| 0.40.0 | 0.45.0 |
| 0.41.0 | 0.46.0 |
| 0.42.0 | 0.47.0, 0.48.0 |
| 0.43.0 | 0.49.0 |
| 0.44.0 | 0.50.0 |
| 0.45.0 | 0.51.0 |
| 0.46.0 | 0.52.0 |
| 0.47.0 | 0.53.0 |
| 0.48.0 | 0.54.0, 0.55.0 |
| 0.49.0 | 0.56.0 |
| 0.50.0 | 0.57.0 |
| 0.51.0 | 0.58.0 |
| 0.52.0 | 0.59.0 |
| 0.53.0 | 0.60.0 |
| 0.54.0 | 0.61.0 |
| 0.55.0 | 0.62.0 |
| 0.56.0 | 0.63.0 |
| 0.57.0 | 0.64.0 |
| 0.58.0 | 0.65.0 |
| 0.59.0 | 0.66.0 |
| 0.60.0 | 0.67.0 |
| 0.61.0 | 0.68.0 |
| 0.62.0 | 0.69.0 |
| 0.63.0 | 0.70.0 |
| 0.64.0 | 0.71.0 |
| 0.65.0 | 0.72.0 |
| 0.66.0 | 0.73.0 |
| 0.67.0 | 0.74.0, 0.75.0, 0.76.0, 0.77.0 |
| 0.68.0 | 0.78.0 |
| 0.69.0 | 0.79.0 |
| 0.70.0 | 0.80.0 |
| 0.71.0 | 0.81.0 |
| 0.72.0 | 0.82.0, 0.83.0, 0.84.0 |
| 0.73.0 | 0.85.0 |
| 0.74.0 | 0.86.0 |
| 0.75.0 | 0.87.0, 0.88.0, 0.89.0 |
| 0.76.0 | 0.90.0, 0.91.0 |
| 0.77.0 | 0.92.0, 0.93.0 |
| 0.78.0 | 0.94.0, 0.95.0 |
| 0.79.0 | 0.96.0, 0.97.0 |
| 0.80.0 | 0.98.0 |
| 0.81.0 | 0.99.0, 0.100.0 |
| 0.82.0 | 0.101.0, 0.102.0 |
| 0.83.0 | 0.103.0, 0.104.0 |
| 0.84.0 | 0.105.0, 0.106.0, 0.107.0 |
| 0.85.0 | 0.108.0, 0.109.0, 0.110.0 |
| 0.86.0 | 0.111.0 |
| 0.87.0 | 0.112.0 |
| 0.88.0 | 0.113.0 |
| 0.89.0 | 0.114.0 |
| 0.90.0 | 0.115.0 |
| 0.91.0 | 0.116.0 |
| 0.92.0 | 0.117.0 |
| 0.93.0 | 0.118.0 |
| 0.94.0 | 0.119.0 |
| 0.95.0 | 0.120.0 |
| 0.96.0 | 0.121.0, 0.122.0 |
| 0.97.0 | 0.123.0, 0.124.0 |
| 0.98.0 | 0.125.0 |
| 0.99.0 | 0.126.0 |
| 0.100.0 | 0.127.0 |
| 0.101.0 | 0.128.0 |
| 0.102.0 | 0.129.0 |
| 0.103.0 | 0.130.0 |
| 0.104.0 | 0.131.0 |
| 0.105.0 | 0.132.0 |
| 0.106.0 | 0.133.0 |
| 0.107.0 | 0.134.0 |
| 0.108.0 | 0.135.0 |
| 0.109.0 | 0.136.0 |
| 0.110.0 | 0.137.0 |
