# Source frontend and Unicode

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.61.0 — Source contract and trivia

**Setup:** Start from completed 0.60.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source contract and trivia with explicit validation and resource contracts.

**Deliverables:** Pin grammar/edition sources; introduce borrowed UTF-8 source and lossless trivia ranges.

**Verification:** Invalid byte input is rejected explicitly; no-op traversal preserves bytes and source identity. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.62.0 — ASCII identifiers and punctuation

**Setup:** Start from completed 0.61.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** ASCII identifiers and punctuation with explicit validation and resource contracts.

**Deliverables:** Implement an explicitly restricted ASCII lexer with raw/keyword/edition rules and punctuation jointness.

**Verification:** Reject non-ASCII identifiers as unsupported; no complete-Rust lexer claim; reserved prefixes and lifetimes tested. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.63.0 — Comments and groups

**Setup:** Start from completed 0.62.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Comments and groups with explicit validation and resource contracts.

**Deliverables:** Lex nested block/line/doc comments and validated delimiters with iterative depth accounting.

**Verification:** Unclosed/deep comments, delimiters inside comments and trivia boundaries consume bounded work. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.64.0 — Source literal scanning

**Setup:** Start from completed 0.63.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source literal scanning with explicit validation and resource contracts.

**Deliverables:** Recognize strings/raw strings, chars, byte/C strings and numeric tokens into borrowed ranges.

**Verification:** Escapes, raw hashes, lifetime/char ambiguity, malformed exponent/suffix and oversized literals. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.65.0 — Unicode data generation

**Setup:** Start from completed 0.64.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unicode data generation with explicit validation and resource contracts.

**Deliverables:** Pin official XID and normalization data, checksums, licenses and an offline Rust generator; shard tables.

**Verification:** Regeneration byte equality, input checksum mismatch rejection and every generated file at most 500 lines. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.66.0 — Unicode identifier classification

**Setup:** Start from completed 0.65.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unicode identifier classification with explicit validation and resource contracts.

**Deliverables:** Implement XID_Start/XID_Continue with ASCII fast path and edition/raw identifier restrictions.

**Verification:** Exhaustive scalar classification against pinned data; invalid scalar boundaries and table edge vectors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.67.0 — NFC normalization

**Setup:** Start from completed 0.66.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** NFC normalization with explicit validation and resource contracts.

**Deliverables:** Implement bounded decomposition, canonical ordering and composition with caller storage.

**Verification:** Official normalization tests, Hangul and adversarial combining sequences; output/work exhaustion and idempotence. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.68.0 — Canonical identifier identity

**Setup:** Start from completed 0.67.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Canonical identifier identity with explicit validation and resource contracts.

**Deliverables:** Keep original spelling separate from normalized identity and compiler-normalized tokens.

**Verification:** Canonically equivalent source identifiers compare as defined; source bytes unchanged; native/source differences triaged. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.69.0 — Source/native conformance

**Setup:** Start from completed 0.68.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Source/native conformance with explicit validation and resource contracts.

**Deliverables:** Run the same supported declaration/schema grammar against source and native token frontends.

**Verification:** Compare normalized structure, never lost whitespace; grammar/Unicode/compiler version axes recorded. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.70.0 — Standalone and no-allocator acceptance

**Setup:** Start from completed 0.69.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Standalone and no-allocator acceptance with explicit validation and resource contracts.

**Deliverables:** Ship source inspection and caller-storage parser examples with optional fallible owned storage.

**Verification:** Bare-metal and mobile target checks; tiny/exact buffers; alloc reservation errors and no implicit file loading. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
