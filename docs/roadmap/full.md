# Full items and traversal

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.81.0 — Statements and local bindings

**Setup:** Start from completed 0.80.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Statements and local bindings with explicit validation and resource contracts.

**Deliverables:** Implement let/let-else, expression/item statements and semicolon distinctions.

**Verification:** Diverging else syntax and tail-expression boundaries; malformed local patterns/initializers rejected. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.82.0 — Function and signature grammar

**Setup:** Start from completed 0.81.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Function and signature grammar with explicit validation and resource contracts.

**Deliverables:** Complete qualifiers, ABI, receivers, variadics and signatures against the supported grammar.

**Verification:** Illegal qualifier sequences and receiver contexts; preserve spans and exact body boundaries. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.83.0 — Trait and impl items

**Setup:** Start from completed 0.82.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Trait and impl items with explicit validation and resource contracts.

**Deliverables:** Recognize trait/impl associated items, bounds and context-specific member syntax.

**Verification:** Invalid member contexts, where placement, negative/unsafe impl grammar and default items. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.84.0 — Modules imports and foreign items

**Setup:** Start from completed 0.83.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Modules imports and foreign items with explicit validation and resource contracts.

**Deliverables:** Implement module/use/extern productions without loading referenced files.

**Verification:** Nested use trees, glob/rename cases, inline versus external modules; no filesystem side effects. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.85.0 — Aliases constants statics and remaining items

**Setup:** Start from completed 0.84.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Aliases constants statics and remaining items with explicit validation and resource contracts.

**Deliverables:** Close the admitted item inventory including alias/type contexts, const/static and remaining stable forms.

**Verification:** Every remaining reference production gets a named fixture; unsupported unstable forms are explicit. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.86.0 — Macro syntax representation

**Setup:** Start from completed 0.85.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro syntax representation with explicit validation and resource contracts.

**Deliverables:** Represent macro_rules definitions and invocations with validated outer structure and explicit opaque DSL bodies.

**Verification:** Metavariables/repetition delimiters preserved; no expansion or fabricated validation of arbitrary DSL contents. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.87.0 — Full grammar acceptance

**Setup:** Start from completed 0.86.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Full grammar acceptance with explicit validation and resource contracts.

**Deliverables:** Reconcile edition/production matrix, grammar revision and native/source support for all admitted stable Rust syntax.

**Verification:** Pinned compiler/independent-parser corpus triage; no missing production hidden in a percentage claim. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.88.0 — Borrowed visits

**Setup:** Start from completed 0.87.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Borrowed visits with explicit validation and resource contracts.

**Deliverables:** Add typed iterative traversal with explicit limits over validated syntax views and opaque regions.

**Verification:** Every node visited as specified, cancellation/exhaustion bounded, deep traversal no stack overflow. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.89.0 — Structural mapping and folds

**Setup:** Start from completed 0.88.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Structural mapping and folds with explicit validation and resource contracts.

**Deliverables:** Add checked transformation plans with validation before mutation and provenance-aware replacements.

**Verification:** Failure atomicity, foreign ranges, generated syntax validity and unchanged input on rejected edits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.90.0 — Full-profile acceptance

**Setup:** Start from completed 0.89.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Full-profile acceptance with explicit validation and resource contracts.

**Deliverables:** Expose full grammar as an additive optional profile and publish source-analysis/transformation workflows.

**Verification:** Minimal macro profile cost unchanged; full profile production matrix and independent consumer fixtures pass. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
