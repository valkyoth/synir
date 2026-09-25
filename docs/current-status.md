# Current status

Candidate: 0.1.0, unpublished. Stage: workspace foundation.

| Capability | State | Evidence / owner |
| --- | --- | --- |
| Four first-party crates, safe-code policy | Scaffold implemented | Cargo manifests and local checks |
| no_std/no-allocator core boundary | Compile-verifiable scaffold | Platform gate; no syntax algorithms |
| Repository policy/freshness tools | Implemented | xtask tests and checks |
| Native compiler adapter | Planned | [Native tokens and portable views](RELEASE_PLAN.md#native-tokens-and-portable-views) |
| Derive parsing and strict attributes | Planned | [Declarations](RELEASE_PLAN.md#declaration-recognition-and-generic-projections) and [schemas](RELEASE_PLAN.md#literal-decoding-and-typed-attributes) |
| Manual emission and quotation | Planned | [Emission](RELEASE_PLAN.md#fallible-emission-and-usable-macro-profile) and [quotation](RELEASE_PLAN.md#quotation-and-generated-schemas) |
| Source lexer and Unicode/NFC | Planned | [Source frontend and Unicode](RELEASE_PLAN.md#source-frontend-and-unicode) |
| Full grammar, traversal and source edits | Planned | [Full syntax](RELEASE_PLAN.md#full-items-and-traversal) and [edits](RELEASE_PLAN.md#owned-syntax-and-lossless-edits) |
| Independent pentest / production | Not completed | Pentest and 1.0 acceptance gates |

See [setup results](verification/setup.md), [roadmap](RELEASE_PLAN.md), and
[original proposal](IDEA.md). A feature name or empty crate is not implemented
functionality. This file must change with every admitted capability.
