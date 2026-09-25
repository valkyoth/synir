# Setup reference provenance

Reviewed 2026-09-25 from local repositories under
`/home/eldryoth/Work/codex-projects/`. These paths are development references,
not build dependencies. Synir builds without sibling repositories.

| Reference | Adopted practice |
| --- | --- |
| brynja | MSRV/full-check table; small facade; crate-facing README; security gates; CODEOWNERS/FUNDING copied |
| eth | .gitignore, licenses, deny and GitHub templates copied/adapted; centered README header; granular Goal/Deliverables/Verification/Exit roadmap |
| sanitization | Separate platform boundaries and exhaustive feature/negative-test matrix; requested sanitization-rust-crate directory was absent |
| aesynx | Exact candidate pentest reports, release notes and future platform separation |
| skrifheim | Separate implementation/version plans and reviewable crate ownership |
| fluxheim | Security/supply-chain evidence, CodeQL Default-only configuration and release checks |

Eth-specific protocol jobs and Brynja SHA/standards jobs are not applicable to
Synir; their verification principles are adapted instead of committing broken
workflow references. All general GitHub files are retained/adapted: contribution
and PR templates, issue form, owners, funding, dependency maintenance, CI and
release metadata validation. The pre-existing Synir image is preserved.

License prose in the references was inconsistent (including obsolete EUPL text);
Synir's package metadata and contribution policy consistently state MIT OR
Apache-2.0. Upstream template notices are retained where applicable.
The idea's A: source anchors refer to an earlier attachment not present here;
its historical review statements are not new Synir verification evidence.
