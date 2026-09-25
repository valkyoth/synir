Synir — high-assurance Rust syntax and macro toolkit

Architecture proposal and review of the supplied plan
Review date: 25 September 2026
Status: Proposed design, not an implemented or independently audited product.

Executive decision

Build Synir, but change the central promise. The useful product is not “syn in 2,500 lines” or “automatically safer because it has no dependencies.” It is a self-contained, resource-bounded syntax and macro toolkit with explicit validation guarantees, a genuinely portable core, and an ergonomic path from input tokens to generated code.

The supplied plan contains good objectives: no third-party crate dependencies, safe Rust, native compiler-token integration, source parsing, shallow structural inspection, and integrated emission. Its example implementations do not establish those objectives. Several contain incorrect parsing, silently accepted malformed attributes, incomplete generic handling, allocation hidden behind “zero allocation” descriptions, and code that should not compile as written. Treat them as problem illustrations, not implementation foundations. [A:93–121, 230–237, 628–659, 993–1106, 1377–1439, 1585–1589]

The strongest initial product is a dependable replacement for the parsing, attribute decoding, and emission infrastructure needed by derive and declaration-oriented attribute macros. The broader architecture should also accommodate a full Rust syntax layer, standalone tools, and source-preserving edits, without forcing those costs onto the small profile.

Recommended public description:

Synir is a zero-third-party-dependency Rust syntax toolkit for predictable procedural macros and source tools. It combines bounded parsing, explicit validation, structured attribute decoding, and span-aware token generation, with a no_std core and optional allocation.

This document distinguishes facts checked against external documentation, findings from the attachment, and proposed engineering decisions. Reference identifiers are resolved in the final section. All new APIs shown below are proposed interfaces; they are not claims that these APIs currently exist. Rust compilation and performance benchmarks were not run for this review.

1. What to retain and what to correct

1.1 Retain the design direction

Keep the dependency policy, #![forbid(unsafe_code)], caller-controlled resource use, compiler-native spans, efficient structural views, and a single coherent parser-to-emitter experience. Keep support for custom macro languages; a toolkit limited to a fixed struct extractor will be much less useful than one with reusable parsing primitives. [A:93–121, 228–259]

Keep the Nordic project name as a branding choice. Do not make linguistic claims about its etymology without a separate verification; the architecture does not depend on them.

1.2 Correct the comparison with existing crates

Current Syn documentation describes substantial feature gating, including separate derive and full facilities. Its normal dependency manifest lists proc-macro2, unicode-ident, and optional quote; the development-dependency list is a different category. The attachment's “dozens of transitive compilation units” description is not an accurate general characterization of that small stack. Compare actual resolved configurations, not every dependency used to develop the upstream projects. [R01, R02]

There is also an important no_std nuance: the inspected Syn source declares #![no_std] but explicitly imports std. A header alone is not proof of suitability for a target without the standard library. Synir should prove that suitability with target builds. [R03]

Existing alternatives deserve a fair comparison. Venial is declaration-oriented. Unsynn combines parser construction and token emission and documents a native proc_macro configuration without proc_macro2; another feature controls its keyword-hashing dependency. Nanoserde's derive package advertises no external dependencies. Conversely, the inspected mini-internal package for Miniserde depends on Syn, Quote, and Proc-macro2. The attachment groups some of these together incorrectly. [R06, R07, R08, R09]

The relevant opportunity is therefore a better combination of contracts and usability, not proof that nobody has attempted small or dependency-light macro tooling.

1.3 Remove unsupported guarantees

Do not publish claims of instantaneous compilation, a fixed tiny codebase, zero vulnerabilities, or easy certification. No measurements or certification evidence in the attachment establish them. A comprehensive grammar, Unicode handling, correct quotation, and an adversarial test suite are significant maintenance commitments. [A:199–205, 372–376, 565–569, 1656–1665]

“Safer” should be decomposed into measurable properties: bounded internal work, no silent omission, no third-party package dependencies in the shipped family, safe implementation code, tested grammar coverage, and documented compiler boundaries.

2. Replacement scope: capabilities, not misleading package equivalence

Existing component

Synir replacement capability

Important boundary

syn

Derive/declaration views; eventually full syntax, traversal, transformation, and custom parsing

A derive-only subset is not full Syn replacement. API compatibility is a separate undertaking.

quote

Fallible builders, interpolation, repetition, span policies, typed emission

Must cover real quotation semantics, not only a token-muncher demonstration.

proc-macro2

Compiler adapter plus independent portable token representation

Native compiler tokens alone do not provide standalone operation or ordinary unit testing.

unicode-ident

Generated, versioned XID classification tables for source mode

Identifier normalization is an additional responsibility, not just XID classification.

darling

Typed attribute schemas and optional generated decoders

Include validation, duplicate handling, nested metadata, error accumulation, and forwarding.

venial

Lightweight declaration parsing with preserved opaque regions

This is a useful initial comparison rather than a separate compatibility target.

Ad hoc derive parsers such as nanoserde-derive internals

Reusable parser and emitter infrastructure

Serialization behavior and runtime traits remain a different product.

miniserde

Its macro frontend could be reimplemented using Synir

Synir is not a replacement JSON engine or serialization trait ecosystem.

deriver

Framework for writing individual derives

The inspected package provides a particular derive, not a general compiler frontend.

macro_railroad

Potentially an application built on a later macro-grammar parser

It generates syntax diagrams; that is not a core parser/emitter replacement requirement.

These boundaries follow the respective documented roles. [R01, R04–R12]

Rust's #[derive(...)] expansion mechanism remains part of the compiler. Synir can support custom derives; it does not replace the compiler's hook or make another ecosystem's traits disappear. [R13]

There are three distinct compatibility promises:

Behavioral replacement: a migrated macro accepts its documented inputs and produces equivalent behavior.

Source/API replacement: existing macro implementation code builds after changing imports. This is not an initial goal.

Dependency-graph replacement: the consumer's entire graph stops using the old libraries. This requires every relevant macro dependency to migrate; installing Synir cannot force that outcome.

3. Threat model and assurance boundary

3.1 Inputs and assets

Treat raw source, balanced but unusual compiler tokens, attribute arguments, generated names, quoting repetitions, and third-party macro expansions as potentially adversarial inputs.

Protect build availability, correctness of generated code, diagnostic integrity, source and span provenance, reproducibility, and consumers' ability to understand what was and was not validated.

The central security risk is not only memory corruption. A parser that silently misses a field can lead a downstream macro to omit a validation rule, serialization field, access check, or redaction decision. Those consequences depend on the macro, but the infrastructure must not quietly convert an error into success.

3.2 Guarantees Synir should own

Every shipped first-party crate should forbid unsafe implementation code. Every parser entry point should return a typed result. Invalid or unsupported syntax should not become successful partial parsing unless the caller explicitly requested an inspection API that advertises partial results.

Internal loops, recursion or explicit stacks, intermediate storage, and emitted output should be bounded. Error handling should itself have bounded work and output. Public documentation should describe each API's completeness and resource contract.

3.3 Guarantees it cannot honestly own

Procedural macros execute with the compiler's resources and have build-script-like security concerns. A helper library does not sandbox the calling macro, compiler, allocator, or arbitrary callbacks. no_std is not an operating-system access-control mechanism. [R13, R23]

