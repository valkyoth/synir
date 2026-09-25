# Security policy

Synir is pre-release syntax infrastructure. No release is currently admitted for
production. Security includes build availability, complete parsing, provenance,
and generated-code correctness, as well as memory safety.

## Report privately

Use [GitHub private vulnerability reporting](https://github.com/valkyoth/synir/security/advisories/new).
If unavailable, contact [@eldryoth](https://github.com/eldryoth) to arrange a
private channel; do not post exploit details in public issues. Maintainers must
enable private reporting before the first publication. No response-time SLA is
claimed. Reports should include affected revision, reproducer, resource limits,
expected behavior, impact, and a suggested disclosure timeline.

## Supported releases

| Line | Status |
| --- | --- |
| Unreleased 0.1.0 | Foundation only; fixes on the development branch |
| Future pre-1.0 releases | Latest published checkpoint only, unless explicitly extended |
| 1.0 | Production support policy must be published before admission |

Triage parser omission, schema failure-open behavior, budget bypass, provenance
confusion and code injection as security-relevant. Never downgrade them merely
because Rust prevents memory corruption. Add a regression test for every fix;
publish affected scope and migration impact without disclosing an unfixed exploit.

## Required controls

- No third-party Cargo dependencies, unsafe implementation code or build scripts.
- No implicit I/O, environment evaluation, macro execution, or file loading in core.
- Strict errors, complete-consumption APIs and one monotonic invocation budget.
- Bounded diagnostic output with redaction; transactional high-level emission.
- Pinned tools and action commits; weekly freshness and pre-release review.
- Every release version requires a candidate pentest, remediation and retest.
  Commit its report with the final candidate and update it with later CI fixes;
  the Brynja-style release gate requires PASS/PASS with zero open findings.
- Local tests, CI, CodeQL and a maintainer review are distinct evidence sources.

Run `scripts/checks.sh`, compiler/platform matrices, `cargo deny check`,
`cargo audit --deny warnings`, and `cargo xtask freshness` before release.
Use GitHub **CodeQL Default setup** only. Do not add an advanced CodeQL workflow.
Default setup, private reporting and branch protections are repository settings;
files alone do not establish that they are enabled.

See [threat model](docs/threat-model.md), [verification](docs/VERIFICATION.md),
[pentest process](security/pentest/README.md), and [release runbook](docs/RELEASE_RUNBOOK.md).
