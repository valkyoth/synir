# Expanded types, expressions and patterns

[Roadmap and mandatory gates](../RELEASE_PLAN.md). All versions are planned,
except the 0.1.0 foundation candidate under local verification.

## 0.71.0 — Complete type grammar inventory

**Setup:** Start from completed 0.70.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Complete type grammar inventory with explicit validation and resource contracts.

**Deliverables:** Close supported Rust type-production gaps and record opaque macro/unstable policies against pinned reference.

**Verification:** Matrix names every type form; unsupported syntax cannot be mislabeled as a validated type. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.72.0 — Type grammar completion

**Setup:** Start from completed 0.71.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Type grammar completion with explicit validation and resource contracts.

**Deliverables:** Implement remaining qualified, bare-function, impl-trait, trait-object and associated-constraint productions.

**Verification:** Production-specific positive/negative/compiler corpus; bounded speculation and nested generic regressions. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.73.0 — Expression atoms and paths

**Setup:** Start from completed 0.72.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Expression atoms and paths with explicit validation and resource contracts.

**Deliverables:** Recognize expression literals, paths, groups and explicit opaque macro invocations.

**Verification:** No semantic resolution claims; distinguish malformed paths from opaque macro bodies. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.74.0 — Postfix expressions

**Setup:** Start from completed 0.73.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Postfix expressions with explicit validation and resource contracts.

**Deliverables:** Add calls, method calls, field/index/await/try operators under explicit edition rules.

**Verification:** Chained syntax, turbofish and malformed argument separators; complete consumption and limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.75.0 — Unary binary and assignment expressions

**Setup:** Start from completed 0.74.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Unary binary and assignment expressions with explicit validation and resource contracts.

**Deliverables:** Implement precedence/associativity-aware operators, ranges, casts and assignment forms.

**Verification:** Operator precedence corpus, exact tree structure and context-specific restrictions; no angle counting guesses. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.76.0 — Block and control expressions

**Setup:** Start from completed 0.75.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Block and control expressions with explicit validation and resource contracts.

**Deliverables:** Add blocks, if/loop/while/for/break/continue/return forms with explicit grammar contexts.

**Verification:** Dangling else, labels, let conditions and nested control-flow boundaries under work/depth limits. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.77.0 — Closure async and special expressions

**Setup:** Start from completed 0.76.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Closure async and special expressions with explicit validation and resource contracts.

**Deliverables:** Add closures, async/const/unsafe blocks and remaining admitted special expression forms.

**Verification:** Edition-specific grammar, closure parameter/type ambiguity and bounded parser alternatives. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.78.0 — Pattern atoms and destructuring

**Setup:** Start from completed 0.77.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Pattern atoms and destructuring with explicit validation and resource contracts.

**Deliverables:** Implement bindings, wildcards, literals, paths, tuples, arrays and record patterns.

**Verification:** Rest position, reference/mut binding syntax and malformed destructuring; no duplicate omission. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.79.0 — Pattern alternatives and match

**Setup:** Start from completed 0.78.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Pattern alternatives and match with explicit validation and resource contracts.

**Deliverables:** Add ranges, or-patterns, guards and match arms integrating the expression recognizer.

**Verification:** Precedence, nested alternatives, guards and arm comma rules; contextual invalid patterns fail. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.

## 0.80.0 — Typed expression emission

**Setup:** Start from completed 0.79.0 baseline; scope owner: core; facade only for admitted re-exports.
Record exact APIs, excluded adjacent work, budgets, fixtures and command in the
release scope manifest before implementation.

**Goal:** Typed expression emission with explicit validation and resource contracts.

**Deliverables:** Emit validated expressions with conservative precedence/associativity grouping.

**Verification:** Compile precedence-sensitive generated programs; raw token interpolation makes no typed-expression promise. Run the mandatory gates and record the exact candidate.

**Exit criteria:** Deliverables and tests match the documented scope. Stop for an
exact-candidate **pentest**, resolve findings and obtain retest evidence before
release. Update notes/status; no tag or publication with missing evidence.
