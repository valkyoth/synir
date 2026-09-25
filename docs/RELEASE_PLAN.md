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

## Version map

| Versions | Detailed implementation passes |
| --- | --- |
| 0.1.0–0.10.0 | [Foundation and accounting](#foundation-and-accounting) |
| 0.11.0–0.20.0 | [Native tokens and portable views](#native-tokens-and-portable-views) |
| 0.21.0–0.30.0 | [Declaration recognition and generic projections](#declaration-recognition-and-generic-projections) |
| 0.31.0–0.40.0 | [Literal decoding and typed attributes](#literal-decoding-and-typed-attributes) |
| 0.41.0–0.50.0 | [Fallible emission and usable macro profile](#fallible-emission-and-usable-macro-profile) |
| 0.51.0–0.60.0 | [Quotation and generated schemas](#quotation-and-generated-schemas) |
| 0.61.0–0.70.0 | [Source frontend and Unicode](#source-frontend-and-unicode) |
| 0.71.0–0.80.0 | [Expanded types, expressions and patterns](#expanded-types-expressions-and-patterns) |
| 0.81.0–0.90.0 | [Full items and traversal](#full-items-and-traversal) |
| 0.91.0–0.100.0 | [Owned syntax and lossless edits](#owned-syntax-and-lossless-edits) |
| 0.101.0–0.110.0 | [Security, performance and production admission](#security-performance-and-production-admission) |
| 1.0.0-rc.N | [Exact stable candidate validation and retest](#100-rcn--exact-candidate) |
| 1.0.0 | [First serious production-ready release](#100--production-admission) |

## Foundation and accounting

### 0.1.0 — Workspace and assurance setup

**Setup:** Start from initial repository and supplied idea; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Workspace and assurance setup with explicit validation and resource contracts.

**Deliverables:** Establish the four-crate DAG, licenses, separate READMEs, tool pins, CI, policies and complete roadmap.

**Verification:** Run local, compiler and target gates; review dependency rejection tests and package contents. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.2.0 — Contract inventory and errors

**Setup:** Start from completed 0.1.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Contract inventory and errors with explicit validation and resource contracts.

**Deliverables:** Define typed portable error codes and the inspect/validate/recover/opaque contract inventory; pin the ten adversarial fixture inputs.

**Verification:** Check deterministic error identity, no input disclosure, and each fixture owner; no parser success claims. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.3.0 — Explicit resource limits

**Setup:** Start from completed 0.2.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Explicit resource limits with explicit validation and resource contracts.

**Deliverables:** Define units and checked limits for bytes, tokens, depth, nodes, decoded data, storage, work, output and diagnostics.

**Verification:** Test zero/exact/one-over boundaries and usize/u64 overflow on 32-bit and 64-bit targets. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.4.0 — Monotonic work budget

**Setup:** Start from completed 0.3.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Monotonic work budget with explicit validation and resource contracts.

**Deliverables:** Implement one invocation fuel ledger shared by child operations; charge before action.

**Verification:** Exhaust fuel during nested operations; failed work cannot wrap, reset or regain credit. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.5.0 — Caller-owned storage

**Setup:** Start from completed 0.4.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Caller-owned storage with explicit validation and resource contracts.

**Deliverables:** Implement safe initialized-slot storage with fallible insertion and occupancy accounting.

**Verification:** Test empty/exact/full buffers, failed insertion, reuse and no allocator linkage. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.6.0 — Token identities and provenance

**Setup:** Start from completed 0.5.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Token identities and provenance with explicit validation and resource contracts.

**Deliverables:** Introduce private store-bound token/source identities and checked range constructors.

**Verification:** Reject foreign IDs, inverted ranges, overflow and stale source identities; compile-fail forging attempts. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.7.0 — Flat portable token tape

**Setup:** Start from completed 0.6.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Flat portable token tape with explicit validation and resource contracts.

**Deliverables:** Define token kinds, punctuation spacing and checked delimiter links in flat caller storage.

**Verification:** Reject unmatched/crossed groups and invalid indices; traverse deepest allowed tape iteratively. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.8.0 — Cursor and checkpoints

**Setup:** Start from completed 0.7.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Cursor and checkpoints with explicit validation and resource contracts.

**Deliverables:** Add bounded lookahead, checkpoints and explicit complete versus prefix consumption.

**Verification:** Rollback restores position but never fuel; trailing input and out-of-range seeks fail. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.9.0 — Progress-safe parser combinators

**Setup:** Start from completed 0.8.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Progress-safe parser combinators with explicit validation and resource contracts.

**Deliverables:** Implement alternatives, optional and separated repetition with explicit trailing-separator rules.

**Verification:** Zero-progress success fails repetition; adversarial failed alternatives exhaust the shared budget. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.10.0 — Deterministic assurance runner

**Setup:** Start from completed 0.9.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Deterministic assurance runner with explicit validation and resource contracts.

**Deliverables:** Add Rust/std compile-pass/fail runner, bounded corpus generation, process limits and replay metadata.

**Verification:** Exercise successful and failing fixtures, timeout and nonzero compiler exit; runners must fail on missing evidence. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Native tokens and portable views

### 0.11.0 — Host invocation ownership

**Setup:** Start from completed 0.10.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Host invocation ownership with explicit validation and resource contracts.

**Deliverables:** Define invocation-local handle storage with explicit compiler-thread and lifetime restrictions.

**Verification:** Compile-fail escaped/foreign handles; test contracts through real compiler fixtures. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.12.0 — Native leaf import

**Setup:** Start from completed 0.11.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native leaf import with explicit validation and resource contracts.

**Deliverables:** Import identifiers, literals and punctuation preserving original handles, spans and spacing.

**Verification:** Macro fixtures compare joint operators, lifetimes, negative literals and raw identifiers; charge spelling extraction. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.13.0 — Native group import

**Setup:** Start from completed 0.12.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native group import with explicit validation and resource contracts.

**Deliverables:** Traverse native groups into the flat tape with explicit depth/token/storage accounting.

**Verification:** Deep and wide groups exhaust each limit cleanly; no recursive clone/drop path introduced. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.14.0 — Native spelling cache

**Setup:** Start from completed 0.13.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native spelling cache with explicit validation and resource contracts.

**Deliverables:** Cache per-token spelling under byte/work/storage quotas without stringify-and-reparse.

**Verification:** Repeated cache hits/misses preserve identity and charge correctly; long spellings fail boundedly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.15.0 — Native span policies

**Setup:** Start from completed 0.14.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native span policies with explicit validation and resource contracts.

**Deliverables:** Separate source offsets from invocation spans, opening/closing spans and resolution policy.

**Verification:** Real diagnostic locations and hygiene fixtures; no behavior depends on optional span source_text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.16.0 — Opaque forwarding

**Setup:** Start from completed 0.15.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Opaque forwarding with explicit validation and resource contracts.

**Deliverables:** Preserve original groups and token provenance with an explicit reason for opacity.

**Verification:** Compile macro_rules forwarding with Delimiter::None; compare precedence and original-token identity. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.17.0 — Bounded diagnostic rendering

**Setup:** Start from completed 0.16.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Bounded diagnostic rendering with explicit validation and resource contracts.

**Deliverables:** Render portable errors through caller sinks and native compile_error tokens; cap and redact data.

**Verification:** Overflow during error reporting still yields bounded failure; escape control characters and suppress literals/paths. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.18.0 — Inspection views

**Setup:** Start from completed 0.17.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Inspection views with explicit validation and resource contracts.

**Deliverables:** Expose header-only views distinct from validated shapes and opaque/recovered regions.

**Verification:** Malformed tails never acquire validated types; consuming a header cannot authorize high-level generation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.19.0 — Borrowing ergonomics

**Setup:** Start from completed 0.18.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Borrowing ergonomics with explicit validation and resource contracts.

**Deliverables:** Separate immutable input from mutable work/output and prototype nested user parser contexts.

**Verification:** Compile realistic borrowed views through decoding/emission contexts; callbacks cannot reset internal counters. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.20.0 — Native foundation acceptance

**Setup:** Start from completed 0.19.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native foundation acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate import, views, diagnostic failures and unchanged forwarding in real consumer fixtures.

**Verification:** Run MSRV/current stable, no_std target consumers, host OS tests and hostile group corpus. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Declaration recognition and generic projections

### 0.21.0 — Item prefixes and visibility

**Setup:** Start from completed 0.20.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Item prefixes and visibility with explicit validation and resource contracts.

**Deliverables:** Recognize ordered outer attributes, visibility and item introducers; never scan past unexpected prefixes.

**Verification:** Reject skipped leading syntax; test pub, restricted visibility, raw names and preserved unrelated attributes. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.22.0 — Path and generic-argument boundaries

**Setup:** Start from completed 0.21.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Path and generic-argument boundaries with explicit validation and resource contracts.

**Deliverables:** Recognize qualified paths, nested arguments and associated bindings without comma scanning shortcuts.

**Verification:** Result<T,E>, qualified paths, turbofish and joint punctuation have exact ranges or explicit unsupported errors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.23.0 — Reference pointer and tuple type boundaries

**Setup:** Start from completed 0.22.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Reference pointer and tuple type boundaries with explicit validation and resource contracts.

**Deliverables:** Recognize references, raw pointers, tuple/group types and slices over token views.

**Verification:** Nested commas/lifetimes, invalid mutability and malformed delimiters have precise complete-consumption errors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.24.0 — Function and bounded type regions

**Setup:** Start from completed 0.23.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Function and bounded type regions with explicit validation and resource contracts.

**Deliverables:** Recognize function pointers, higher-ranked binders, trait-object bounds and array const regions with explicit opacity.

**Verification:** Fn(T)->Result<T,E> arrows never close generic lists; unsupported const internals cannot become validated expressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.25.0 — Named structs

**Setup:** Start from completed 0.24.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Named structs with explicit validation and resource contracts.

**Deliverables:** Parse every named field and attached attributes using grammar-aware type boundaries.

**Verification:** Nested type commas, missing colons, duplicates as syntax policy, malformed last field and early iterator stop. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.26.0 — Tuple and unit structs

**Setup:** Start from completed 0.25.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Tuple and unit structs with explicit validation and resource contracts.

**Deliverables:** Add tuple/unit forms and their item-specific terminators/where positions.

**Verification:** Tuple where clauses before semicolon, empty tuples, forbidden separators and trailing junk. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.27.0 — Enum variants

**Setup:** Start from completed 0.26.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Enum variants with explicit validation and resource contracts.

**Deliverables:** Recognize unit/tuple/record variants and explicit discriminant regions with bounded shape validation.

**Verification:** Every variant/field counted; malformed final variant rejected even if iteration stops early. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.28.0 — Union shapes

**Setup:** Start from completed 0.27.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Union shapes with explicit validation and resource contracts.

**Deliverables:** Parse union declarations with an explicit caller derive policy and no implicit field access.

**Verification:** Reject invalid union shapes; acceptance example policy rejects unsupported derive operations clearly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.29.0 — Generic parameter and where models

**Setup:** Start from completed 0.28.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generic parameter and where models with explicit validation and resource contracts.

**Deliverables:** Parse lifetime/type/const parameters, bounds, defaults and grammar-aware where predicates.

**Verification:** Where without generics, function trait bounds, raw parameters, empty lists, const defaults and tuple placement. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.30.0 — Validated impl projections

**Setup:** Start from completed 0.29.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Validated impl projections with explicit validation and resource contracts.

**Deliverables:** Produce declaration/impl/type-argument projections and explicit bound-policy plans from complete shapes.

**Verification:** Defaults removed only for impl parameters; predicates/spans/order preserved; generated minimal impls compile. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Literal decoding and typed attributes

### 0.31.0 — Literal classification

**Setup:** Start from completed 0.30.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Literal classification with explicit validation and resource contracts.

**Deliverables:** Distinguish all admitted Rust literal token forms and preserve original spellings and suffixes.

**Verification:** Reject wrong literal kind and malformed prefix/suffix; document unsupported forms explicitly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.32.0 — String decoding

**Setup:** Start from completed 0.31.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** String decoding with explicit validation and resource contracts.

**Deliverables:** Decode escaped and raw strings into caller buffers with decoded-byte/work accounting.

**Verification:** Escape, continuation, raw delimiter and UTF-8 boundary vectors; exact output capacity and one-over failure. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.33.0 — Byte and character decoding

**Setup:** Start from completed 0.32.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Byte and character decoding with explicit validation and resource contracts.

**Deliverables:** Decode byte strings/bytes/chars with form-specific Unicode and length checks.

**Verification:** Reject multi-scalar char, out-of-range byte and invalid escapes; no silent fallback to text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.34.0 — Numeric and C-string decoding

**Setup:** Start from completed 0.33.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Numeric and C-string decoding with explicit validation and resource contracts.

**Deliverables:** Add checked integer schema conversion and C-string validation; preserve floats until explicit conversion policy.

**Verification:** Overflow, signs, bases, suffixes and interior NUL vectors; unsupported float conversion fails explicitly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.35.0 — Attribute syntax

**Setup:** Start from completed 0.34.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Attribute syntax with explicit validation and resource contracts.

**Deliverables:** Parse paths, flags, assignments, nested lists and explicit opaque payloads separately from schema policy.

**Verification:** Unexpected values never become flags; trailing tokens and nesting exhaustion fail; preserve unrelated attributes. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.36.0 — Strict scalar schemas

**Setup:** Start from completed 0.35.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Strict scalar schemas with explicit validation and resource contracts.

**Deliverables:** Add handwritten schemas for booleans, strings, checked integers, paths and choices.

**Verification:** Wrong type, unknown key, misspelling, missing required value and malformed optional values all error. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.37.0 — Duplicate and conflict policies

**Setup:** Start from completed 0.36.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Duplicate and conflict policies with explicit validation and resource contracts.

**Deliverables:** Enforce singleton/repeatable cardinality, defaults and mutually exclusive keys.

**Verification:** No last-write-wins accident; explicit repeat ordering; duplicate/conflict diagnostics retain both locations. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.38.0 — Nested context schemas

**Setup:** Start from completed 0.37.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Nested context schemas with explicit validation and resource contracts.

**Deliverables:** Support bounded nested records and separate container/variant/field/generic contexts.

**Verification:** Misplaced keys, deep records and too many repeated values fail with bounded deterministic output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.39.0 — Custom validation and diagnostics

**Setup:** Start from completed 0.38.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Custom validation and diagnostics with explicit validation and resource contracts.

**Deliverables:** Add caller validators with documented external-work boundary and bounded independent error aggregation.

**Verification:** Callbacks cannot forge validated state; source-order diagnostics, truncation and error-budget exhaustion. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.40.0 — Schema acceptance

**Setup:** Start from completed 0.39.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Schema acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate literal decoding, strict schemas and derive shapes with validated attribute values.

**Verification:** Run hostile configuration corpus and context matrix; no malformed attribute activates behavior. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Fallible emission and usable macro profile

### 0.41.0 — Transactional token sink

**Setup:** Start from completed 0.40.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Transactional token sink with explicit validation and resource contracts.

**Deliverables:** Implement explicit start/commit/discard output with token/byte limits and reserved error capacity.

**Verification:** Any mid-output failure returns error without publishing a truncated successful stream. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.42.0 — Checked token builders

**Setup:** Start from completed 0.41.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Checked token builders with explicit validation and resource contracts.

**Deliverables:** Construct identifiers, supported punctuation, groups and typed literals; reject untrusted invalid names.

**Verification:** Invalid Ident input is rejected before compiler constructors; strings emit data, not executable text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.43.0 — Native export

**Setup:** Start from completed 0.42.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native export with explicit validation and resource contracts.

**Deliverables:** Export portable constructions with explicit span policy while forwarding original native objects unchanged.

**Verification:** Group/spacing/span fixtures and output exhaustion through real proc macros; document compiler allocation boundary. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.44.0 — Generic impl emitter

**Setup:** Start from completed 0.43.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generic impl emitter with explicit validation and resource contracts.

**Deliverables:** Emit validated impl plans with projected generics, original predicates and explicit trait/dependency paths.

**Verification:** Compile lifetime/default/const/where examples; no incidental Clone/Debug bounds; renamed runtime paths work. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.45.0 — Describe structs acceptance

**Setup:** Start from completed 0.44.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Describe structs acceptance with explicit validation and resource contracts.

**Deliverables:** Implement a real metadata derive for named, tuple and unit structs using the manual pipeline.

**Verification:** Compile and execute metadata tests; no_std target contains only intended trait/data, no parser runtime. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.46.0 — Describe enum and union policy

**Setup:** Start from completed 0.45.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Describe enum and union policy with explicit validation and resource contracts.

**Deliverables:** Extend Describe to enums and document explicit union behavior and field options.

**Verification:** Wrong attributes and unsupported union behavior emit deliberate errors; no omitted variants/fields. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.47.0 — Declaration-preserving attribute macro

**Setup:** Start from completed 0.46.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Declaration-preserving attribute macro with explicit validation and resource contracts.

**Deliverables:** Provide a compiled function attribute example retaining opaque bodies and unrelated attributes.

**Verification:** Body/provenance unchanged; malformed owned options and unsupported signatures fail without body truncation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.48.0 — Error derive and custom DSL examples

**Setup:** Start from completed 0.47.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Error derive and custom DSL examples with explicit validation and resource contracts.

**Deliverables:** Add an independent error-type derive workflow and a small custom language acceptance fixture.

**Verification:** Verify distinct generic-bound policies, deliberate failures and progress-safe custom parsing. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.49.0 — Macro-development profile

**Setup:** Start from completed 0.48.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro-development profile with explicit validation and resource contracts.

**Deliverables:** Expose a documented additive macro-dev preset for implemented derive/attribute/manual emission capabilities.

**Verification:** Feature-isolated builds, renamed facade/runtime dependencies, namespace shadowing and no_std consumers. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.50.0 — Macro workflow acceptance

**Setup:** Start from completed 0.49.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro workflow acceptance with explicit validation and resource contracts.

**Deliverables:** Measure and review end-to-end native import to validated plan to output on several real use cases.

**Verification:** Publish boundedness and cold/warm benchmark methodology; run independent compiler fixtures and pentest. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Quotation and generated schemas

### 0.51.0 — Quotation language specification

**Setup:** Start from completed 0.50.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Quotation language specification with explicit validation and resource contracts.

**Deliverables:** Specify a clean-slate template grammar, marker escaping and builder lowering independent of Quote API compatibility.

**Verification:** Ambiguous templates, literal markers and unsupported constructs have explicit diagnostics and bounded parsing. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.52.0 — Scalar interpolation

**Setup:** Start from completed 0.51.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Scalar interpolation with explicit validation and resource contracts.

**Deliverables:** Lower scalar token/literal/view interpolation into fallible manual builder calls.

**Verification:** Data/code distinction, nested groups and runtime errors preserve transactional output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.53.0 — Renamed-crate hygiene

**Setup:** Start from completed 0.52.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Renamed-crate hygiene with explicit validation and resource contracts.

**Deliverables:** Implement tested framework-path plumbing and explicit span selection for generated builder code.

**Verification:** Rename Synir, shadow common names and combine multiple dependency aliases in real compilation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.54.0 — Simple repetition

**Setup:** Start from completed 0.53.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Simple repetition with explicit validation and resource contracts.

**Deliverables:** Implement bounded zero-or-more/nonempty/optional repetitions and separators.

**Verification:** Empty, one, many, missing item and trailing-separator cases; work/output counters cover every iteration. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.55.0 — Nested and multiple repetition

**Setup:** Start from completed 0.54.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Nested and multiple repetition with explicit validation and resource contracts.

**Deliverables:** Define lexical repetition scopes and exact finite sequence length matching.

**Verification:** Mismatched lengths error, never zip-truncate; deeply nested output growth exhausts shared limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.56.0 — Punctuation and lifetime quotation

**Setup:** Start from completed 0.55.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Punctuation and lifetime quotation with explicit validation and resource contracts.

**Deliverables:** Preserve jointness, lifetimes, negative literals and delimiter semantics through templates.

**Verification:** Native compilation distinguishes operator token fusion and invisible-group precedence regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.57.0 — Quotation resource acceptance

**Setup:** Start from completed 0.56.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Quotation resource acceptance with explicit validation and resource contracts.

**Deliverables:** Bound template compilation as well as generated execution and eliminate recursive expansion blowups.

**Verification:** Large templates, escaped markers and pathological repetition have measured bounded failures. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.58.0 — Generated schema decoder core

**Setup:** Start from completed 0.57.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generated schema decoder core with explicit validation and resource contracts.

**Deliverables:** Generate the same strict scalar decoder semantics as handwritten schemas without self-bootstrap dependencies.

**Verification:** Compare handwritten/generated behavior for valid and malformed options; helper crate never depends on facade. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.59.0 — Generated nested schema support

**Setup:** Start from completed 0.58.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generated nested schema support with explicit validation and resource contracts.

**Deliverables:** Extend schema helpers to contexts, nested/repeated values, forwarding and custom validation hooks.

**Verification:** Parity for duplicate/unknown/conflict diagnostics and locations; code-size and budget regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.60.0 — Helper workflow acceptance

**Setup:** Start from completed 0.59.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Helper workflow acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate quotation and generated schemas as optional conveniences over the working manual path.

**Verification:** Run all helper feature combinations and real fixtures; publish manual-versus-helper compile cost. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Source frontend and Unicode

### 0.61.0 — Source contract and trivia

**Setup:** Start from completed 0.60.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source contract and trivia with explicit validation and resource contracts.

**Deliverables:** Pin grammar/edition sources; introduce borrowed UTF-8 source and lossless trivia ranges.

**Verification:** Invalid byte input is rejected explicitly; no-op traversal preserves bytes and source identity. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.62.0 — ASCII identifiers and punctuation

**Setup:** Start from completed 0.61.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** ASCII identifiers and punctuation with explicit validation and resource contracts.

**Deliverables:** Implement an explicitly restricted ASCII lexer with raw/keyword/edition rules and punctuation jointness.

**Verification:** Reject non-ASCII identifiers as unsupported; no complete-Rust lexer claim; reserved prefixes and lifetimes tested. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.63.0 — Comments and groups

**Setup:** Start from completed 0.62.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Comments and groups with explicit validation and resource contracts.

**Deliverables:** Lex nested block/line/doc comments and validated delimiters with iterative depth accounting.

**Verification:** Unclosed/deep comments, delimiters inside comments and trivia boundaries consume bounded work. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.64.0 — Source literal scanning

**Setup:** Start from completed 0.63.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source literal scanning with explicit validation and resource contracts.

**Deliverables:** Recognize strings/raw strings, chars, byte/C strings and numeric tokens into borrowed ranges.

**Verification:** Escapes, raw hashes, lifetime/char ambiguity, malformed exponent/suffix and oversized literals. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.65.0 — Unicode data generation

**Setup:** Start from completed 0.64.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unicode data generation with explicit validation and resource contracts.

**Deliverables:** Pin official XID and normalization data, checksums, licenses and an offline Rust generator; shard tables.

**Verification:** Regeneration byte equality, input checksum mismatch rejection and every generated file at most 500 lines. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.66.0 — Unicode identifier classification

**Setup:** Start from completed 0.65.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unicode identifier classification with explicit validation and resource contracts.

**Deliverables:** Implement XID_Start/XID_Continue with ASCII fast path and edition/raw identifier restrictions.

**Verification:** Exhaustive scalar classification against pinned data; invalid scalar boundaries and table edge vectors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.67.0 — NFC normalization

**Setup:** Start from completed 0.66.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** NFC normalization with explicit validation and resource contracts.

**Deliverables:** Implement bounded decomposition, canonical ordering and composition with caller storage.

**Verification:** Official normalization tests, Hangul and adversarial combining sequences; output/work exhaustion and idempotence. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.68.0 — Canonical identifier identity

**Setup:** Start from completed 0.67.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Canonical identifier identity with explicit validation and resource contracts.

**Deliverables:** Keep original spelling separate from normalized identity and compiler-normalized tokens.

**Verification:** Canonically equivalent source identifiers compare as defined; source bytes unchanged; native/source differences triaged. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.69.0 — Source/native conformance

**Setup:** Start from completed 0.68.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source/native conformance with explicit validation and resource contracts.

**Deliverables:** Run the same supported declaration/schema grammar against source and native token frontends.

**Verification:** Compare normalized structure, never lost whitespace; grammar/Unicode/compiler version axes recorded. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.70.0 — Standalone and no-allocator acceptance

**Setup:** Start from completed 0.69.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Standalone and no-allocator acceptance with explicit validation and resource contracts.

**Deliverables:** Ship source inspection and caller-storage parser examples with optional fallible owned storage.

**Verification:** Bare-metal and mobile target checks; tiny/exact buffers; alloc reservation errors and no implicit file loading. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Expanded types, expressions and patterns

### 0.71.0 — Complete type grammar inventory

**Setup:** Start from completed 0.70.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Complete type grammar inventory with explicit validation and resource contracts.

**Deliverables:** Close supported Rust type-production gaps and record opaque macro/unstable policies against pinned reference.

**Verification:** Matrix names every type form; unsupported syntax cannot be mislabeled as a validated type. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.72.0 — Type grammar completion

**Setup:** Start from completed 0.71.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Type grammar completion with explicit validation and resource contracts.

**Deliverables:** Implement remaining qualified, bare-function, impl-trait, trait-object and associated-constraint productions.

**Verification:** Production-specific positive/negative/compiler corpus; bounded speculation and nested generic regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.73.0 — Expression atoms and paths

**Setup:** Start from completed 0.72.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Expression atoms and paths with explicit validation and resource contracts.

**Deliverables:** Recognize expression literals, paths, groups and explicit opaque macro invocations.

**Verification:** No semantic resolution claims; distinguish malformed paths from opaque macro bodies. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.74.0 — Postfix expressions

**Setup:** Start from completed 0.73.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Postfix expressions with explicit validation and resource contracts.

**Deliverables:** Add calls, method calls, field/index/await/try operators under explicit edition rules.

**Verification:** Chained syntax, turbofish and malformed argument separators; complete consumption and limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.75.0 — Unary binary and assignment expressions

**Setup:** Start from completed 0.74.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unary binary and assignment expressions with explicit validation and resource contracts.

**Deliverables:** Implement precedence/associativity-aware operators, ranges, casts and assignment forms.

**Verification:** Operator precedence corpus, exact tree structure and context-specific restrictions; no angle counting guesses. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.76.0 — Block and control expressions

**Setup:** Start from completed 0.75.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Block and control expressions with explicit validation and resource contracts.

**Deliverables:** Add blocks, if/loop/while/for/break/continue/return forms with explicit grammar contexts.

**Verification:** Dangling else, labels, let conditions and nested control-flow boundaries under work/depth limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.77.0 — Closure async and special expressions

**Setup:** Start from completed 0.76.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Closure async and special expressions with explicit validation and resource contracts.

**Deliverables:** Add closures, async/const/unsafe blocks and remaining admitted special expression forms.

**Verification:** Edition-specific grammar, closure parameter/type ambiguity and bounded parser alternatives. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.78.0 — Pattern atoms and destructuring

**Setup:** Start from completed 0.77.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Pattern atoms and destructuring with explicit validation and resource contracts.

**Deliverables:** Implement bindings, wildcards, literals, paths, tuples, arrays and record patterns.

**Verification:** Rest position, reference/mut binding syntax and malformed destructuring; no duplicate omission. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.79.0 — Pattern alternatives and match

**Setup:** Start from completed 0.78.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Pattern alternatives and match with explicit validation and resource contracts.

**Deliverables:** Add ranges, or-patterns, guards and match arms integrating the expression recognizer.

**Verification:** Precedence, nested alternatives, guards and arm comma rules; contextual invalid patterns fail. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.80.0 — Typed expression emission

**Setup:** Start from completed 0.79.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Typed expression emission with explicit validation and resource contracts.

**Deliverables:** Emit validated expressions with conservative precedence/associativity grouping.

**Verification:** Compile precedence-sensitive generated programs; raw token interpolation makes no typed-expression promise. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Full items and traversal

### 0.81.0 — Statements and local bindings

**Setup:** Start from completed 0.80.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Statements and local bindings with explicit validation and resource contracts.

**Deliverables:** Implement let/let-else, expression/item statements and semicolon distinctions.

**Verification:** Diverging else syntax and tail-expression boundaries; malformed local patterns/initializers rejected. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.82.0 — Function and signature grammar

**Setup:** Start from completed 0.81.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Function and signature grammar with explicit validation and resource contracts.

**Deliverables:** Complete qualifiers, ABI, receivers, variadics and signatures against the supported grammar.

**Verification:** Illegal qualifier sequences and receiver contexts; preserve spans and exact body boundaries. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.83.0 — Trait and impl items

**Setup:** Start from completed 0.82.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Trait and impl items with explicit validation and resource contracts.

**Deliverables:** Recognize trait/impl associated items, bounds and context-specific member syntax.

**Verification:** Invalid member contexts, where placement, negative/unsafe impl grammar and default items. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.84.0 — Modules imports and foreign items

**Setup:** Start from completed 0.83.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Modules imports and foreign items with explicit validation and resource contracts.

**Deliverables:** Implement module/use/extern productions without loading referenced files.

**Verification:** Nested use trees, glob/rename cases, inline versus external modules; no filesystem side effects. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.85.0 — Aliases constants statics and remaining items

**Setup:** Start from completed 0.84.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Aliases constants statics and remaining items with explicit validation and resource contracts.

**Deliverables:** Close the admitted item inventory including alias/type contexts, const/static and remaining stable forms.

**Verification:** Every remaining reference production gets a named fixture; unsupported unstable forms are explicit. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.86.0 — Macro syntax representation

**Setup:** Start from completed 0.85.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro syntax representation with explicit validation and resource contracts.

**Deliverables:** Represent macro_rules definitions and invocations with validated outer structure and explicit opaque DSL bodies.

**Verification:** Metavariables/repetition delimiters preserved; no expansion or fabricated validation of arbitrary DSL contents. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.87.0 — Full grammar acceptance

**Setup:** Start from completed 0.86.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Full grammar acceptance with explicit validation and resource contracts.

**Deliverables:** Reconcile edition/production matrix, grammar revision and native/source support for all admitted stable Rust syntax.

**Verification:** Pinned compiler/independent-parser corpus triage; no missing production hidden in a percentage claim. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.88.0 — Borrowed visits

**Setup:** Start from completed 0.87.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Borrowed visits with explicit validation and resource contracts.

**Deliverables:** Add typed iterative traversal with explicit limits over validated syntax views and opaque regions.

**Verification:** Every node visited as specified, cancellation/exhaustion bounded, deep traversal no stack overflow. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.89.0 — Structural mapping and folds

**Setup:** Start from completed 0.88.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Structural mapping and folds with explicit validation and resource contracts.

**Deliverables:** Add checked transformation plans with validation before mutation and provenance-aware replacements.

**Verification:** Failure atomicity, foreign ranges, generated syntax validity and unchanged input on rejected edits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.90.0 — Full-profile acceptance

**Setup:** Start from completed 0.89.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Full-profile acceptance with explicit validation and resource contracts.

**Deliverables:** Expose full grammar as an additive optional profile and publish source-analysis/transformation workflows.

**Verification:** Minimal macro profile cost unchanged; full profile production matrix and independent consumer fixtures pass. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Owned syntax and lossless edits

### 0.91.0 — Owned arena storage

**Setup:** Start from completed 0.90.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned arena storage with explicit validation and resource contracts.

**Deliverables:** Add flat owned syntax arenas with fallible reservation and private source/node identities.

**Verification:** Allocation-size overflow, exhaustion, failed insertion and cross-arena IDs; no recursive destruction. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.92.0 — Owned view conversion

**Setup:** Start from completed 0.91.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned view conversion with explicit validation and resource contracts.

**Deliverables:** Convert validated borrowed views into owned nodes under shared limits without losing opacity/provenance.

**Verification:** Round-trip normalized structure and span origin; stale native handles cannot be serialized. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.93.0 — Deep operation safety

**Setup:** Start from completed 0.92.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Deep operation safety with explicit validation and resource contracts.

**Deliverables:** Audit clone/drop/debug/equality/hash of all owned token and syntax structures.

**Verification:** Adversarial depth on small-stack worker processes; every operation has bounded traversal/output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.94.0 — Revision-bound edit plans

**Setup:** Start from completed 0.93.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Revision-bound edit plans with explicit validation and resource contracts.

**Deliverables:** Define source identity/revision, UTF-8 ranges and deterministic insertion order.

**Verification:** Reject wrong source/revision even at matching length; invalid UTF-8 boundary, inverted and overflow ranges. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.95.0 — Edit conflict validation

**Setup:** Start from completed 0.94.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Edit conflict validation with explicit validation and resource contracts.

**Deliverables:** Check overlap, duplicate replacement and equal-offset insertion policies before commit.

**Verification:** Permutation/conflict corpus and failure atomicity; no container-order-dependent winners. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.96.0 — Lossless edit application

**Setup:** Start from completed 0.95.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Lossless edit application with explicit validation and resource contracts.

**Deliverables:** Apply validated edits in one pass to caller sinks or budgeted owned output.

**Verification:** No-op byte identity, untouched comments/trivia, exact output limit and no partial successful artifact. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.97.0 — Explicit configuration and file boundaries

**Setup:** Start from completed 0.96.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Explicit configuration and file boundaries with explicit validation and resource contracts.

**Deliverables:** Design application-supplied cfg/source loading contexts without implicit environment/filesystem access.

**Verification:** Unresolved cfg preserved; denied loads and hostile paths remain application errors, not hidden parser I/O. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.98.0 — Owned transformation acceptance

**Setup:** Start from completed 0.97.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned transformation acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate full traversal, owned plans and lossless edits in standalone source tools.

**Verification:** Compiler-valid transformed fixtures, error propagation, caller-storage and alloc variants. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.99.0 — Workflow migration guides

**Setup:** Start from completed 0.98.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Workflow migration guides with explicit validation and resource contracts.

**Deliverables:** Publish task-oriented guides for derive, schema, quote, custom DSL, source analysis and transformations.

**Verification:** Compile every example with dependency renames/no_std where relevant; no API compatibility promises. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.100.0 — Capability replacement acceptance

**Setup:** Start from completed 0.99.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Capability replacement acceptance with explicit validation and resource contracts.

**Deliverables:** Reconcile all idea sections and practical replacement workflows with real downstream acceptance projects.

**Verification:** No descriptor/mock-only claims; parser, builder, helper and full/edit profiles each have independent consumers. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Security, performance and production admission

### 0.101.0 — Hostile-input campaign

**Setup:** Start from completed 0.100.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Hostile-input campaign with explicit validation and resource contracts.

**Deliverables:** Expand deterministic mutation/generation across tokens, grammar, literals, schemas, quoting and edits.

**Verification:** Run with external CPU/memory/time limits; replay seeds and record worst-case work/storage growth. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.102.0 — Independent differential campaign

**Setup:** Start from completed 0.101.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Independent differential campaign with explicit validation and resource contracts.

**Deliverables:** Compare admitted syntax and emitted behavior against pinned independent compiler/parser tools.

**Verification:** Triage every disagreement; semantic errors separated from syntax; save reproducible regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.103.0 — Platform runtime acceptance

**Setup:** Start from completed 0.102.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Platform runtime acceptance with explicit validation and resource contracts.

**Deliverables:** Run real consumers on Linux/Windows/macOS/FreeBSD and Android/iOS device or emulator/simulator setups.

**Verification:** Record exact hosts/targets/SDKs and results; cross compile alone cannot satisfy a runtime claim; Aesynx remains future. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.104.0 — Performance and footprint review

**Setup:** Start from completed 0.103.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Performance and footprint review with explicit validation and resource contracts.

**Deliverables:** Measure complete equivalent workloads with minimal/manual/helper/full profiles, including native import.

**Verification:** Publish hardware/cache/tool versions, cold/warm CPU/wall/RSS, allocations and malformed-input behavior. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.105.0 — API and profile stabilization

**Setup:** Start from completed 0.104.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** API and profile stabilization with explicit validation and resource contracts.

**Deliverables:** Review ergonomics, error stability, ownership, feature additivity and all four version axes for 1.0.

**Verification:** Independent consumer migrations; additive-feature matrix; documentation examples and semantic API snapshot. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.106.0 — Supply-chain and package rehearsal

**Setup:** Start from completed 0.105.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Supply-chain and package rehearsal with explicit validation and resource contracts.

**Deliverables:** Produce verified archives, first-party dependency graph/SBOM, provenance and checksums using isolated staging.

**Verification:** Rebuild from archives without sibling repos; install every package in dependency order; verify all pins and metadata. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.107.0 — Independent security review

**Setup:** Start from completed 0.106.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Independent security review with explicit validation and resource contracts.

**Deliverables:** Obtain named external review of parser completeness, native spans, Unicode, emission and release process.

**Verification:** Publish scope, evidence and findings; no automation substitute or unverified independence claim. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.108.0 — Remediation and regression release

**Setup:** Start from completed 0.107.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Remediation and regression release with explicit validation and resource contracts.

**Deliverables:** Resolve review/campaign findings with focused fixes, split further whenever fixes are not reviewable together.

**Verification:** Each issue has a reproducer, regression, exact fixed revision and reviewer retest; no open release blockers. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.109.0 — Stable documentation and support contract

**Setup:** Start from completed 0.108.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Stable documentation and support contract with explicit validation and resource contracts.

**Deliverables:** Finalize capability/grammar/Unicode/platform matrices, security response policy and stable API guarantees.

**Verification:** Every public claim links to current evidence; all examples compile; unsupported/opaque boundaries remain explicit. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

### 0.110.0 — Production candidate freeze

**Setup:** Start from completed 0.109.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Production candidate freeze with explicit validation and resource contracts.

**Deliverables:** Freeze the complete admitted scope and prepare exact 1.0.0-rc.N source/artifact candidates.

**Verification:** All gates, independent findings closure, package rehearsal, native acceptance and candidate pentest must be current. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## Production release

### 1.0.0-rc.N — exact candidate

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

### 1.0.0 — production admission

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
