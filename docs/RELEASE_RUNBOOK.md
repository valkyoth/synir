# Release runbook

## Prepare a reviewable candidate

1. Select one milestone from [the release plan](RELEASE_PLAN.md); record exact
   APIs, inputs, outputs, errors, excluded work, resource bounds and test commands.
2. Recheck current Rust/tools, grammar sources and all first-party versions.
   Run `cargo xtask freshness`; unavailable sources block freshness evidence.
3. Implement and test the scope; update status, public docs, changelog and
   `release-notes/vX.Y.Z.md`. Run local, compiler, platform and security gates.
4. Verify all package archives in dependency order in an isolated local registry
   or staging environment before publication; never publish to test packaging.
   The foundation local gate verifies core; sibling registry resolution and
   complete family publication remain later release work.
5. Record exact implementation commit, lockfile, compiler, target/profile matrix,
   grammar/Unicode revisions (or N/A), tests, corpus/fuzz summaries, benchmarks
   if claimed, graph/SBOM and archive checksums. External signing tools own
   signatures; Synir must not implement signing cryptography.

## Pentest stop for every version

Freeze the implementation candidate and identify its full commit hash. Pentest
scope includes changed code, negative/budget tests, CI/release controls and any
new trust boundary. The foundation pentest is a tooling/design review.
The report template is [security/pentest/TEMPLATE.md](../security/pentest/TEMPLATE.md).
Do not fabricate tester identity, results, PASS status or independence.

Resolve findings and rerun affected checks. A changed implementation invalidates
its old passing report: repeat the relevant pentest on the new exact candidate.
Record unresolved findings and dispositions. Release-blocking issues cannot be
waived by a passing local test suite. Preserve the final report in
`security/pentest/vX.Y.Z.md` with candidate hash, reviewer/date/scope, findings,
retest evidence and result. An evidence-only commit can follow the candidate;
verify that its diff changes only the report and release evidence, then bind the
signed tag to that commit and name the reviewed parent explicitly.

## Admit and publish

Confirm GitHub CI and CodeQL Default setup are green for the candidate. Confirm
private reporting, required checks and branch protection in repository settings.
Review the exact report/diff and release artifacts. The current metadata workflow
is a validation aid; it is not an automated publication or pentest approval gate.

Only after maintainer authorization, create/verify a signed tag, push it, and
publish reviewed packages in order: core, host, macros, facade. Keep tokens out
of CI jobs that process untrusted pull requests. Publish release notes and
checksums with the tag. Recheck registry contents and downstream installation.
Nothing in repository initialization authorizes tagging or publishing now.

1.0 additionally requires the full capability acceptance matrix, native platform
claims backed by execution, independent parser/emitter review, remediation,
API freeze and candidate retest. No milestone is complete because time ran out.
