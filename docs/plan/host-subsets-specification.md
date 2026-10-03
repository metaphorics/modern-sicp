# Real host-language editions

Status: Approved by the user's explicit response, "I approve all of them."

Anchor: Christopher Alexander, Tony Hoare, Richard Feynman

## 1. Objective

Remove Scheme from the current repository and the generated books. Each edition must teach its own language throughout. Chapters 4 and 5 must interpret and compile real, documented subsets of Rust, OCaml, TypeScript, and Kotlin.

The user has approved these scope decisions:

- Replace the shared object language with four real host-language subsets.
- Match supported syntax and core behavior to the real host toolchain.
- Reject unsupported features with explicit diagnostics.
- Present lazy evaluation and nondeterministic evaluation as named experimental extensions.
- Remove archived Scheme text from the current tree after replacing its consumers.
- Preserve attribution, licenses, useful figures, and Git history.
- Audit the migration surface first. Audit the entire repository afterward.

### Completion contract

Keep every chapter, section number, exercise number, and instructional objective. Rewrite exercises whose premise requires the removed language. Keep the exercise map consistent with statements, scaffolds, solutions, and rationales.

Provide an edition-specific source grammar, typed syntax representation, type rules, evaluator, analyzer, explicit-control evaluator, and compiler. Share one syntax contract between these consumers within each edition. Do not force the four editions into one source syntax or one value model.

The core languages must support the constructs needed by the translated teaching programs: literals, arithmetic, comparisons, bindings, functions, recursion, closures, conditionals, sequencing, structured data, and the applicable mutation mechanisms. An edition must not claim support for a feature it cannot type-check and execute correctly. Each grammar must enumerate its supported constructs and exclusions before implementation of that edition begins.

Every core execution path must require successful source type checking. Accepted core programs must also compile under the pinned native toolchain in the documented subset context. Distinguish host-invalid syntax or types from host-valid but unsupported constructs. A dynamic internal value representation must not permit an invalid source program to execute.

The Rust grammar must enumerate supported ownership and closure-capture forms. Enforce moves, borrow exclusivity, and relevant lifetimes for each admitted form, or reject that form as unsupported. Runtime reference counting must not grant copies or aliases that Rust forbids. The OCaml contract must enforce its own recursive-binding and mutable-reference rules.

For TypeScript, require acceptance by the pinned TypeScript checker under the edition's compiler options. Compare execution of the checked JavaScript with each teaching engine. JavaScript runtime acceptance alone does not prove TypeScript source validity.

For Kotlin, specify admitted numeric types, literal inference, result types, conversions, overflow, division, Boolean conditions, and nullable operations. Enforce these distinctions before execution. Do not inherit one shared numeric model from the old runtime.

For each edition, specify lexical scope, recursive bindings, captured-binding lifetime, and permitted mutation. Closures must observe changes to captured mutable bindings where the host does. Reject reassignment of immutable bindings. Verify escaping closures in interpreted and compiled paths.

Keep quotation and symbolic-computation lessons through explicit syntax trees and domain values. Do not retain quoted legacy source under another name. Preserve self-interpretation where the chapter teaches it: an evaluator must run translated evaluator code, not delegate that exercise to the host's eval facility.

For self-interpretation, the translated evaluator must itself be valid guest source. Parse and type-check that source, execute it with the teaching evaluator on a translated guest program, and compare the result with direct execution. Its recursive data, pattern matching, functions, and closure facilities must fall inside the declared subset. A call to the host evaluator, reflection, or a host eval primitive does not satisfy this requirement.

Keep the query-language and register-machine lessons. Express their programs through host-language data constructors or a clearly specified non-Scheme notation. They are domain languages, not claims of built-in host-language features. Preserve search, unification, instruction execution, stack discipline, collection, and compilation behavior.

Remove legacy parsers, primitive names, source strings, runtime aliases, fixtures, corpus runners, and build dependencies after their consumers move. A parser that lowers modern punctuation into the old dynamic language without matching host semantics does not satisfy this contract.

The final current tree must contain no Scheme teaching text or executable material, including embedded strings, figure labels, archived prose, solution prose, and generated deliverables. Preserve license text and legal attribution verbatim where required. Do not rewrite Git history or delete published releases.

