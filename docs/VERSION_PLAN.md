# Version policy

The canonical milestone contents live in [RELEASE_PLAN.md](RELEASE_PLAN.md).

| Version | Meaning |
| --- | --- |
| 0.1.0 | Unpublished workspace/documentation foundation candidate |
| 0.N.0 | One narrowly scoped implementation or assurance pass |
| 0.N.P | Focused corrective release with its own tests and pentest |
| 1.0.0-rc.N | Exact candidate for the complete admitted stable scope |
| 1.0.0 | First serious production-ready crate family |

No dates or maximum minor number are promised. Split oversized unpublished
passes into more minor versions, updating all maps before implementation.
Never hide new functionality in a patch or mark a milestone shipped from plans.
First-party crates are versioned together initially; exact internal versions
prevent accidental mixing. Review this policy before independent versioning.

Every tagged version needs release notes, verification, a pentest, remediation,
and signed-tag provenance. Tagging does not automatically publish crates.
Publication is explicit and follows dependency order: core, host, macros, facade.
Do not inherit Eth's five-minor publication cadence without a Synir decision.
The foundation is not currently published and package-name availability is not
claimed. No release workflow carries registry credentials or publishes artifacts.

Track four axes independently: Cargo API version; supported source grammar and
editions; Unicode/NFC data version; compiler-native adapter versions. At setup
only Cargo/compiler axes exist. Before 1.0, capability coverage and unresolved
limitations must be explicit. No API compatibility with existing libraries is
promised. Security fixes can reject malformed formerly accepted input; document
that compatibility impact rather than preserving failure-open behavior.
