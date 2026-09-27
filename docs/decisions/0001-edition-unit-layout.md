# 0001: How the build units reach `examples/`, `exercises/`, and `solutions/`

Date: 2026-09-22. Status: settled at A2.

## Question

The plan (D7, the repository layout, the rules across the four roots) puts `book/`, `examples/`, `exercises/`, and `solutions/` directly under each language root, with one subdirectory per chapter (`examples/ch4/`, `examples/ch0/`). The companions and the scaffold contract sketch the per-chapter build units (`crates/chNN`, `packages/chN`, Gradle subprojects, dune libraries) and name paths inside those units. The plan wins; this file records how each build system reaches the root-level directories.

## Decision

The three code directories live at the language root, one subdirectory per chapter. The build units live beside them and declare their targets by path:

|Edition|Chapter unit|How it reaches the root directories|
|---|---|---|
|Rust|`rust/crates/chNN/Cargo.toml` with `autoexamples = false`, `autotests = false`|`[[example]] path = "../../examples/chNN/<listing>.rs"`; `[[test]] name = "scaffold_sec_X_Y" path = "../../exercises/chNN/sec_X_Y.rs"`; `[[test]] name = "solutions_sec_X_Y" path = "../../solutions/chNN/sec_X_Y.rs"`; the crate's `src/lib.rs` holds the section modules the examples and solutions share|
|OCaml|`ocaml/examples/chN/dune` defines library `sicp_chN` (the chapter's listings are the chapter library) with its Alcotest runner; `ocaml/exercises/chN/dune` defines `sicp_chN_exercises` and the scaffold executables under `(alias (name scaffold))`; `ocaml/solutions/chN/dune` defines `sicp_chN_solutions` with its tests|dune builds any directory tree; `ocaml/common/` is `sicp_common`; `ocaml/appendix/chN/` holds `sicp_chN_base` and `sicp_chN_core`|
|TypeScript|`typescript/packages/chN/` holds `package.json`, `tsconfig.json`, `vitest.config.ts`|`tsconfig.json` sets `rootDir: "../.."` and includes `../../examples/chN`, `../../exercises/chN`, `../../solutions/chN`; `vitest.config.ts` names the project `chN` and includes the same three directories; the root `vitest.config.ts` lists `packages/*` under `test.projects`|
|Kotlin|`kotlin/chN/build.gradle.kts`|source sets `examples`, `exercises`, `solutions` with `kotlin.srcDir("../examples/chN")` and so on; `examplesTest` and `solutionsTest` run under `check`; `exercisesTest` compiles under `check` and runs only from `just scaffold`|

Chapter directories are zero-padded in Rust (`ch00` to `ch05`, matching the crate names) and plain in the other three (`ch0` to `ch5`). Runtime code lives in `rust/crates/sicp-runtime/`, `ocaml/common/`, `typescript/packages/runtime/` (created when 4.1 needs it), and `kotlin/runtime/`.

File names inside the chapter directories: Rust and OCaml group by section (`sec_1_1.rs`, `sec_1_1.ml`) with one test per exercise named `ex_1_01`; TypeScript and Kotlin have one file per exercise (`ex_1_01.ts`, `E1_01.kt`). Every solution has a rationale `ex_N_MM.md` beside it.

## The shared `random`

Every runtime ships the seeded `random` of D31 as xorshift64* (Vigna 2016): `x ^= x >> 12; x ^= x << 25; x ^= x >> 27; return x * 0x2545F4914F6CDD1D` over unsigned 64-bit words, seed nonzero. `random(n)` for `n > 0` is `next() mod n`. From seed 1 the first five words are 5180492295206395165, 12380297144915551517, 13389498078930870103, 5599127315341312413, 1036278371763004928, so `random(1000)` yields 165, 517, 103, 413, 928. Every edition asserts this vector, which makes the four editions' Monte Carlo sections agree.
