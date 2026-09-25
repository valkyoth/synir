# Declaration recognition and generic projections

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.21.0 — Item prefixes and visibility

**Setup:** Start from completed 0.20.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Item prefixes and visibility with explicit validation and resource contracts.

**Deliverables:** Recognize ordered outer attributes, visibility and item introducers; never scan past unexpected prefixes.

**Verification:** Reject skipped leading syntax; test pub, restricted visibility, raw names and preserved unrelated attributes. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.22.0 — Path and generic-argument boundaries

**Setup:** Start from completed 0.21.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Path and generic-argument boundaries with explicit validation and resource contracts.

**Deliverables:** Recognize qualified paths, nested arguments and associated bindings without comma scanning shortcuts.

**Verification:** Result<T,E>, qualified paths, turbofish and joint punctuation have exact ranges or explicit unsupported errors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.23.0 — Reference pointer and tuple type boundaries

**Setup:** Start from completed 0.22.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Reference pointer and tuple type boundaries with explicit validation and resource contracts.

**Deliverables:** Recognize references, raw pointers, tuple/group types and slices over token views.

**Verification:** Nested commas/lifetimes, invalid mutability and malformed delimiters have precise complete-consumption errors. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.24.0 — Function and bounded type regions

**Setup:** Start from completed 0.23.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Function and bounded type regions with explicit validation and resource contracts.

**Deliverables:** Recognize function pointers, higher-ranked binders, trait-object bounds and array const regions with explicit opacity.

**Verification:** Fn(T)->Result<T,E> arrows never close generic lists; unsupported const internals cannot become validated expressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.25.0 — Named structs

**Setup:** Start from completed 0.24.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Named structs with explicit validation and resource contracts.

**Deliverables:** Parse every named field and attached attributes using grammar-aware type boundaries.

**Verification:** Nested type commas, missing colons, duplicates as syntax policy, malformed last field and early iterator stop. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.26.0 — Tuple and unit structs

**Setup:** Start from completed 0.25.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Tuple and unit structs with explicit validation and resource contracts.

**Deliverables:** Add tuple/unit forms and their item-specific terminators/where positions.

**Verification:** Tuple where clauses before semicolon, empty tuples, forbidden separators and trailing junk. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.27.0 — Enum variants

**Setup:** Start from completed 0.26.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Enum variants with explicit validation and resource contracts.

**Deliverables:** Recognize unit/tuple/record variants and explicit discriminant regions with bounded shape validation.

**Verification:** Every variant/field counted; malformed final variant rejected even if iteration stops early. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.28.0 — Union shapes

**Setup:** Start from completed 0.27.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Union shapes with explicit validation and resource contracts.

**Deliverables:** Parse union declarations with an explicit caller derive policy and no implicit field access.

**Verification:** Reject invalid union shapes; acceptance example policy rejects unsupported derive operations clearly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.29.0 — Generic parameter and where models

**Setup:** Start from completed 0.28.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generic parameter and where models with explicit validation and resource contracts.

**Deliverables:** Parse lifetime/type/const parameters, bounds, defaults and grammar-aware where predicates.

**Verification:** Where without generics, function trait bounds, raw parameters, empty lists, const defaults and tuple placement. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.30.0 — Validated impl projections

**Setup:** Start from completed 0.29.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Validated impl projections with explicit validation and resource contracts.

**Deliverables:** Produce declaration/impl/type-argument projections and explicit bound-policy plans from complete shapes.

**Verification:** Defaults removed only for impl parameters; predicates/spans/order preserved; generated minimal impls compile. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
