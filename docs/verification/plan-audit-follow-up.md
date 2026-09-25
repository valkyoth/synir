# Plan audit follow-up

Date: 2026-09-25. Scope: foundation tooling and planning changes for the six
audit findings; this is not a pentest report or syntax implementation evidence.

| Finding | Change | Evidence / remaining work |
| --- | --- | --- |
| Release/pentest enforcement | Adapted Brynja's one committed report process, required release check and signed-tag validation. | Disposable Git regression suite executes now; a real candidate pentest is still required. |
| Requirement traceability | Added 26 requirement rows, exact milestone owners and acceptance evidence; named source-text emission and bounded recovery owners. | Roadmap policy rejects missing/duplicate rows, empty evidence and nonexistent owners. Capability evidence remains planned. |
| Grammar baseline too late | Added language/edition contract at 0.3.0, before declaration parsing at 0.25.0. | Reference/compiler revisions and supported productions must be pinned during that pass. |
| Adversarial assurance too late | Split early compiler fixtures, deterministic generation, external-oracle protocol and continuous execution into 0.11.0–0.14.0. | Runnable foundation tests exist; parser/native runners remain planned and must precede capability admission. |
| Compiler checks insufficient | Workspace tests now run in both feature configurations on all 15 advertised compilers; native behavior fixtures are mandatory on admission. | Full current compiler matrix passed. Real proc-macro consumers remain planned. |
| Oversized implementation passes | Split normalization and grouped grammar work; retain one roadmap with 137 passes and an old-to-new version map. | Roadmap policy checks ordered versions and required fields; all 110 original owners map to the 137 replacement passes. |

## Executed verification

- `scripts/checks.sh`: passed format, repository policy, release-gate regression
  fixtures, strict Clippy, minimal/all-feature tests, feature wiring, rustdoc and
  core package verification. The Rust tooling suite contains 12 passing tests.
- `scripts/check-rust-version-matrix.sh`: passed workspace checks and tests with
  minimal and all features on every listed compiler from 1.90.0 through 1.98.1.
- Bash syntax validation passed for all three new release scripts.
- The current-report status command correctly reports missing evidence, and
  `scripts/release/validate-current-pentest.sh --required` rejects it.

Release fixtures cover missing/uncommitted/stale/dirty reports, required fields,
failed/pending assessments, open findings, report file types, candidate/report
updates, signed and unsigned tags, tag subjects and commit targets. These test
gate behavior; they do not establish the truth of an assessment supplied to it.

No dependency or library algorithm was added. Native fixtures, grammar testing,
adversarial runners and independent review remain explicit future deliverables.
Platform checks from foundation setup were not rerun for this tooling/docs change.