forbid(unsafe_code) applies to the checked crate's implementation, not the implementation of the compiler and standard library. It also does not prevent a token emitter from spelling an unsafe block in generated source. Distinguish safe implementation from the safety of generated programs.

Synir cannot provide name resolution, type checking, borrow checking, trait solving, constant evaluation, or proof of application-specific authorization semantics merely by parsing syntax. A field spelled Secret<T> might be an unrelated imported type or alias. Provide syntactic information; do not present it as resolved semantic identity.

For high-assurance builds, require an external restricted build environment, controlled dependencies and toolchains, limited credentials, and operating-system resource limits. Those complement the library rather than being features implemented inside it.

4. Package architecture and dependency graph

Use one user-facing facade over three narrowly scoped implementation packages. All dependencies within this family are first-party, versioned together initially.

synir                         public facade
  ├── synir-core              mandatory; no third-party dependencies
  ├── synir-host              optional compiler/host adapter
  │     └── synir-core
  └── synir-macros            optional proc-macro quotation/schema helpers
        ├── synir-core
        └── synir-host

There must be no reverse dependency from core or host to the facade. The macro helper package must not depend on a facade that re-exports those same helpers. This prevents the bootstrap dependency cycle that easily appears in an “all-in-one” design.

4.1 synir-core

Always #![no_std], always #![forbid(unsafe_code)]. Optional extern crate alloc. No compiler tokens, filesystem calls, environment reads, or process execution.

Own token abstractions, ranges, source lexing, grammar recognition, typed views, literal decoding, attribute schemas, parse work accounting, portable diagnostics, token emission contracts, and optional owned syntax storage.

A minimal core configuration should compile with no allocator and no standard library.

4.2 synir-host

Ordinary host library using the compiler-provided proc_macro API and, where necessary, std. Own compiler-token import/export, original-span handles, native diagnostics rendering, and invocation-lifetime storage.

Compiler-backed objects must not escape the compiler invocation. They must not be serialized as if spans could later be recreated. The native handle types have thread restrictions; keep native state on its invocation thread. [R14, R15]

4.3 synir-macros

A proc-macro = true package providing optional try_quote!, spanned quotation, and schema/AST helper derives. Its implementation initially uses manually written core parsing and emission; it must not require its own procedural macros to compile itself.

Keep the manual builder fully functional without this package. That gives conservative consumers a smaller trusted component set.

4.4 synir facade

Expose stable module paths, profiles, and documentation. Consumers should normally depend on this package rather than coordinate internal versions.

The precise dependency claim is zero third-party Cargo dependencies in shipped packages. First-party packages still appear in the dependency graph and lockfile. Compiler-supplied libraries are part of the trusted toolchain, not external crates.io dependencies.

A consumer demanding literally one package can use synir-core directly. That is not equivalent to receiving the entire compiler-integrated facade in one artifact.

5. Feature and environment model

Use additive capabilities rather than “secure versus insecure” compile flags. Cargo unifies features, so enabling a feature must not weaken an existing parser's validation policy. [R22]

Capability/profile

Environment

Proposed content

Base core

no_std, no allocator

Tokens, cursors, diagnostics, caller-storage interfaces

source

no_std; caller storage or optional allocation

Source lexer and portable tokens

alloc

no_std + alloc

Owned buffers, caches, owned syntax, convenience decoders

derive

Core-compatible

Struct, enum, union, generics, fields, structural validation

attributes

Core-compatible

Metadata grammar and typed schemas

emit

Core-compatible

Fallible token builders and emission traits

proc-macro

Host

Native compiler adapter

quote

Host helper at build time; generated builder code can be portable

Compile-time quotation expansion

full

Larger core syntax capability

Expressions, patterns, statements, all supported item grammars

source-edit

Usually alloc

Checked text edits and formatting-preserving operations

macro-dev

Host preset

Derive, attributes, emission, native adapter, quotation

Choose default = [] to make the baseline unambiguously portable, but prominently document macro-dev as the normal one-line setup for macro authors. Do not require beginners to discover ten individual flags.

Proposed post-1.0 consumer configuration:

[dependencies]
synir = { version = "1", default-features = false, features = ["macro-dev"] }

This is an illustrative future manifest, not an instruction to install an existing 1.0 release.

Source-only users should not compile the native adapter. Enabling both native and source capabilities must expose both APIs, not silently change a public TokenStream alias into a different concrete type.

Choose and test an explicit minimum supported Rust version. Separately version the source grammar and Unicode tables; the compiler required to build Synir and the syntax Synir understands are not the same property.

6. Internal organization

synir-core/src/
  lib.rs
  limits.rs
  work.rs
  diagnostic.rs
  source.rs
  span.rs
  token/
    kind.rs
    range.rs
    tape.rs
    cursor.rs
    storage.rs
  lexer/
    rust.rs
    comments.rs
    identifiers.rs
    literals.rs
  unicode/
    xid.rs
    normalization.rs
    generated/
  parse/
    context.rs
    lookahead.rs
    separated.rs
    item.rs
    derive.rs
    generics.rs
    visibility.rs
    type.rs
    attribute.rs
    expression.rs
    pattern.rs
    statement.rs
  view/
  schema/
  emit/
    sink.rs
    builder.rs
    literals.rs
    generics.rs
    precedence.rs
    template.rs
  owned/
  edit/

synir-host/src/
  input.rs
  compiler_tokens.rs
  span_table.rs
  export.rs
  diagnostic.rs

synir-macros/src/
  quote.rs
  schema.rs
  bootstrap.rs

Keep grammar modules and storage mechanisms distinct. A grammar routine should not decide whether results live in a heap vector, caller-provided slots, or a source-backed view.

Avoid making every token category a separate published crate. Audit boundaries and compiler-host separation justify packages; every small implementation detail does not.

7. Token representations: two frontends, one grammar contract

7.1 Source frontend

Borrow &str as the standard input. Offer &[u8] only through checked UTF-8 validation or an explicitly separate invalid-input inspection API.

Store source tokens as compact entries referencing byte ranges in the original source. Preserve comments and whitespace as trivia. Use an indexed flat token tape with validated delimiter links. Tokens should record kind, spelling range, source location, and delimiter relationships.

This creates a clear allocation contract: source text is borrowed; token metadata occupies caller-provided storage or a budgeted owned buffer.

7.2 Compiler frontend

Do not assume proc_macro::TokenStream offers a borrowed slice of token trees. The public interface is opaque and iterable. Proc-macro2's documented purpose includes making related functionality available outside compiler macro execution; omitting it requires Synir to provide its own portable representation. [R04, R16]

For the first reliable implementation, import native tokens into an indexed, invocation-owned representation in one bounded traversal. Preserve original token objects and group handles in side storage. Record punctuation spacing, delimiter kind, individual spans, and token spelling where needed.

This import is O(n) in visited token structure and requires storage. Call it bounded import followed by borrowed views, not zero-copy, zero-allocation native parsing.

After this representation has been validated and benchmarked, lazy group import can be explored as an optimization. Its design must explain how quotas count unvisited payloads; otherwise laziness creates an accounting blind spot.

7.3 Common grammar interface

Use a small backend contract that exposes token kind, spelling access, group boundaries, and location handles. Keep backend-specific concrete types behind modules.

Opaque public TokenId, TokenRange, and NodeId types should have private fields and checked constructors. A range must be tied to its originating token store, not merely contain two interchangeable integers.

