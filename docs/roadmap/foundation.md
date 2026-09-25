# Foundation and accounting

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.1.0 — Workspace and assurance setup

**Setup:** Start from initial repository and supplied idea; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Workspace and assurance setup with explicit validation and resource contracts.

**Deliverables:** Establish the four-crate DAG, licenses, separate READMEs, tool pins, CI, policies and complete roadmap.

**Verification:** Run local, compiler and target gates; review dependency rejection tests and package contents. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.2.0 — Contract inventory and errors

**Setup:** Start from completed 0.1.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Contract inventory and errors with explicit validation and resource contracts.

**Deliverables:** Define typed portable error codes and the inspect/validate/recover/opaque contract inventory; pin the ten adversarial fixture inputs.

**Verification:** Check deterministic error identity, no input disclosure, and each fixture owner; no parser success claims. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.3.0 — Explicit resource limits

**Setup:** Start from completed 0.2.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Explicit resource limits with explicit validation and resource contracts.

**Deliverables:** Define units and checked limits for bytes, tokens, depth, nodes, decoded data, storage, work, output and diagnostics.

**Verification:** Test zero/exact/one-over boundaries and usize/u64 overflow on 32-bit and 64-bit targets. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.4.0 — Monotonic work budget

**Setup:** Start from completed 0.3.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Monotonic work budget with explicit validation and resource contracts.

**Deliverables:** Implement one invocation fuel ledger shared by child operations; charge before action.

**Verification:** Exhaust fuel during nested operations; failed work cannot wrap, reset or regain credit. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.5.0 — Caller-owned storage

**Setup:** Start from completed 0.4.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Caller-owned storage with explicit validation and resource contracts.

**Deliverables:** Implement safe initialized-slot storage with fallible insertion and occupancy accounting.

**Verification:** Test empty/exact/full buffers, failed insertion, reuse and no allocator linkage. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.6.0 — Token identities and provenance

**Setup:** Start from completed 0.5.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Token identities and provenance with explicit validation and resource contracts.

**Deliverables:** Introduce private store-bound token/source identities and checked range constructors.

**Verification:** Reject foreign IDs, inverted ranges, overflow and stale source identities; compile-fail forging attempts. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.7.0 — Flat portable token tape

**Setup:** Start from completed 0.6.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Flat portable token tape with explicit validation and resource contracts.

**Deliverables:** Define token kinds, punctuation spacing and checked delimiter links in flat caller storage.

**Verification:** Reject unmatched/crossed groups and invalid indices; traverse deepest allowed tape iteratively. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.8.0 — Cursor and checkpoints

**Setup:** Start from completed 0.7.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Cursor and checkpoints with explicit validation and resource contracts.

**Deliverables:** Add bounded lookahead, checkpoints and explicit complete versus prefix consumption.

**Verification:** Rollback restores position but never fuel; trailing input and out-of-range seeks fail. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.9.0 — Progress-safe parser combinators

**Setup:** Start from completed 0.8.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Progress-safe parser combinators with explicit validation and resource contracts.

**Deliverables:** Implement alternatives, optional and separated repetition with explicit trailing-separator rules.

**Verification:** Zero-progress success fails repetition; adversarial failed alternatives exhaust the shared budget. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.10.0 — Deterministic assurance runner

**Setup:** Start from completed 0.9.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Deterministic assurance runner with explicit validation and resource contracts.

**Deliverables:** Add Rust/std compile-pass/fail runner, bounded corpus generation, process limits and replay metadata.

**Verification:** Exercise successful and failing fixtures, timeout and nonzero compiler exit; runners must fail on missing evidence. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
