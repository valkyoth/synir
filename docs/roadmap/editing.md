# Owned syntax and lossless edits

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.91.0 — Owned arena storage

**Setup:** Start from completed 0.90.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned arena storage with explicit validation and resource contracts.

**Deliverables:** Add flat owned syntax arenas with fallible reservation and private source/node identities.

**Verification:** Allocation-size overflow, exhaustion, failed insertion and cross-arena IDs; no recursive destruction. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.92.0 — Owned view conversion

**Setup:** Start from completed 0.91.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned view conversion with explicit validation and resource contracts.

**Deliverables:** Convert validated borrowed views into owned nodes under shared limits without losing opacity/provenance.

**Verification:** Round-trip normalized structure and span origin; stale native handles cannot be serialized. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.93.0 — Deep operation safety

**Setup:** Start from completed 0.92.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Deep operation safety with explicit validation and resource contracts.

**Deliverables:** Audit clone/drop/debug/equality/hash of all owned token and syntax structures.

**Verification:** Adversarial depth on small-stack worker processes; every operation has bounded traversal/output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.94.0 — Revision-bound edit plans

**Setup:** Start from completed 0.93.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Revision-bound edit plans with explicit validation and resource contracts.

**Deliverables:** Define source identity/revision, UTF-8 ranges and deterministic insertion order.

**Verification:** Reject wrong source/revision even at matching length; invalid UTF-8 boundary, inverted and overflow ranges. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.95.0 — Edit conflict validation

**Setup:** Start from completed 0.94.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Edit conflict validation with explicit validation and resource contracts.

**Deliverables:** Check overlap, duplicate replacement and equal-offset insertion policies before commit.

**Verification:** Permutation/conflict corpus and failure atomicity; no container-order-dependent winners. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.96.0 — Lossless edit application

**Setup:** Start from completed 0.95.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Lossless edit application with explicit validation and resource contracts.

**Deliverables:** Apply validated edits in one pass to caller sinks or budgeted owned output.

**Verification:** No-op byte identity, untouched comments/trivia, exact output limit and no partial successful artifact. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.97.0 — Explicit configuration and file boundaries

**Setup:** Start from completed 0.96.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Explicit configuration and file boundaries with explicit validation and resource contracts.

**Deliverables:** Design application-supplied cfg/source loading contexts without implicit environment/filesystem access.

**Verification:** Unresolved cfg preserved; denied loads and hostile paths remain application errors, not hidden parser I/O. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.98.0 — Owned transformation acceptance

**Setup:** Start from completed 0.97.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Owned transformation acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate full traversal, owned plans and lossless edits in standalone source tools.

**Verification:** Compiler-valid transformed fixtures, error propagation, caller-storage and alloc variants. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.99.0 — Workflow migration guides

**Setup:** Start from completed 0.98.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Workflow migration guides with explicit validation and resource contracts.

**Deliverables:** Publish task-oriented guides for derive, schema, quote, custom DSL, source analysis and transformations.

**Verification:** Compile every example with dependency renames/no_std where relevant; no API compatibility promises. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.100.0 — Capability replacement acceptance

**Setup:** Start from completed 0.99.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Capability replacement acceptance with explicit validation and resource contracts.

**Deliverables:** Reconcile all idea sections and practical replacement workflows with real downstream acceptance projects.

**Verification:** No descriptor/mock-only claims; parser, builder, helper and full/edit profiles each have independent consumers. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