A local index from another parse must not be accepted accidentally. Associate ranges with the borrowed input or validate an input identity before use.

7.4 Text access

The current native Ident API exposes display formatting, not a borrowed as_str() method. Do not promise native identifier comparison using a borrowed string that does not exist. [R14]

Start with budgeted, cached spelling extraction. Reuse the original identifier object for emission. An optimized bounded formatting sink may later avoid some intermediate allocations, but any compiler-internal allocation remains outside Synir's control.

Do not stringify and reparse entire native token streams. That discards information that must remain attached to token objects. This prohibition does not forbid extracting one identifier's spelling for schema matching or creating a string literal from an explicitly supplied value.

8. Source fidelity, compiler spans, and hygiene

Define three different fidelity guarantees:

Source fidelity: unmodified source is reproduced byte-for-byte, including comments and whitespace.

Token fidelity: native token kinds, punctuation spacing, and groups are preserved to the extent exposed by the compiler API.

Provenance fidelity: copied tokens retain their original compiler spans and associated resolution context.

Only source input can offer complete original-text fidelity. Compiler token input does not expose a recoverable copy of every original whitespace/comment choice. Source text obtained through a compiler span is diagnostic, best-effort data; the Rust API explicitly says macro behavior should not depend on it. [R15]

Represent portable locations as source identity plus byte range. Represent native locations as invocation-local handles into original compiler spans. Do not present byte offsets as fabricated compiler spans.

Preserve group-opening and group-closing locations when the backend supplies them. Newly generated groups should use an explicit span policy; setting a group's span is not a blanket replacement of every descendant's span.

Invisible Delimiter::None groups require dedicated treatment. Rust documents precedence-related behavior and a compiler limitation affecting recreated invisible groups. Prefer forwarding original groups unchanged; use actual parentheses for newly composed expressions when precedence requires them. Test real compiler behavior rather than assuming an invisible group will preserve semantics. [R17]

Expose separate APIs for location selection and name-resolution context. Avoid the attachment's implicit assumption that every emitted token should inherit the struct name's span.

9. Resource accounting as a first-class API

9.1 Limits

Make limits explicit in parsing, decoding, transformation, and emission. A proposed shape is:

pub struct Limits {
    pub input_bytes: usize,
    pub tokens: usize,
    pub nesting_depth: usize,
    pub nodes: usize,
    pub decoded_literal_bytes: usize,
    pub requested_storage_bytes: usize,
    pub work_units: u64,
    pub output_tokens: usize,
    pub output_bytes: usize,
    pub diagnostics: usize,
}

Field names and exact units are API-design candidates, not a completed contract. Document what each counter includes. In native mode, compiler-owned storage is not included in a claim about Synir-controlled bytes.

Illustrative starting defaults for a host-macro profile could include 8 MiB of source spelling, 262,144 imported tokens, depth 128, and a separately bounded output. These are calibration candidates, not evidence-backed safe thresholds or promises about all legitimate projects.

9.2 Monotonic work budget

Charge work for token visits, lookahead, comparisons, literal decoding, Unicode processing, repeated parsing, diagnostics rendering, and output construction.

Backtracking may restore a cursor and abandon speculative nodes. It must never refund work already performed. Otherwise an attacker can repeatedly trigger expensive failed alternatives without exhausting the advertised budget.

Use one logical invocation budget across nested parsers, cache misses, schema validation, and emission. A parser should not reset its own budget by constructing child contexts.

9.3 Allocation and storage accounting

Budget logical capacity requests before allocation and use fallible reservation for Synir-owned vectors where applicable. Vec::try_reserve reports allocation/capacity failure, but it does not make every allocation in surrounding compiler APIs fallible or give byte-exact control of allocator overhead. [R24]

Track both live logical storage and cumulative work associated with allocation churn. Rewinding an arena can reduce live occupancy without refunding parsing work.

Strict no-allocator mode should use safe caller-provided arrays or slices of initialized optional slots. Reject exhaustion with StorageExhausted. Do not hide unsafe inside a homegrown small-vector implementation.

9.4 Stack safety includes non-parser operations

Use iterative traversal and flat storage for arbitrary-depth structures. Review clone, drop, debug formatting, equality, hashing, and serialization paths as well as parsing. A recursive destructor can still overflow after an iterative parser has accepted a deeply nested structure.

Where bounded recursion is retained for small grammar components, document and test the complete stack path. A nesting limit on one parser function is not a proof that every call chain is bounded.

9.5 Scope of the guarantee

Internal fuel accounting constrains cooperative Synir operations. It cannot interrupt an arbitrary user callback, a malicious custom parser implementation, or work already performed inside the compiler before Synir receives its input. External time and memory limits remain necessary for hostile build workloads.

10. Parsing model: inspect, validate, transform

The most important API improvement is to distinguish an item found from an item sufficiently validated for a particular operation.

10.1 Inspection

inspect_header may locate the item kind, name, and structural regions. It must return an inspection-specific type, not a value that implies all fields and types were checked.

Partial information is useful for editors, indexing, and pass-through operations. It becomes dangerous when a downstream derive mistakes it for a complete model.

10.2 Structural validation

A derive parser must recognize every field or variant, require legal separators, associate attributes correctly, identify generic parameters and where clauses, and consume the full enclosing item.

A lazy field iterator must not report successful parsing merely because the user stopped iterating. Either validate all field boundaries before exposing a derive-ready view or require an explicit finishing step that cannot be bypassed by the safe generation path.

10.3 Deep validation

Optional type, expression, and full-item validation should confirm that an entire region matches a documented grammar. It should not be described as type checking or proof that the compiler will accept the program after expansion.

Macro bodies and unresolved expansion-dependent syntax require explicit opaque representations; a parser cannot know another macro's custom grammar from its token body alone.

10.4 Suggested validity types

HeaderView
DeriveShape
ValidatedTypeSyntax
ValidatedAttributeValues
ValidatedImplPlan
RecoveredSyntax
OpaqueTokens

The default high-level emitter should accept validated plans. Low-level token forwarding remains available, but it must not silently turn RecoveredSyntax into a trusted semantic model.

An OpaqueTokens range records why it is opaque and what is known about its boundary. Preserving a region is not evidence that its internal syntax or meaning was understood.

10.5 No successful prefix parsing by accident

Distinguish parse_all from deliberately named parse_prefix. Public derive entry points should require end-of-input. Recovery should be an inspection operation with diagnostics, not an implicit branch that converts malformed input into a shorter valid AST.

11. Correct Rust recognition without unnecessary AST allocation

The attachment skips fields until a comma and counts </> to find generics. These methods fail on ordinary Rust types and function bounds. Its where-clause parser also stops at any group, which confuses function parameter groups with an item body. [A:749–767, 993–1106, 1493–1499]

Consider syntax such as:

struct Example<T, E> {
    first: Result<T, E>,
    callback: fn(T) -> Result<T, E>,
}

A comma inside Result<T, E> does not terminate a field. Similarly, the > in an arrow is not a generic-list closing delimiter.

Build a grammar recognizer that understands enough context to determine boundaries correctly. It need not allocate a full AST for every recognized subproduction. Avoiding materialization is different from avoiding recognition.

Use bounded recursive-descent or explicit-state parsers for declarations and types. Use token-class lookahead and limited speculation. Expression support should use precedence-aware parsing plus dedicated handling for constructs whose grammar is not just a binary-operator chain.

