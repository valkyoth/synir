# Fallible emission and usable macro profile

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.41.0 — Transactional token sink

**Setup:** Start from completed 0.40.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Transactional token sink with explicit validation and resource contracts.

**Deliverables:** Implement explicit start/commit/discard output with token/byte limits and reserved error capacity.

**Verification:** Any mid-output failure returns error without publishing a truncated successful stream. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.42.0 — Checked token builders

**Setup:** Start from completed 0.41.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Checked token builders with explicit validation and resource contracts.

**Deliverables:** Construct identifiers, supported punctuation, groups and typed literals; reject untrusted invalid names.

**Verification:** Invalid Ident input is rejected before compiler constructors; strings emit data, not executable text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.43.0 — Native export

**Setup:** Start from completed 0.42.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native export with explicit validation and resource contracts.

**Deliverables:** Export portable constructions with explicit span policy while forwarding original native objects unchanged.

**Verification:** Group/spacing/span fixtures and output exhaustion through real proc macros; document compiler allocation boundary. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.44.0 — Generic impl emitter

**Setup:** Start from completed 0.43.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Generic impl emitter with explicit validation and resource contracts.

**Deliverables:** Emit validated impl plans with projected generics, original predicates and explicit trait/dependency paths.

**Verification:** Compile lifetime/default/const/where examples; no incidental Clone/Debug bounds; renamed runtime paths work. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.45.0 — Describe structs acceptance

**Setup:** Start from completed 0.44.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Describe structs acceptance with explicit validation and resource contracts.

**Deliverables:** Implement a real metadata derive for named, tuple and unit structs using the manual pipeline.

**Verification:** Compile and execute metadata tests; no_std target contains only intended trait/data, no parser runtime. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.46.0 — Describe enum and union policy

**Setup:** Start from completed 0.45.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Describe enum and union policy with explicit validation and resource contracts.

**Deliverables:** Extend Describe to enums and document explicit union behavior and field options.

**Verification:** Wrong attributes and unsupported union behavior emit deliberate errors; no omitted variants/fields. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.47.0 — Declaration-preserving attribute macro

**Setup:** Start from completed 0.46.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Declaration-preserving attribute macro with explicit validation and resource contracts.

**Deliverables:** Provide a compiled function attribute example retaining opaque bodies and unrelated attributes.

**Verification:** Body/provenance unchanged; malformed owned options and unsupported signatures fail without body truncation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.48.0 — Error derive and custom DSL examples

**Setup:** Start from completed 0.47.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Error derive and custom DSL examples with explicit validation and resource contracts.

**Deliverables:** Add an independent error-type derive workflow and a small custom language acceptance fixture.

**Verification:** Verify distinct generic-bound policies, deliberate failures and progress-safe custom parsing. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.49.0 — Macro-development profile

**Setup:** Start from completed 0.48.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro-development profile with explicit validation and resource contracts.

**Deliverables:** Expose a documented additive macro-dev preset for implemented derive/attribute/manual emission capabilities.

**Verification:** Feature-isolated builds, renamed facade/runtime dependencies, namespace shadowing and no_std consumers. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.50.0 — Macro workflow acceptance

**Setup:** Start from completed 0.49.0 baseline; scope owner: core emission, host export and facade examples.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Macro workflow acceptance with explicit validation and resource contracts.

**Deliverables:** Measure and review end-to-end native import to validated plan to output on several real use cases.

**Verification:** Publish boundedness and cold/warm benchmark methodology; run independent compiler fixtures and pentest. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
