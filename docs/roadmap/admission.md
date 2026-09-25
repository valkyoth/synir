# Security, performance and production admission

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.101.0 — Hostile-input campaign

**Setup:** Start from completed 0.100.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Hostile-input campaign with explicit validation and resource contracts.

**Deliverables:** Expand deterministic mutation/generation across tokens, grammar, literals, schemas, quoting and edits.

**Verification:** Run with external CPU/memory/time limits; replay seeds and record worst-case work/storage growth. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.102.0 — Independent differential campaign

**Setup:** Start from completed 0.101.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Independent differential campaign with explicit validation and resource contracts.

**Deliverables:** Compare admitted syntax and emitted behavior against pinned independent compiler/parser tools.

**Verification:** Triage every disagreement; semantic errors separated from syntax; save reproducible regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.103.0 — Platform runtime acceptance

**Setup:** Start from completed 0.102.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Platform runtime acceptance with explicit validation and resource contracts.

**Deliverables:** Run real consumers on Linux/Windows/macOS/FreeBSD and Android/iOS device or emulator/simulator setups.

**Verification:** Record exact hosts/targets/SDKs and results; cross compile alone cannot satisfy a runtime claim; Aesynx remains future. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.104.0 — Performance and footprint review

**Setup:** Start from completed 0.103.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Performance and footprint review with explicit validation and resource contracts.

**Deliverables:** Measure complete equivalent workloads with minimal/manual/helper/full profiles, including native import.

**Verification:** Publish hardware/cache/tool versions, cold/warm CPU/wall/RSS, allocations and malformed-input behavior. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.105.0 — API and profile stabilization

**Setup:** Start from completed 0.104.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** API and profile stabilization with explicit validation and resource contracts.

**Deliverables:** Review ergonomics, error stability, ownership, feature additivity and all four version axes for 1.0.

**Verification:** Independent consumer migrations; additive-feature matrix; documentation examples and semantic API snapshot. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.106.0 — Supply-chain and package rehearsal

**Setup:** Start from completed 0.105.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Supply-chain and package rehearsal with explicit validation and resource contracts.

**Deliverables:** Produce verified archives, first-party dependency graph/SBOM, provenance and checksums using isolated staging.

**Verification:** Rebuild from archives without sibling repos; install every package in dependency order; verify all pins and metadata. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.107.0 — Independent security review

**Setup:** Start from completed 0.106.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Independent security review with explicit validation and resource contracts.

**Deliverables:** Obtain named external review of parser completeness, native spans, Unicode, emission and release process.

**Verification:** Publish scope, evidence and findings; no automation substitute or unverified independence claim. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.108.0 — Remediation and regression release

**Setup:** Start from completed 0.107.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Remediation and regression release with explicit validation and resource contracts.

**Deliverables:** Resolve review/campaign findings with focused fixes, split further whenever fixes are not reviewable together.

**Verification:** Each issue has a reproducer, regression, exact fixed revision and reviewer retest; no open release blockers. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.109.0 — Stable documentation and support contract

**Setup:** Start from completed 0.108.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Stable documentation and support contract with explicit validation and resource contracts.

**Deliverables:** Finalize capability/grammar/Unicode/platform matrices, security response policy and stable API guarantees.

**Verification:** Every public claim links to current evidence; all examples compile; unsupported/opaque boundaries remain explicit. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.110.0 — Production candidate freeze

**Setup:** Start from completed 0.109.0 baseline; scope owner: repository tools, documentation and affected crate owners.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Production candidate freeze with explicit validation and resource contracts.

**Deliverables:** Freeze the complete admitted scope and prepare exact 1.0.0-rc.N source/artifact candidates.

**Verification:** All gates, independent findings closure, package rehearsal, native acceptance and candidate pentest must be current. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