Represent list parsers with explicit separator and trailing-separator rules. Every repetition combinator must prove progress or fail; a parser that succeeds without consuming input must not make many() loop forever.

Boundary recognition must account for qualified paths, associated bindings, function pointers, higher-ranked bounds, trait objects, arrays containing const expressions, nested macros, and contextual punctuation. Unsupported forms should produce a specific error, not a guessed comma boundary.

12. Generics and safe implementation generation

The existing plan correctly notices that generic declarations and type applications are different. It does not fully implement the distinction, particularly for defaults and where-clause positioning. [A:943–975, 1155–1188]

Keep three projections over one parsed generic parameter model:

Declaration parameters: original bounds, attributes, and defaults as allowed in that declaration.

Implementation parameters: parameters suitable for an impl, with defaults removed where required.

Type arguments: only the applied lifetime, type, and const arguments.

The grammar also distinguishes the position of a tuple struct's where clause from a braced struct's. Follow the Rust grammar rather than scanning every item the same way. [R19, R20]

For example, this original declaration:

struct Packet<'a, T: ?Sized = [u8], const N: usize = 16>
where
    T: 'a,
{
    value: &'a T,
    flags: [u8; N],
}

should permit generation of:

impl<'a, T: ?Sized, const N: usize> Packet<'a, T, N>
where
    T: 'a,
{
    pub const FIELD_NAMES: &'static [&'static str] = &["value", "flags"];
}

No generic default belongs in that impl parameter list. The field-name operation introduces no reason to add T: Clone, T: Debug, or other incidental bounds.

Provide helpers corresponding to split_for_impl, but return validated projection views rather than duplicated token streams. Allow explicit addition of predicates by the macro author. Do not attempt semantic predicate deduplication using textual equality.

For other derives, let authors state their bound policy. A field-type bound such as FieldType: SomeTrait can differ from requiring every type parameter to implement that trait. Neither policy is universally correct.

Preserve existing spans, parameter order, and user-written predicates. Test parameters with raw identifiers, associated types, const defaults, lifetimes, empty parameter lists, and item-specific where-clause forms.

13. Attribute parsing that can replace Darling's role

13.1 Separate grammar, decoding, and policy

First parse attributes into syntax: path, flag, assignment, list, and explicitly opaque payloads. Then decode expected values. Finally apply the macro's schema and semantic rules.

Rust attributes are not universally limited to the attachment's flag/string/bool shape. The general parser should preserve valid syntax even when a particular helper schema intentionally accepts a smaller language. Attribute macro expansion order also means Synir must not assume every embedded expression has already been evaluated. [R21]

13.2 Strict schema defaults

An owned namespace should reject unknown keys by default. Reject wrong types, missing required values, duplicate non-repeatable keys, forbidden combinations, and unexpected trailing tokens.

For example:

#[describe(rename = "first", rename = "second")]
#[describe(skip = perhaps)]
#[describe(renmae = "name")]

should produce clear diagnostics, not silently choose one value, interpret a typo as a flag, or ignore an unknown directive.

Unrelated attributes should remain untouched. Strictness inside #[describe(...)] does not mean rejecting every attribute owned by another macro.

13.3 Schema facilities

Support required and optional fields, defaults, validated strings, booleans, checked integers, paths, enum choices, nested records, repeatable keys with explicit ordering, and custom validation callbacks.

Container, variant, field, and generic-parameter schemas should be distinct contexts. A container-only key on a field should be an error with a suggested placement.

For mutually exclusive choices, diagnostics should point to both the conflicting occurrence and the earlier relevant occurrence when the rendering backend supports it.

An initial schema API can be handwritten and static. An optional FromMeta-style helper derive should generate the same decoder without becoming mandatory. This is how the product replaces useful Darling workflows rather than merely providing get_str() and has_flag(). [R05]

13.4 Do not silently lose provenance

Store the location of the attribute key and value separately. A malformed escape should point to the literal; a duplicate should point to the second key. Error collections should be bounded and returned deterministically in source order.

13.5 Fix the attachment's failure-open behavior

The sample maps some unexpected value tokens to AttrValue::Flag and skips other unknown syntax. That is unacceptable for a security-oriented helper attribute. Parsing must return Result, and invalid syntax must not activate an option. [A:1390–1439]

14. Literal and Unicode responsibilities

14.1 Literal decoding

Removing surrounding quotes is not string decoding. The attachment's decoder mishandles escapes and raw string spellings. Its fallback also treats non-string literal text as if it were a string option. [A:1393–1402]

Implement Rust literal recognition and schema-specific decoding as separate layers. Preserve the original literal token for forwarding; produce a decoded value only when requested. Numeric decoding should reject overflow and unexpected suffixes instead of wrapping or silently accepting a prefix.

Support the literal forms required by the published grammar baseline, including ordinary and raw strings, byte-oriented literals, characters, numeric spellings, and applicable C-string forms. Honor each form's escape and validity rules. [R18]

No-allocator users need a decoding iterator or caller-provided output buffer. An owned string convenience method belongs behind alloc and consumes the decoded-byte budget.

14.2 Identifier classification

Raw source needs correct Unicode XID_Start and XID_Continue behavior, raw-identifier rules, and keyword/edition handling. Rust also compares identifiers after NFC normalization, and compiler macro inputs already carry normalized identifiers. [R10]

Do not substitute char::is_alphabetic() for Rust identifier validation.

Maintain generated static Unicode tables from a pinned official data release. Record input checksums, generation commands, licensing notices, and the grammar/Unicode version in release metadata. Run the generator offline as a maintainer tool, not during downstream builds.

14.3 Normalization is a separate subsystem

An XID classifier does not by itself normalize strings. Source-mode semantic identifier equality needs a normalization strategy or an API that explicitly restricts itself to lexical spelling comparisons.

Use an ASCII fast path, then bounded normalization where necessary. Account for combining sequences, output storage, and comparison work. Preserve original source spelling for exact edits; keep canonical spelling separate for identifier identity.

A limited first source profile may explicitly reject non-ASCII identifiers. It must be named as a restricted profile and cannot be advertised as a complete Rust lexer. Native macro input can still forward valid Unicode identifiers without requiring source-mode normalization.

Confusable-name and bidirectional-control analysis can be an optional policy layer. Do not silently rewrite user identifiers or label every non-ASCII identifier unsafe.

15. Emission: make the safe path ergonomic

15.1 Fallible sink contract

The primary emission trait should propagate resource and validation failures:

pub trait TryToTokens<S: TokenSink> {
    fn try_to_tokens(&self, sink: &mut S) -> Result<(), EmitError>;
}

This is a proposed interface; exact trait associated types should be settled through prototypes.

Provide native-token, portable-token, and source-text sinks. A text sink must insert separators that prevent accidental token fusion, not simply concatenate token display strings.

Output limits apply to every sink, including repetitions and literal escaping. A native sink's final compiler allocation is still part of the documented external boundary.

15.2 Validated construction

Expose checked identifier construction, raw-identifier construction, supported punctuation enums, typed literal constructors, and group builders that cannot be finished with mismatched delimiters.

The current compiler Ident::new and new_raw APIs can panic on invalid names. Synir should validate untrusted names before calling them, with a documented relationship between its identifier tables and supported compiler versions. Initially, generated ASCII names plus forwarding original identifiers is a particularly clear trusted path. [R14]

