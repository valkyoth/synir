# Literal decoding and typed attributes

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.31.0 — Literal classification

**Setup:** Start from completed 0.30.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Literal classification with explicit validation and resource contracts.

**Deliverables:** Distinguish all admitted Rust literal token forms and preserve original spellings and suffixes.

**Verification:** Reject wrong literal kind and malformed prefix/suffix; document unsupported forms explicitly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.32.0 — String decoding

**Setup:** Start from completed 0.31.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** String decoding with explicit validation and resource contracts.

**Deliverables:** Decode escaped and raw strings into caller buffers with decoded-byte/work accounting.

**Verification:** Escape, continuation, raw delimiter and UTF-8 boundary vectors; exact output capacity and one-over failure. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.33.0 — Byte and character decoding

**Setup:** Start from completed 0.32.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Byte and character decoding with explicit validation and resource contracts.

**Deliverables:** Decode byte strings/bytes/chars with form-specific Unicode and length checks.

**Verification:** Reject multi-scalar char, out-of-range byte and invalid escapes; no silent fallback to text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.34.0 — Numeric and C-string decoding

**Setup:** Start from completed 0.33.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Numeric and C-string decoding with explicit validation and resource contracts.

**Deliverables:** Add checked integer schema conversion and C-string validation; preserve floats until explicit conversion policy.

**Verification:** Overflow, signs, bases, suffixes and interior NUL vectors; unsupported float conversion fails explicitly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.35.0 — Attribute syntax

**Setup:** Start from completed 0.34.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Attribute syntax with explicit validation and resource contracts.

**Deliverables:** Parse paths, flags, assignments, nested lists and explicit opaque payloads separately from schema policy.

**Verification:** Unexpected values never become flags; trailing tokens and nesting exhaustion fail; preserve unrelated attributes. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.36.0 — Strict scalar schemas

**Setup:** Start from completed 0.35.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Strict scalar schemas with explicit validation and resource contracts.

**Deliverables:** Add handwritten schemas for booleans, strings, checked integers, paths and choices.

**Verification:** Wrong type, unknown key, misspelling, missing required value and malformed optional values all error. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.37.0 — Duplicate and conflict policies

**Setup:** Start from completed 0.36.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Duplicate and conflict policies with explicit validation and resource contracts.

**Deliverables:** Enforce singleton/repeatable cardinality, defaults and mutually exclusive keys.

**Verification:** No last-write-wins accident; explicit repeat ordering; duplicate/conflict diagnostics retain both locations. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.38.0 — Nested context schemas

**Setup:** Start from completed 0.37.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Nested context schemas with explicit validation and resource contracts.

**Deliverables:** Support bounded nested records and separate container/variant/field/generic contexts.

**Verification:** Misplaced keys, deep records and too many repeated values fail with bounded deterministic output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.39.0 — Custom validation and diagnostics

**Setup:** Start from completed 0.38.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Custom validation and diagnostics with explicit validation and resource contracts.

**Deliverables:** Add caller validators with documented external-work boundary and bounded independent error aggregation.

**Verification:** Callbacks cannot forge validated state; source-order diagnostics, truncation and error-budget exhaustion. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.40.0 — Schema acceptance

**Setup:** Start from completed 0.39.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Schema acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate literal decoding, strict schemas and derive shapes with validated attribute values.

**Verification:** Run hostile configuration corpus and context matrix; no malformed attribute activates behavior. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
