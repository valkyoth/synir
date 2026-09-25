# Native tokens and portable views

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.11.0 — Host invocation ownership

**Setup:** Start from completed 0.10.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Host invocation ownership with explicit validation and resource contracts.

**Deliverables:** Define invocation-local handle storage with explicit compiler-thread and lifetime restrictions.

**Verification:** Compile-fail escaped/foreign handles; test contracts through real compiler fixtures. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.12.0 — Native leaf import

**Setup:** Start from completed 0.11.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native leaf import with explicit validation and resource contracts.

**Deliverables:** Import identifiers, literals and punctuation preserving original handles, spans and spacing.

**Verification:** Macro fixtures compare joint operators, lifetimes, negative literals and raw identifiers; charge spelling extraction. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.13.0 — Native group import

**Setup:** Start from completed 0.12.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native group import with explicit validation and resource contracts.

**Deliverables:** Traverse native groups into the flat tape with explicit depth/token/storage accounting.

**Verification:** Deep and wide groups exhaust each limit cleanly; no recursive clone/drop path introduced. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.14.0 — Native spelling cache

**Setup:** Start from completed 0.13.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native spelling cache with explicit validation and resource contracts.

**Deliverables:** Cache per-token spelling under byte/work/storage quotas without stringify-and-reparse.

**Verification:** Repeated cache hits/misses preserve identity and charge correctly; long spellings fail boundedly. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.15.0 — Native span policies

**Setup:** Start from completed 0.14.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native span policies with explicit validation and resource contracts.

**Deliverables:** Separate source offsets from invocation spans, opening/closing spans and resolution policy.

**Verification:** Real diagnostic locations and hygiene fixtures; no behavior depends on optional span source_text. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.16.0 — Opaque forwarding

**Setup:** Start from completed 0.15.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Opaque forwarding with explicit validation and resource contracts.

**Deliverables:** Preserve original groups and token provenance with an explicit reason for opacity.

**Verification:** Compile macro_rules forwarding with Delimiter::None; compare precedence and original-token identity. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.17.0 — Bounded diagnostic rendering

**Setup:** Start from completed 0.16.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Bounded diagnostic rendering with explicit validation and resource contracts.

**Deliverables:** Render portable errors through caller sinks and native compile_error tokens; cap and redact data.

**Verification:** Overflow during error reporting still yields bounded failure; escape control characters and suppress literals/paths. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.18.0 — Inspection views

**Setup:** Start from completed 0.17.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Inspection views with explicit validation and resource contracts.

**Deliverables:** Expose header-only views distinct from validated shapes and opaque/recovered regions.

**Verification:** Malformed tails never acquire validated types; consuming a header cannot authorize high-level generation. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.19.0 — Borrowing ergonomics

**Setup:** Start from completed 0.18.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Borrowing ergonomics with explicit validation and resource contracts.

**Deliverables:** Separate immutable input from mutable work/output and prototype nested user parser contexts.

**Verification:** Compile realistic borrowed views through decoding/emission contexts; callbacks cannot reset internal counters. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.20.0 — Native foundation acceptance

**Setup:** Start from completed 0.19.0 baseline; scope owner: host and core; real compiler fixtures.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Native foundation acceptance with explicit validation and resource contracts.

**Deliverables:** Integrate import, views, diagnostic failures and unchanged forwarding in real consumer fixtures.

**Verification:** Run MSRV/current stable, no_std target consumers, host OS tests and hostile group corpus. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