Avoid arbitrary “push string as code” methods. A string value should emit a literal by default. Inserting Rust source should require an explicit parse operation returning a typed or explicitly opaque token sequence.

15.3 Span and hygiene policy

Provide named policies for copied input, caller-visible generated items, and internal helper identifiers. Use absolute library paths and caller-supplied dependency paths where appropriate.

Do not promise that a generated unusual name is collision-proof. Prefer scoped helper items, controlled bindings, and correct resolution context. Procedural macro hygiene remains constrained by the compiler model. [R13, R15]

For a renamed runtime dependency, allow the user to provide its path explicitly. Do not make the high-assurance profile scan Cargo.toml, read the environment, or guess a package rename behind the caller's back.

15.4 Expression-aware emission

When a typed expression is embedded into a larger expression, its emitter must understand precedence and associativity. Parenthesize conservatively where the position requires it.

Raw token interpolation should remain exactly that: raw tokens with no invented guarantee that their meaning is safe in every syntactic position. Offer an explicit typed-expression interpolation path so users can request correct structural grouping.

15.5 Transactional output

High-level expansion should validate the plan before publishing output. On any failure, discard the incomplete generated stream and emit a bounded diagnostic. Never return a truncated implementation as successful output.

Reserve enough diagnostic capacity to report an output-limit failure. This is a logical reservation, not a guarantee that a process suffering compiler-level OOM can always recover.

16. Quotation beyond the attachment's prototype

The supplied emit! macro is not a complete Quote replacement. It relies on omitted helpers and does not implement the full grouping, interpolation, and repetition surface expected by macro authors. [A:469–525]

Provide a small reliable manual builder first, then quotation compiled by the optional synir-macros package. The quotation compiler should transform a template into direct builder calls or a compact validated instruction sequence, rather than recursively stringifying every token at macro runtime.

The eventual quotation contract should include scalar interpolation, all delimiter kinds, punctuation jointness, lifetimes, nested repetition, separators, optional and nonempty repetition, explicit span selection, and a documented way to escape template marker syntax. These correspond to genuine quotation use cases, not merely cosmetic convenience. [R25]

A proposed usage is:

// Proposed Synir API, not currently runnable library code.
let output = synir::try_quote!(cx => {
    impl #impl_generics #name #type_generics #where_clause {
        pub const FIELD_NAMES: &'static [&'static str] = &[
            #(#field_names),*
        ];
    }
})?;

For multiple iterators in one repetition, define the length contract. Prefer an error for mismatched finite sequences rather than silent data omission. Do not clone and buffer an unbounded iterator without an explicit budget.

Bound both template compilation and template execution. Replacing a runtime parser with an enormous compile-time recursive macro is not automatically a compile-time improvement.

Use a hygienic public declarative wrapper to pass the framework's $crate path to the internal generator, or another tested equivalent. Quotation must work when the dependency has been renamed. Never guess that the public crate is spelled synir in every consumer.

The compiler's own experimental quotation API should not be assumed stable merely because a rustdoc page exists. The inspected native quote! documentation marks it experimental. Keep the stable Synir design independent of nightly-only features. [R26]

17. Diagnostics as a product feature

Use portable structured errors with stable error codes, a primary location, optional secondary locations, and bounded message arguments.

Examples of useful categories:

UnexpectedToken
UnexpectedEnd
TrailingInput
UnsupportedSyntax
InvalidLiteral
UnknownAttributeKey
DuplicateAttributeKey
ConflictingAttributeKeys
WorkLimitExceeded
DepthLimitExceeded
StorageExhausted
OutputLimitExceeded

The no-allocator form should fit in fixed storage and render through core::fmt::Write or an equivalent caller sink. Allocating renderers are conveniences, not requirements.

For stable native macro execution, return a compile_error! token sequence at the offending span. The current compiler API exposes some location methods on stable, while richer diagnostics and span joining remain experimental; do not build the baseline around nightly methods. [R13, R15]

Batch errors where independent validation can continue safely, but cap their number. Include a final truncation notice rather than allocating an unbounded diagnostic list.

Escape control characters in displayed user text. Do not dump full token streams, local absolute paths, or sensitive literal values by default. Diagnostics are part of the information-disclosure surface.

Never silently return an empty token stream after a failed derive. That can hide a failure and make the resulting error appear elsewhere—or omit intended behavior.

18. Full Rust syntax support without bloating the common profile

The long-term replacement for Syn's full-syntax workflows needs more than derives. Plan a separate optional grammar layer over the same tokens and diagnostics.

It should cover supported item kinds, signatures, types, patterns, statements, expressions, attributes, paths, generic arguments, and relevant macro syntax. Maintain an explicit support matrix tied to language features, not a marketing percentage.

Use typed borrowed views over a syntax index for inspection. Offer owned arenas for users who need transformations or persistent storage. Do not immediately choose a reference-counted green tree solely because it is fashionable; a flat tape and arena are simpler for one-shot macros.

A source-editing or incremental-analysis profile can later justify an immutable concrete syntax tree. Its additional sharing, invalidation, and revision-lifetime rules should be designed as tooling requirements, not forced onto every derive macro.

Add visitor, structural mapping, and fold-style facilities. For borrowed structures, prefer explicit edit plans to arbitrary in-place mutation of shared source ranges. Changes should be validated before being committed.

Do not expand macros or resolve names implicitly. Preserve expansion-dependent regions with clear status. A “full syntax” claim should specify how macro bodies and unstable syntax are represented.

Unknown future syntax should never be silently reclassified as a harmless known construct. Provide explicit unsupported-syntax errors or intentionally opaque preservation depending on the chosen operation.

19. Source edits and standalone tools

For exact-source tools, keep the original source plus a checked list of edits:

Edit = source identity + byte range + replacement

Validate UTF-8 boundaries, source identity, edit ordering, and overlap. Define insertion ordering for equal offsets. Reject conflicting replacements instead of applying them in an incidental container order.

Apply edits in one pass to a sink or a budgeted output buffer. A no-op edit plan must reproduce the original bytes. An edit must not silently apply to a different source revision with matching length.

Comments and whitespace should remain untouched outside edited regions. Pretty printing is a separate feature from lossless editing; users should be able to choose.

Standalone source tests are valuable even for macro authors because the parser core can run in ordinary unit tests. Native integration still requires real compiler-driven macro fixtures; source spans are not a complete simulation of compiler hygiene. [R04]

Do not automatically read source files referenced by mod, include!, or attributes. File loading belongs to an explicit host application interface with its own path and resource policy.

Similarly, do not silently evaluate cfg against the host's environment. A source tool should receive an explicit configuration context or preserve unresolved conditional alternatives. A native macro should work from the tokens the compiler supplies and respect expansion ordering. [R21]

20. End-user experience and adoption

20.1 Primary users

The direct audience is procedural-macro authors and Rust source-tool authors. Their users should benefit without learning Synir's internal syntax model.

A migrated macro should ideally keep its existing public derive and helper-attribute syntax. Requiring every application to rewrite its declarations merely because a macro changed parser libraries makes adoption harder.

20.2 A useful first example

Use the attachment's Describe idea as an end-to-end acceptance project, but make it robust. Cover named, tuple, and unit structs; enums; generics; documented union behavior; per-field options; and diagnostics. [A:577–599, 913–932]

