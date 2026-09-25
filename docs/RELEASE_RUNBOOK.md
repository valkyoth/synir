# Release runbook

## Prepare the candidate

1. Select one [roadmap milestone](RELEASE_PLAN.md); record exact APIs, errors,
   exclusions, resource bounds, corpus entries and test commands in its scope
   manifest. Update the requirement coverage rows with actual evidence links.
2. Run `cargo xtask freshness`, local checks and applicable compiler/platform
   and assurance gates. Unavailable evidence does not count as a pass.
3. Update status, README, changelog and `release-notes/vX.Y.Z.md`.
4. Before publication, verify all package archives in dependency order in an
   isolated staging environment. Record graph/SBOM, compiler/profile/corpus and
   grammar/Unicode revisions, checksums and relevant resource/performance evidence.
   The foundation local gate verifies core; full-family archive rehearsal is
   still planned and must complete before the first family publication.

## One report committed with the candidate

Follow Brynja's reporting workflow. Use one permanent report at
`security/pentest/vX.Y.Z[-rc.N].md`, starting from the
[template](../security/pentest/TEMPLATE.md). Record Version, Status, Open-Findings,
Retest, Date, Tester and Scope, followed by methods, findings, remediation,
retest results and limitations. No separate commit-hash field is required.

Perform the candidate pentest, fix findings and retest. Commit the final report
with the candidate. If CI requires a subsequent change, review/retest it and
commit that change with the corresponding report update, then rerun CI. Do not
invent a passing assessment or describe automated checks as independent review.
A report is evidence from its author; the gate validates its state and Git
relationship, not the quality or truth of that assessment.

For a non-release status check:

```sh
scripts/release/validate-current-pentest.sh
```

Missing evidence leaves release blocked. A committed report may say
`Status: RETEST REQUIRED`, `Open-Findings: 0`, `Retest: PENDING` while awaiting
retest; this never passes the required release gate. Keep ordinary implementation
CI free of report freshness requirements; it runs the gate's regression tests.

## Release and tag check

After the final report is committed, require:

```sh
scripts/release/validate-current-pentest.sh --required
```

The gate selects the current Cargo version and requires PASS/PASS with zero open
findings, a regular committed report, a clean checkout, a current candidate/report
update and an absent release tag. The direct equivalent for a named candidate is
`scripts/release/validate-release-readiness.sh vX.Y.Z`.

The release workflow fetches full history and runs this required check. GitHub
CodeQL uses Default setup only. The user confirms that CI and CodeQL are green
and authorizes tagging. Then create a signed annotated tag with subject
`Synir vX.Y.Z` (lowercase `synir` is also accepted). Verify before publication:

```sh
SYNIR_RELEASE_PUBLISH_TAG=vX.Y.Z scripts/release/validate-release-readiness.sh vX.Y.Z
```

This context checks the signature, subject and direct HEAD commit target of the
existing tag. The gate itself never creates/pushes tags or publishes packages.
Publish reviewed packages in order: core, host, macros, facade; retain notes,
checksums and provenance, and verify installed registry contents. Keep publishing
credentials out of untrusted pull-request jobs. Local development commits remain
allowed without a pentest; release tags and publication require the passing gate.

Synir keeps pentests for every release version, including patches and RCs. The
adaptation does not add Brynja's checkpoint cadence or exceptional deferral flow.
1.0 also requires complete capability evidence, independent review/remediation,
platform runtime acceptance and the admitted production candidate.
