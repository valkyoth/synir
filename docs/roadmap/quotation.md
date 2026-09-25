# Quotation and generated schemas

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.51.0 — Quotation language specification

**Setup:** Start from completed 0.50.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Quotation language specification with explicit validation and resource contracts.

**Deliverables:** Specify a clean-slate template grammar, marker escaping and builder lowering independent of Quote API compatibility.

**Verification:** Ambiguous templates, literal markers and unsupported constructs have explicit diagnostics and bounded parsing. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.52.0 — Scalar interpolation

**Setup:** Start from completed 0.51.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Scalar interpolation with explicit validation and resource contracts.

**Deliverables:** Lower scalar token/literal/view interpolation into fallible manual builder calls.

**Verification:** Data/code distinction, nested groups and runtime errors preserve transactional output. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.53.0 — Renamed-crate hygiene

**Setup:** Start from completed 0.52.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Renamed-crate hygiene with explicit validation and resource contracts.

**Deliverables:** Implement tested framework-path plumbing and explicit span selection for generated builder code.

**Verification:** Rename Synir, shadow common names and combine multiple dependency aliases in real compilation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.54.0 — Simple repetition

**Setup:** Start from completed 0.53.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Simple repetition with explicit validation and resource contracts.

**Deliverables:** Implement bounded zero-or-more/nonempty/optional repetitions and separators.

**Verification:** Empty, one, many, missing item and trailing-separator cases; work/output counters cover every iteration. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.55.0 — Nested and multiple repetition

**Setup:** Start from completed 0.54.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Nested and multiple repetition with explicit validation and resource contracts.

**Deliverables:** Define lexical repetition scopes and exact finite sequence length matching.

**Verification:** Mismatched lengths error, never zip-truncate; deeply nested output growth exhausts shared limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.56.0 — Punctuation and lifetime quotation

**Setup:** Start from completed 0.55.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Punctuation and lifetime quotation with explicit validation and resource contracts.

**Deliverables:** Preserve jointness, lifetimes, negative literals and delimiter semantics through templates.

**Verification:** Native compilation distinguishes operator token fusion and invisible-group precedence regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.57.0 — Quotation resource acceptance

**Setup:** Start from completed 0.56.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Quotation resource acceptance with explicit validation and resource contracts.

**Deliverables:** Bound template compilation as well as generated execution and eliminate recursive expansion blowups.

**Verification:** Large templates, escaped markers and pathological repetition have measured bounded failures. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.58.0 — Generated schema decoder core

**Setup:** Start from completed 0.57.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generated schema decoder core with explicit validation and resource contracts.

**Deliverables:** Generate the same strict scalar decoder semantics as handwritten schemas without self-bootstrap dependencies.

**Verification:** Compare handwritten/generated behavior for valid and malformed options; helper crate never depends on facade. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.59.0 — Generated nested schema support

**Setup:** Start from completed 0.58.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generated nested schema support with explicit validation and resource contracts.

**Deliverables:** Extend schema helpers to contexts, nested/repeated values, forwarding and custom validation hooks.

**Verification:** Parity for duplicate/unknown/conflict diagnostics and locations; code-size and budget regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.60.0 — Helper workflow acceptance

**Setup:** Start from completed 0.59.0 baseline; scope owner: macros and core; host integration.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Helper workflow acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate quotation and generated schemas as optional conveniences over the working manual path.

**Verification:** Run all helper feature combinations and real fixtures; publish manual-versus-helper compile cost. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