The target-side contract can remain tiny:

pub trait Describe {
    const FIELD_NAMES: &'static [&'static str];
}

The derive implementation should generate only the intended metadata and reference the agreed trait path. It should not impose unrelated bounds on generic parameters or link parser machinery into the runtime output.

20.3 High-level workflow

The recommended macro-development path should be:

native input
    -> bounded token import
    -> complete derive-shape validation
    -> typed helper-attribute decoding
    -> application-specific plan validation
    -> generic projections and bound policy
    -> fallible emission
    -> output or precise compile_error

Proposed entry-point style:

// API sketch: implementation details are intentionally not claimed to exist.
#[proc_macro_derive(Describe, attributes(describe))]
pub fn derive_describe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    synir::native::expand(input, synir::Config::macro_defaults(), |cx| {
        let input = cx.parse_derive()?;
        let plan = build_describe_plan(&input, cx)?;
        plan.emit(cx)
    })
}

The context design must be prototyped for ergonomic Rust borrowing. One sensible implementation separates an immutable input store from mutable work and output state, allowing views to borrow the input while decoding and emission consume budget.

20.4 Required examples

Publish complete, compiled examples for a metadata derive, validated helper attributes, an error-type derive, an attribute macro that preserves function bodies, a custom DSL, a standalone source inspection program, and a no-allocator parser with caller storage.

Include examples with renamed dependencies and no_std target consumers. “No runtime overhead” should mean the expansion does not introduce unintended runtime support, not a blanket assertion about whatever application code a macro generates.

20.5 Familiar vocabulary

Expose familiar operations such as parse-all, punctuated sequences, split-for-impl, visits, spanned errors, and token emission. Migration friendliness does not require making every Syn public type a permanent compatibility obligation.

Offer migration documentation by workflow. Do not implement a giant compatibility facade before the small native API has real users.

21. Security regression findings from the supplied examples

These are static findings about the supplied text, not results of compiling a completed repository.

Finding

Consequence

Required replacement

skip_until_keyword("struct") scans rather than parses the start of the item

Unexpected preceding syntax can be discarded

Parse allowed attributes/visibility/item kind in order

Field scanning stops at any comma

Nested generic arguments can become false fields or truncate parsing

Grammar-aware type/field boundary recognition

Angle counting handles every > alike

Function arrows and contextual operators can terminate generics incorrectly

Context-aware punctuation recognition

Generic parsing returns early when no < is present

A where clause without generic parameters may not be handled

Parse item-specific where clauses independently

Where scanning stops at every token group

Function-trait bounds and other groups can be mistaken for bodies

Parse where predicates and item body position

Generic declarations are reused for impl

Defaults can be emitted where inappropriate

Separate declaration, implementation, and argument projections

Later examples discard parsed generics

Earlier functionality is silently regressed

One integrated AST/plan model with regression fixtures

Attribute parser maps unexpected values to flags

Invalid input may activate a behavior

Typed errors; no success fallback

Literal decoding strips quotes only

Escapes and raw strings are interpreted incorrectly

Complete literal decoder for supported forms

Duplicate/unknown helper keys are not strictly handled

Intent can be ambiguous or misspelled behavior ignored

Schema-level duplicate and unknown-key policy

`.unwrap_or_else(



&field.ident.to_string())` borrows a temporary

The shown fallback has an invalid returned reference

Keep an owned fallback or branch and emit within its lifetime

Expected input errors use panic!

Generic macro failure rather than deliberate diagnostics

Structured errors rendered as compile_error!

Vec and .collect() are called in “zero-allocation” examples

Resource behavior differs from the advertised contract

Honest storage accounting and no-allocator alternatives

Quotation references missing helpers and covers limited forms

The example is not an implemented general Quote replacement

Tested builder first; separately specified quotation compiler

Source anchors: A:628–659, 677–686, 709–767, 864, 993–1106, 1155–1179, 1377–1439, 1483, 1533–1597.

Do not assign vulnerability severities to these fragments as if they were an operational product. Their impact depends on how a real macro consumes the result. They are nevertheless sufficient reason not to implement the architecture by incrementally polishing the snippets.

22. Verification strategy

22.1 Unit and property tests

Test token boundaries, delimiter links, range ownership, cursor restoration, parser progress, quota exhaustion, literal decoding, Unicode classification, generic projections, schemas, and emission.

Specify properties such as:

successful parse_all => full input consumed
cursor rollback => input position restored, work not refunded
no-op source edit => identical bytes
malformed owned attribute => error, never a default flag
validated derive shape => every field/variant accounted for
output limit failure => no partial successful expansion

These are concrete invariants suitable for ordinary tests, property generators, and bounded verification tools.

22.2 Native compile fixtures

Create real consumer crates that invoke actual proc macros. Test valid compilation, intentional errors, generated behavior, dependency renaming, generic defaults, macro_rules forwarding, invisible groups, and span selection.

Do not use a standalone call to compiler-native TokenStream as a substitute for a macro execution fixture. Native APIs are context-specific. [R04]

For diagnostics, compare stable aspects such as error code, key text, and relevant source location rather than every incidental line of the compiler's display output.

22.3 Differential testing

Run a separate assurance workspace or external tool against supported Syn configurations, a second implementation where useful, and compiler fixtures. A disagreement is a triage event, not automatic proof that Synir or the comparison parser is wrong.

Separate syntax validity from semantic validity. Rustc may reject a syntactically valid test because names are unresolved or trait bounds are unsatisfied; that does not automatically demonstrate a parser error.

For native/source comparisons, compare the defined normalized structure, not whitespace that native tokens no longer contain. Test hygiene using native compilation rather than textual equality.

22.4 Fuzzing and adversarial workloads

Fuzz source bytes, legal and malformed token structures, near-valid generic syntax, nested groups, huge literals, long identifiers, duplicate metadata, output repetitions, and edit conflicts.

Include denial-of-service objectives: growth in visits, allocations, stack depth, diagnostics, and output—not just crashes. Run compiler-facing cases with external time and memory limits.

Check all public parser and decoder error paths. Exercise fuel exhaustion during speculative parsing and during error reporting itself.

22.5 No external crate policy during verification

Keep the shipped workspace free of third-party normal, build, and development dependencies if that is the desired strict policy. Put optional comparison libraries and fuzzing harness dependencies in an excluded, separately locked assurance workspace.

External verification tools do not need to become shipped library dependencies. A standard-library-only test runner can execute compiler fixtures and deterministic generators. Document precisely which tools are needed to reproduce which assurance evidence.

This is preferable to weakening verification merely to make a development machine contain no outside software.

23. Required test matrix

Dimension

Required cases

Environments

Host OS builds, bare-metal no_std core, no-allocator core, no_std + alloc source mode

Toolchains

Explicit MSRV, supported stable toolchain, forward-compatibility checks on beta/nightly without requiring their APIs

Language

Supported editions and named grammar baseline; raw identifiers and contextual keywords

Data shapes

Named, tuple, unit, enum variants, union parsing with explicit derive policy

Generics

Lifetimes, defaults, const parameters, where placement, function bounds, associated types

Tokens

Joint punctuation, lifetimes, negative literals, invisible groups, nested macro forwarding

Attributes

Unknown keys, duplicates, conflicts, nested metadata, wrong types, escaped/raw strings

Memory

