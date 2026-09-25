# Threat model

## Assets and attackers

Protect compiler availability, complete interpretation of user declarations,
generated-code integrity, source/span provenance, reproducible builds and
private source contents. Treat source bytes, tokens, helper attributes, generated
names, quote repetitions, stale edit handles and downstream expansions as hostile.

| Boundary | Failure | Required defense and evidence |
| --- | --- | --- |
| Input/store | Foreign IDs, UTF-8 split ranges, stale revisions | Private checked IDs; ownership and revision tests |
| Parser | Lost fields, successful prefixes, unbounded speculation | Complete validation; progress checks; monotonic shared fuel |
| Storage | Overflow, exhaustion, recursion in clone/drop/debug | Checked arithmetic; fallible reservation; iterative traversal tests |
| Attributes | Typos activate flags, duplicate shadowing, type confusion | Strict owned schema; deterministic bounded diagnostics |
| Literals/Unicode | Incorrect escapes, numeric overflow, NFC confusion | Pinned grammar/data; conformance and exact-boundary tests |
| Native adapter | Lost hygiene, forged spans, None-group precedence | Original handles; real compiler/macro_rules fixtures |
| Emission | Source injection, token fusion, truncated impl, huge expansion | Typed data/code distinction; transactional sinks; output limits |
| Edits | Wrong source, overlaps, invalid UTF-8 cuts | Revision-bound plans; preflight before a single output pass |
| Diagnostics | Literal/path leaks and unbounded reporting | Redaction; control escaping; capped errors and work |
| CI/release | Dependency substitution, stale tools, unreviewed publication | Local graph allowlist; pinned actions; exact candidate review |

No_std is portability, not a sandbox. Safe implementation does not prove safe
emitted programs. The compiler, Rust libraries, OS, allocator and caller callbacks
remain trusted/external boundaries. Synir does not resolve names, evaluate cfg,
load modules, run macros, or type-check applications implicitly. Enforce process
time/memory limits and credential isolation for hostile compiler workloads.

At foundation stage these defenses are design requirements; only repository
policies and compilation boundaries are implemented. There is no parser attack
surface yet and no product pentest evidence. Revisit this model at each new
input boundary, profile, compiler/grammar update and before release.