### Non-goals

Do not implement four complete production compilers. Excluded modules, libraries, macros, advanced type features, or ownership forms must be stated rather than silently approximated. Do not change language/tool versions without a migration requirement. Do not redesign unrelated chapters or the site.

## 2. Commands

The following are existing gates, observed in the edition justfiles and CI. Run them from the stated working directory. The orchestrator owns execution; implementation workers must not run gates or formatters.

### Repository root

```sh
just check
just test
uv run --project tools python tools/exercise_map_check.py --root .
just books
```

For book commands, put `/home/alpha/.cache/modern-sicp/opt/bin` first on PATH and set `MODERN_SICP_PREFIX=/home/alpha/.cache/modern-sicp/opt`. The current default shell resolves Texinfo 7.2; the book contract requires 7.3. Never build the same edition concurrently.

`just check` called the legacy corpus checker before the migration. The migration replaced it: the root recipes run the edition gates, and `test-conformance` is the host-subset conformance gate under `just test`. The old archive parity command is removed.

### Rust: rust/

```sh
cargo +1.98.1 fmt --all -- --check
cargo +1.98.1 clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo +1.98.1 nextest run --workspace --locked
cargo +1.98.1 test --doc --workspace --locked
```

Unset `CARGO_BUILD_BUILD_DIR` and set `RUSTC_WRAPPER=` when the local environment needs the repository's documented gate setup.

### OCaml: ocaml/

```sh
opam exec --switch=5.5.1 -- dune build @check
opam exec --switch=5.5.1 -- dune runtest --force
opam exec --switch=5.5.1 -- dune build @fmt
```

### TypeScript: typescript/

```sh
just fmt
just lint
just test
```

These recipes select the repository's Node installation. The lint recipe runs Biome and recursive TypeScript checking. The test recipe runs Vitest.

### Kotlin: kotlin/

```sh
./gradlew --console=plain ktlintCheck build
./gradlew --console=plain test examplesTest solutionsTest --rerun-tasks
```

### Shared tools: repository root

```sh
just check-tools
just test-tools
```

The ordered implementation plan must add exact native-oracle and interpreter/compiler smoke commands for each new grammar. A command named here as required is not a claim that it has passed.

## 3. Project structure

Keep edition-owned code under its existing edition root.

| Owner | Existing surface |
|---|---|
| Rust | rust/crates/sicp-runtime, rust/crates/ch04, rust/crates/ch05, rust/exercises, rust/solutions, rust/book |
| OCaml | ocaml/common, ocaml/examples, ocaml/exercises, ocaml/solutions, ocaml/test, ocaml/book |
| TypeScript | typescript/packages, typescript/examples, typescript/exercises, typescript/solutions, typescript/book |
| Kotlin | kotlin/ch4, kotlin/ch5, kotlin/examples, kotlin/exercises, kotlin/solutions, kotlin/book |
| Shared contracts | spec/, docs/exercise-map.md, docs/decisions/, docs/plan/ |
| Build and QC | tools/, edition justfiles, root justfile, .github/workflows/ci.yml |
| Assets and publication | text/assets/, text/original/figures/, edition figures, site/, NOTICE.md, LICENSE files |

Replace `spec/scheme-subset` with edition-specific contracts and corpus sources under `spec/host-subsets/<edition>/`. Share semantic case identifiers where they represent the same lesson. Do not require byte-identical source or diagnostics across different host languages.

Remove `text/original/sicp-pocket.texi` and the old `text/split` prose after replacing their consumers. Keep useful figure files and update any labels or source paths that retain the removed language. Do not remove a directory wholesale merely because it contains obsolete prose.

The migration record belongs under docs/plan/ after approval. Execution briefs, review packages, and the progress ledger belong under .outline/sdd/. Preserve the prior campaign ledger; start a distinct migration section with new acceptance evidence.

Retire the old plan's active shared-language requirements when the replacement lands. Preserve the prior campaign record in Git and the execution ledger. No obsolete plan clause may govern the new implementation.

## 4. Code style

Follow the current language-specific layout. Keep OCaml interfaces paired with implementations. Use explicit Rust enums, TypeScript discriminated unions, Kotlin sealed types, and OCaml variants for syntax and errors. Keep token locations for diagnostics. Do not expose parser internals to evaluator callers.