Tiny scratch buffers, exact-limit buffers, allocation reservation failure where testable

Work

Fuel exhaustion at every major phase; repeated failed alternatives

Output

Oversized repetition, escaped literals, precedence-sensitive expression insertion

Integration

Renamed framework/runtime crates, namespace shadowing, no_std consumers

Editing

UTF-8 boundaries, overlaps, stale source identity, no-op fidelity

Build minimal and combined feature configurations explicitly. --all-features alone does not verify that --no-default-features works. [R22]

Compiler and source grammar changes should trigger focused updates to this matrix, not merely a version-number bump.

24. Benchmark plan and claims policy

Do not measure only the parser on a tiny struct and call the result “compile-time savings.” Measure the user workflow.

Use equivalent tasks and documented upstream features: derive-only inspection, attribute decoding, a complete derive expansion, declaration-preserving attribute macros, and full-source transformations.

Measure cold builds, warm incremental builds, macro execution, peak resident memory, Synir-controlled allocations, imported token count, emitted token count, and failure behavior under hostile inputs. Keep native token import in the benchmark when the competitor's import is included.

Measure both compile CPU time and wall-clock critical path. Record toolchain, optimization profile, hardware, cache conditions, parallelism, exact crate versions, and fixture corpus.

A small helper proc-macro package can increase build cost even if its generated runtime code is efficient. Benchmark the manual builder and quotation profile independently.

Use published results to make narrow claims: “On this corpus and configuration, the derive profile used X less memory,” not “all macros compile instantly.” No numeric speedup is established by this review.

25. Supply-chain and release controls

Require no build scripts in the shipped family unless a later explicit design decision demonstrates necessity. Generated tables and grammar support files should be committed and reproducible; downstream builds must not fetch data or regenerate them.

Use restricted publication permissions, reviewed release changes, tagged source revisions, locked release and assurance environments, and independent review for parser, Unicode, and emission changes where practical.

Generate a release evidence bundle containing the shipped dependency graph, supported configurations, toolchain/grammar/Unicode versions, test results, fuzzing summary, benchmark methodology, known limitations, and security advisories.

Do not implement custom cryptography inside Synir merely to sign releases. Release signing belongs to established external tooling and organizational controls.

Distinguish package identity from publisher identity. Replacing several established dependencies with a new first-party family reduces the number of separately maintained components, but consumers still need to assess the new maintainer, release process, and code quality.

Publish an actual security policy with a reporting channel, triage criteria, supported release lines, and patch expectations. Avoid claiming a certification or standards guarantee without the corresponding process and assessment.

26. Compatibility and change policy

Document four version axes separately: library API, source grammar, Unicode data, and compiler-adapter support.

A future grammar change can cause syntax previously classified as opaque to become recognized. Avoid downstream code that treats opaque syntax as semantically harmless. Provide explicit rejection helpers and completeness checks.

Security fixes may intentionally reject previously accepted malformed input. Document such changes and add regression fixtures. Do not retain failure-open parsing solely to preserve accidental compatibility.

Feature additions must not silently relax limits, change error handling to recovery, normalize previously preserved source bytes, or replace concrete backend types. Strict behavior should be selected by an explicit operation/configuration contract, not depend on whether another library enabled a “permissive” feature.

For deeply constrained consumers, publish which profiles receive the strongest boundedness evidence. A large source-editing profile and a small no-allocator parser may share primitives without having identical assurance evidence.

27. Versioned delivery roadmap

The version ranges below are organizational milestones, not dates, duration estimates, or claims that every phase fits a single release. Split further whenever a change is difficult to review.

Releases

Scope

Exit requirement

0.1.0

Contracts, threat model, workspace DAG, minimal errors and limits

Core builds without std/alloc; no third-party shipped dependencies

0.2.0–0.3.x

Token IDs/ranges, storage, cursor, native import, portable fixtures

Range ownership and resource accounting tests pass

0.4.0–0.6.x

Derive shapes, visibility, fields, variants, generics

Grammar-aware boundaries; defaults and where clauses handled

0.7.0–0.9.x

Literal decoding and strict helper-attribute schemas

No silent unknowns, duplicates, type confusion, or lost fields

0.10.0–0.12.x

Fallible builders, generic projections, native export, diagnostics

Robust Describe example and real native compile fixtures

0.13.0–0.15.x

Quotation and generated schema decoders

Repetition, punctuation, spans, renames, and quota failures tested

0.16.0–0.19.x

Independent source lexer, Unicode/NFC work, no-allocator interfaces

Published lexical support matrix and backend comparison tests

0.20.0–0.23.x

Rich declaration/type support, custom DSL ergonomics, migration examples

Several materially different macro projects migrated

0.24.0–0.29.x

Optional expressions, patterns, statements, full syntax, traversal

Explicit grammar matrix and compiler/differential corpus coverage

0.30.0–0.33.x

Optional owned syntax and source edits

No-op fidelity, checked edits, safe traversal/drop behavior

0.34.0–0.39.x

External review, hostile-input hardening, performance, API stabilization

Release evidence and known limitations published

1.0.0

Stable, accurately scoped public contracts

Only verified profiles and capabilities receive stable guarantees

Do not delay the first useful macro profile until the entire full-syntax layer exists. Publish working pre-1.0 releases for real feedback. Equally, do not label an incomplete grammar a full Syn replacement to accelerate adoption.

A focused macro toolkit can stabilize before full-source transformation support; its release notes must say exactly which replacement workflows are covered. The larger plan can continue without compromising the small profile's contracts.

28. First implementation tasks

Start with a contracts document and ten adversarial fixtures, not a large API surface. Establish the core/host separation and prove that no-allocator builds actually work.

Implement checked token IDs and ranges, an immutable tape, budgeted storage, and a cursor with limited transactional lookahead. Test rollback without work refunds before adding elaborate grammars.

Import native tokens while preserving original handles. Create one real proc-macro fixture that forwards input without modification and another that reports a deliberate error at a selected token.

Implement the smallest honest named-struct parser, followed immediately by tuple/unit forms, nested generic type boundaries, and the generic/where-clause cases that break the supplied snippets. Reject unsupported forms explicitly at every stage.

Add strict literal and attribute decoding before generating behavior controlled by those attributes. Then build the manual emitter and robust Describe acceptance project.

Only after this path is correct should quotation, large grammars, incremental trees, or compatibility facades be introduced. The benchmark and fuzz harness should grow alongside each subsystem, not be reserved for the end.

29. Final recommendation

Synir is worth building as a high-assurance engineering project. Its practical value should be the removal of repetitive, error-prone macro plumbing while giving consumers better control over completeness, resource use, and dependency exposure.

The winning combination is:

small selectable capabilities, explicit trust boundaries, correct grammar recognition, typed configuration, faithful token forwarding, fallible ergonomic generation, and evidence-backed releases.

The losing combination is:

a tiny handwritten parser that silently skips unfamiliar syntax, wrapped in broad security and performance claims.

Keep the original ambition, but replace its prototypes and marketing guarantees with the contracts in this architecture. That would create a useful alternative for real macro authors and their downstream users, rather than simply a smaller dependency list.

Sources and review provenance

A refers to the supplied Pasted markdown(2).md attachment, using the numbered extraction lines surfaced in the conversation. Source-derived findings refer to that text; architectural decisions beyond it are proposals in this document.