The following existing Rust method shows the project's direct naming and explicit arithmetic style. Source: rust/crates/sicp-runtime/src/random.rs.

```rust
#[must_use]
pub fn next_u64(&mut self) -> u64 {
    self.state ^= self.state >> 12;
    self.state ^= self.state << 25;
    self.state ^= self.state >> 27;
    self.state.wrapping_mul(0x2545_F491_4F6C_DD1D)
}
```

Use existing dependencies or the standard library before adding a package. A new dependency requires a demonstrated need and user approval. Do not introduce compatibility aliases for the removed runtime. Keep public contracts documented; remove stale implementation commentary as the corresponding code changes.

## 5. Testing strategy

### Approved verification seams

Use the existing reader/run and compiler/run entry points after migrating their signatures and callers. Tests must submit source programs and inspect results, effects, or diagnostic categories. Do not test copied implementation text.

For each supported core program, compare these executions:

- The pinned native host compiler/runtime.
- The edition's direct evaluator and analyzer.
- The explicit-control evaluator.
- The compiled program on the teaching machine.

Compare observable values and effect order. Do not equate internal stack counts across different implementations. Fix native evaluation choices where the host specifies them. Avoid tests that invent an order where the host leaves it unspecified.

Define a reproducible source wrapper and native compile/run command for each edition. Use the same declared types, imports, inputs, and observation protocol. Record native compiler acceptance separately from runtime output. Reject host-invalid programs before any guest effect occurs; matching a late runtime error is not typing agreement. Report host-valid excluded constructs as unsupported. Do not compare unstable native diagnostic wording.

Check invalid syntax, type errors, unsupported constructs, lexical shadowing, closure capture, recursion, mutable aliasing, short-circuit evaluation, numeric boundaries, and nested compiled calls that can overwrite live registers. Use generated well-typed programs for cross-engine agreement where the existing property framework supports this.

For experimental lazy and search semantics, use independent finite reference models and behavioral invariants. Give each experiment a named execution mode or separate construct, with explicit source-admission rules. The default core must retain host evaluation and effect semantics. Do not submit experimental constructs to the native oracle as built-in host features.

Keep exercise coverage through the existing map checker. Replace source-archive parity with stable section, exercise, figure, and reference inventories. Derive this inventory from the current source before removing it; do not hand-type expected omissions.

Record provenance for every expected fixture: native execution, independently checked reference model, or source transcript. Existing fixtures have mixed provenance; do not treat a manifest's blanket generation claim as proof. Native core expectations must be regenerated from checked host programs. Experimental expectations require an independent model or reviewed derivation.

Build HTML, EPUB, and PDF for every edition. Run asset, mathematics, and EPUB validation. Inspect representative rendered pages, including chapters 4 and 5, and verify that the generated artifacts contain no legacy source. Search cannot substitute for behavioral conformance.

Repair the observed Rust exercise 3.46 trial-state defect before claiming the baseline is green. Each trial must own a fresh cell. A required race witness must use a controlled interleaving, not a guarantee that a scheduler will expose a race.

## 6. Boundaries

### Always

Preserve learning objectives, numbering, attribution, and useful assets. Validate source input at parser and type-checker boundaries. Migrate all callers in the same accepted change. Record actual gate exits and smoke results. Keep one integration owner for shared files. Use fresh implementation workers and independent reviewers with explicit agent-tier selection.

The available task tool exposes agent tiers, not arbitrary model selection. Record the selected tier in each brief; do not claim a model override that the tool cannot enforce.

### Ask first

Obtain approval of this specification and its test seams. Obtain approval of the subsequent ordered plan and task list. Ask before adding dependencies, changing a public contract outside the approved language cutover, or discarding any further data. CI changes that replace the removed runtime dependency must be enumerated in the implementation plan.

### Never

Do not preserve the old language behind renamed types, adapters, hidden fixtures, or skipped checks. Do not delete a lesson to make the migration smaller. Do not change native semantics and call the result a host subset. Do not remove legal notices, rewrite Git history, delete published releases, or deploy a partially migrated edition. Do not report all-green while a gate fails or has not run.