External primary documentation was consulted on 25 September 2026. Live latest pages can change. The versions below are the versions exposed by the consulted pages; preserve exact source snapshots when making a project release.

ID

Source

R01

Syn 3.0.6 crate documentation; optional features and parser scope

R02

Syn 3.0.6 packaged Cargo manifest; normal versus development dependencies

R03

Syn 3.0.6 lib.rs; no_std header and explicit standard-library import

R04

Proc-macro2 1.0.107 documentation; standalone operation and unit-testing role

R05

Darling 0.24.1 documentation; schema/attribute-decoding capabilities

R06

Venial 0.6.1 documentation; lightweight declaration parser

R07

Unsynn 0.3.0 documentation; parsing, emission, and configurable backends

R08

Nanoserde-derive 0.2.1 package metadata

R09

Mini-internal 0.1.46 package metadata and dependencies

R10

Rust Reference, Identifiers; XID rules and NFC normalization

R11

Macro_railroad 0.1.9 documentation; syntax-diagram generation

R12

Deriver 0.0.0 documentation; From derive

R13

Rust Reference, Procedural macros; execution model, derives, hygiene, diagnostics

R14

Rust proc_macro::Ident API documentation

R15

Rust proc_macro::Span API documentation

R16

Rust proc_macro::TokenStream API documentation

R17

Rust proc_macro::Delimiter API documentation

R18

Rust Reference, Tokens; literal and token forms

R19

Rust Reference, Generic parameters

R20

Rust Reference, Structs

R21

Rust Reference, Attributes

R22

Cargo Book, Features; additivity, unification, and testing

R23

Rust Reference, Preludes; no_std behavior

R24

Rust alloc::vec::Vec::try_reserve documentation

R25

Quote 1.x quote! documentation

R26

Rust proc_macro::quote! documentation; experimental status

Source locations:

R01 https://docs.rs/syn/3.0.6/syn/
R02 https://docs.rs/crate/syn/3.0.6/source/Cargo.toml
R03 https://docs.rs/syn/3.0.6/src/syn/lib.rs.html
R04 https://docs.rs/proc-macro2/1.0.107/proc_macro2/
R05 https://docs.rs/darling/0.24.1/darling/
R06 https://docs.rs/venial/0.6.1/venial/
R07 https://docs.rs/unsynn/0.3.0/unsynn/
R08 https://docs.rs/crate/nanoserde-derive/0.2.1
R09 https://docs.rs/crate/mini-internal/0.1.46
R10 https://doc.rust-lang.org/reference/identifiers.html
R11 https://docs.rs/macro_railroad/0.1.9/macro_railroad/
R12 https://docs.rs/deriver/0.0.0/deriver/
R13 https://doc.rust-lang.org/reference/procedural-macros.html
R14 https://doc.rust-lang.org/proc_macro/struct.Ident.html
R15 https://doc.rust-lang.org/proc_macro/struct.Span.html
R16 https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
R17 https://doc.rust-lang.org/proc_macro/enum.Delimiter.html
R18 https://doc.rust-lang.org/reference/tokens.html
R19 https://doc.rust-lang.org/reference/items/generics.html
R20 https://doc.rust-lang.org/reference/items/structs.html
R21 https://doc.rust-lang.org/reference/attributes.html
R22 https://doc.rust-lang.org/cargo/reference/features.html
R23 https://doc.rust-lang.org/reference/names/preludes.html
R24 https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.try_reserve
R25 https://docs.rs/quote/latest/quote/macro.quote.html
R26 https://doc.rust-lang.org/proc_macro/macro.quote.html

just to be clear I am not looking for api compatibility, just a modern 2026 improvement over those older crates. and the api can be totally different as long as the end result will be you can do the same "but in a different way" and with security in mind that is kind of what I was thinking and also so we keep it is more lightweight. is this kind of what you proposed?

Memory updated

Yes—that is the architecture I intended: replace the capabilities, not reproduce the existing APIs. Someone should be able to accomplish the same parsing, macro-generation, attribute-processing, and source-transformation tasks with Synir, but through a different, deliberately designed interface.

Your clarification also makes one requirement firmer. I previously wrote “not immediate source compatibility.” I would change that to:

    API and source compatibility with existing crates are not design goals. Capability coverage, correctness, security, and efficiency are.

That removes any suggestion that Synir should eventually grow compatibility wrappers.
A new design, not a recreation

I would not recreate Syn’s types under different names, copy Quote’s interpolation syntax because it is familiar, or mirror Darling’s traits. Those existing tools help identify what users need to accomplish, not dictate how Synir must work.

For example, a macro author might need to read a declaration, validate its attributes, add appropriate generic bounds, and generate an implementation. Synir should provide a coherent workflow for that whole task. Its interfaces could be completely different, provided the resulting implementation is correct and the author retains the necessary control.

A migration guide would therefore explain “how to accomplish this task in Synir,” rather than provide a mechanical search-and-replace conversion.
Lightweight should mean paying for the capabilities you use

This is central to what I proposed, and it fits your original emphasis on optional allocation and shallow-by-default inspection.

I would aim for:
User’s task	What Synir should require
Inspect fields and generate a derive implementation	Structural parsing, relevant attribute validation, and emission—not a fully materialized syntax tree.
Decode a macro’s configuration attributes	The attribute and literal-decoding facilities, without unrelated transformation machinery.
Analyze or rewrite function bodies	Deeper syntax facilities, enabled deliberately.
Parse source without std or an allocator	The portable core with caller-provided storage and explicit capacity limits.

Broad capability across the project does not have to mean a heavyweight configuration for every user.

However, I would not make “the entire implementation must be tiny” a requirement. That can encourage fragile shortcuts. The goal is a small necessary footprint for each workload, with deeper capabilities available when needed.

Crucially, not building a detailed syntax tree must not mean guessing where a field ends. Synir should still recognize enough grammar to establish correct boundaries.
Security should shape the API itself

The improvements should go beyond a zero-dependency manifest. I would design the normal, convenient path so that:

Incomplete inspection is not confused with successful validation. A header-only view and a fully checked derive input should express different guarantees.

Malformed configuration produces an error. An invalid attribute should not silently become a flag, disappear, or trigger a default.

Parsing and generation have explicit resource limits. Limits should cover repeated work and output expansion, not just input size.

Code generation distinguishes data from syntax. Inserting a string value should produce a string literal; interpreting text as executable syntax should require an explicit operation.

These are design goals—not a claim that Synir would automatically be more secure simply because it is newer or self-contained. That advantage would need to be demonstrated through implementation, testing, review, and measurement.
The initial scope is a starting point, not the ceiling

My recommendation to implement derives and declaration-oriented macros first was a delivery sequence, not a recommendation to permanently abandon the broader capabilities.

The intended destination is a comprehensive toolkit whose common use cases stay lightweight. Full syntax inspection, custom macro languages, traversal, and transformations can use different interfaces and representations without being excluded from the project.

So the project definition I would use is:

    Synir is a clean-slate Rust syntax and code-generation toolkit, with zero third-party crate dependencies and a no_std-capable core. It aims to cover the practical capabilities of the existing parsing and macro-tooling stack through a unified API designed around explicit validation, bounded resource use, and lightweight configurations. Compatibility with existing APIs is intentionally out of scope.

That is your idea as I understand it—and yes, it is the direction of the proposed architecture.
