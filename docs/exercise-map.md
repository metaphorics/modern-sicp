# Exercise map

Grounded on 2026-09-22 from `sicp-pocket.texi`. Every one of the 356 SICP exercises has one row per chapter table below; the five chapter tables were classified by five independent readers and are joined here without edits. `tools/exercise_map_check.py` parses this file.

## Policy

- Classes per language: `T` translate (syntax changes only); `A: reason` adapt (the idea survives, the statement changes because the host language differs); `R: topic` replace (the exercise is about Scheme itself; a language-tailored exercise under the same number replaces it). The chosen class is binding for the unit that writes the section.
- Numbering is 1:1 with SICP in every edition. An `R` row keeps its number. A tailored addition is numbered `N.Ma` and sits right after `N.M`. At most one addition per section per edition; the unit picks it from the `Tailored addition idea` column at unit start, or picks none, and records the pick by appending `chosen: <lang>` to the cell. Ideas not picked stay ideas.
- Reader-facing signposts are fixed sentences. An `R` statement opens with `@emph{SICP exercise N.M is about Scheme itself, so this edition replaces it; the exercise below keeps the number and teaches the same section material.}` An addition opens with `@emph{Exercise N.Ma is added by this edition and extends exercise N.M; SICP numbers stop at N.M.}`
- Every exercise has three artifacts per edition: the statement in `book/` (the section's exercise block), a pending scaffold in `exercises/` (signature plus a test carrying the language's pending marker), and a reference solution with passing tests and a rationale in `solutions/`. Prose-only exercises (a diagram, a proof, a discussion) are marked `prose` in the topic cell by the unit and get a Markdown answer instead of tests.
- Chapter 0 exercises (0.1 onward) are original per edition and are listed in each companion, not here.

## Overrides recorded at integration

These override the cells they name; the chapter tables are otherwise binding.

- 2.5 (and every row whose out-of-library table names `num-bigint` or `zarith`): Rust uses checked `u128`, OCaml uses 63-bit `int`; the statement states the overflow bound. TypeScript uses `bigint`, Kotlin uses `java.math.BigInteger`. No bignum library in any edition (D14, D17).
- 2.86: Rust uses a trait defined in the section, not `num-traits`.
- 2.74, 2.75 (TypeScript): plain discriminated unions, no `Schema`; chapters 1 and 2 import no Effect module (Effect enters at 3.1).
- 5.50, 5.52 (all editions): the shared corpus ships `spec/scheme-subset/programs/metacircular.scm`, the chapter 4 evaluator written in the object language, so "compile the metacircular evaluator" keeps its meaning and both rows stay `A`.
- Chapter 4 rows 4.11, 4.12, 4.13, 4.27, 4.44, 4.45, 4.46, 4.50, 4.51, 4.60, 4.61, 4.62, 4.63, 4.69 were classified from surrounding text; the unit that writes each section reads the statement and confirms or corrects the class in the same commit.
- Row counts: chapter 1 has 46, chapter 2 has 97, chapter 3 has 82, chapter 4 has 79, chapter 5 has 52; total 356. `exercise_map_check.py` fails when this file's unique row count differs.

## Chapter 1 exercise classification (1.1 to 1.46)

Legend: `T` = translate, syntax changes only. `A: reason` = adapt, the statement changes because the host language differs. `R: topic` = replace, the exercise is about Scheme itself; the replacement extends the same number. Tailored additions are numbered as the row plus a letter; empty cells mean no addition.

### Section 1.1 (lines 1107 to 2629, 8 exercises)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 1.1 | evaluate expression sequence in order | A: no REPL, print in main | T | T | T | |
| 1.2 | prefix notation translation | R: write as pure nested calls | R: write as pure nested calls | R: write as pure nested calls | R: write as pure nested calls | |
| 1.3 | sum squares of two larger | T | T | T | T | |
| 1.4 | operator chosen by conditional | A: branch yields operator function | T | A: operator picked via ternary | A: operator picked via reference | |
| 1.5 | evaluation order termination test | T | T | T | T | |
| 1.6 | conditional as ordinary procedure | T | T | T | T | |
| 1.7 | tolerance fails at extremes | T | T | T | T | |
| 1.8 | cube root by Newton | T | T | T | T | |

Counts, summing to 8 per language: Rust 5 T / 2 A / 1 R; OCaml 7 / 0 / 1; TypeScript 6 / 1 / 1; Kotlin 6 / 1 / 1.

### Section 1.2 (lines 2630 to 4191, 20 exercises)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 1.9 | substitution model, recursive vs iterative | A: no tail-call guarantee | T | A: no tail-call guarantee | T | 1.9a: measure stack depth until overflow, compare versions (chosen: rust) |
| 1.10 | Ackermann function values | T | T | T | T | |
| 1.11 | recursive and iterative function | A: iterative form as loop | T | A: iterative form as loop | T | |
| 1.12 | Pascal triangle recursion | T | T | T | T | |
| 1.13 | Fibonacci induction proof | T | T | T | T | |
| 1.14 | count-change tree and growth | T | T | T | T | 1.14a: check predicted growth with doubling measurements (chosen: ocaml) |
| 1.15 | sine reduction steps, growth | T | T | T | T | |
| 1.16 | invariant-based fast exponentiation | A: loop replaces tail recursion | T | A: loop replaces tail recursion | T | |
| 1.17 | fast multiplication, double halve | T | T | T | T | |
| 1.18 | iterative Russian peasant multiplication | A: loop replaces tail recursion | T | A: loop replaces tail recursion | T | 1.18a: verify loop against built-in multiplication |
| 1.19 | transform squaring Fibonacci | A: integer width forces choice | A: 63-bit int overflows fast | A: number precision, use bigint | A: Long overflow, use BigInteger | 1.19a: probe overflow boundary of native integers (chosen: kotlin) |
| 1.20 | normal order remainder count | R: count remainder calls in traced eager gcd | R: count remainder calls in traced eager gcd | R: count remainder calls in traced eager gcd | R: count remainder calls in traced eager gcd | |
| 1.21 | smallest divisors of three numbers | T | T | T | T | |
| 1.22 | timed prime search ranges | A: Instant replaces runtime | A: Sys.time replaces runtime | A: performance.now replaces runtime | A: System.nanoTime replaces runtime | 1.22a: time with warmup, report median timings (chosen: typescript) |
| 1.23 | skip even divisors, remeasure | A: clock API as in 1.22 | A: clock API as in 1.22 | A: clock API as in 1.22 | A: clock API as in 1.22 | |
| 1.24 | timed Fermat test | A: rand crate plus Instant | A: Random module plus Sys.time | A: Math.random plus performance.now | A: Random plus System.nanoTime | |
| 1.25 | naive expmod critique | A: i128 overflows, debug panics | A: 63-bit overflow wraps silently | A: f64 precision, bigint slow | A: Long wraps, BigInteger slow | 1.25a: compare overflow behavior across build modes |
| 1.26 | double recursion growth blunder | T | T | T | T | |
| 1.27 | Carmichael numbers fool Fermat | T | T | T | T | |
| 1.28 | Miller-Rabin nontrivial root | T | T | T | T | |

Counts, summing to 20 per language: Rust 10 T / 9 A / 1 R; OCaml 15 / 4 / 1; TypeScript 10 / 9 / 1; Kotlin 15 / 4 / 1.

### Section 1.3 (lines 4192 to 5722, 18 exercises)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 1.29 | Simpson rule integration | T | T | T | T | |
| 1.30 | iterative sum fill-in | A: iterative loop, no fill-in | T | A: iterative loop, no fill-in | T | |
| 1.31 | product abstraction, Wallis formula | A: iterative variant as loop | T | A: iterative variant as loop | T | |
| 1.32 | accumulate general combiner | A: iterative variant as loop | T | A: iterative variant as loop | T | |
| 1.33 | filtered accumulate with predicate | T | T | T | T | 1.33a: re-express filter and accumulate with fold (chosen: rust) |
| 1.34 | apply procedure to itself | A: type checker rejects it | A: type checker rejects it | A: type checker rejects it | A: type checker rejects it | 1.34a: compare compile rejection with runtime failure (chosen: typescript) |
| 1.35 | golden ratio fixed point | T | T | T | T | |
| 1.36 | printed fixed-point iterations | T | T | T | T | |
| 1.37 | continued fraction, both processes | A: iterative variant as loop | T | A: iterative variant as loop | T | 1.37a: find k for ten digits, note float limits |
| 1.38 | Euler e expansion | T | T | T | T | |
| 1.39 | tangent continued fraction | T | T | T | T | |
| 1.40 | cubic for Newton's method | T | T | T | T | 1.40a: solve specified equations end to end |
| 1.41 | double combinator puzzle | T | T | T | T | |
| 1.42 | function composition | T | T | T | T | |
| 1.43 | n-fold repeated application | T | T | T | T | 1.43a: compose by squaring for logarithmic repeated (chosen: kotlin) |
| 1.44 | smoothing and n-fold smoothing | T | T | T | T | 1.44a: denoise synthetic noisy samples |
| 1.45 | n-th roots, repeated damping | T | T | T | T | 1.45a: record convergence failures of under-damped roots |
| 1.46 | iterative improvement abstraction | A: recursive closure needs helper | T | A: loop inside returned function | T | 1.46a: return lazy sequence of successive guesses (chosen: ocaml) |

Counts, summing to 18 per language: Rust 12 T / 6 A / 0 R; OCaml 17 / 1 / 0; TypeScript 12 / 6 / 0; Kotlin 17 / 1 / 0.

Overall over 46 exercises: Rust 27 T / 17 A / 2 R; OCaml 39 / 5 / 2; TypeScript 28 / 16 / 2; Kotlin 38 / 6 / 2.

### Hard spots

- Tail-call honesty dominates section 1.2. The book's claim that an iterative process runs in constant space through ordinary calls holds in OCaml and in Kotlin with `tailrec`. In Rust and TypeScript it does not, so every "iterative process" exercise becomes an explicit loop and the section prose needs a per-language restatement of the recursion versus iteration distinction.
- Integer width flips two lessons. In 1.19 and 1.25 Scheme's unbounded integers make the point about wasteful huge arithmetic; fixed-width hosts turn it into overflow (Rust panics in debug builds and wraps in release; OCaml's 63-bit int wraps silently; TypeScript number loses precision past 2^53 and bigint is exact but slow; Kotlin Long wraps). The statements must be rewritten around this.
- 1.20 cannot be translated because no host has normal-order evaluation; the replacement counts remainder calls in an instrumented eager gcd and can say that thunks or lazy sequences would simulate normal order.
- 1.34 becomes a compile-time discussion in all four languages. The book's runtime error "the object 2 is not applicable" never happens; the statement must ask why the code does not type check.
- Timing exercises (1.22 to 1.24) need per-language clock notes: OCaml `Sys.time` is processor time; `performance.now()` is coarsened to 100 microseconds without cross-origin isolation; JIT warmup in TypeScript and Kotlin; the book's microsecond `runtime` narrative does not carry over.
- Rust has no standard REPL, so 1.1 and all "what does the interpreter print" framing become program output. The 1.46 fix is a named inner `fn` or a loop because a Rust closure cannot refer to itself.
- One line on working defaults: if the interview later requires exercise statements to stay verbatim (no rewrites), every `A` row above moves to `R` and the tailored-addition density roughly doubles; no other default changes this table.

### Out-of-standard-library needs

| Language | Exercises | Need | Evidence |
|---|---|---|---|
| Rust | 1.24, 1.28 | `rand` crate for random picks; `std::time::Instant` covers timing | https://docs.rs/rand/latest/rand/ ; https://doc.rust-lang.org/std/time/struct.Instant.html |
| Rust | 1.19 | `i128` (std) suffices to about Fib(180); larger n needs `num-bigint` | overflow semantics: https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow |
| OCaml | 1.19 | `zarith` for arbitrary precision; stdlib `int` is 63-bit (`Sys.int_size`) | https://opam.ocaml.org/packages/zarith/ ; https://ocaml.org/manual/5.5/api/Sys.html |
| OCaml | 1.22 to 1.24 | none beyond std: `Sys.time`, `Random.int` | https://ocaml.org/manual/5.5/api/Sys.html ; https://ocaml.org/manual/5.5/api/Random.html |
| TypeScript | 1.19 | none: `bigint` is a language primitive | https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt |
| TypeScript | 1.22 to 1.24 | none beyond platform globals: `Math.random`, `performance.now()` | https://developer.mozilla.org/en-US/docs/Web/API/Performance/now |
| Kotlin | 1.19 | `java.math.BigInteger`, part of the JDK but not kotlin-stdlib | https://docs.oracle.com/javase/8/docs/api/java/math/BigInteger.html |
| Kotlin | 1.24 | none beyond std: `kotlin.random.Random` | https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.random/-random/ |
| Kotlin | 1.22 to 1.24 | timing via `System.nanoTime` (JDK); the kotlinlang `measureNanoTime` page fetch failed this session, so that helper is unverified | unverified |

Grounding notes: Kotlin `tailrec` semantics from https://kotlinlang.org/docs/functions.html#tail-recursive-functions (compiler rewrites qualifying self-tail calls to loops). The OCaml guaranteed-tail-call fact is a stated working default; both citation attempts (ocaml.org/docs/tail-recursion, manual tailc page) returned 404, so it is marked unverified as a citation. No Effect v4 APIs are needed anywhere in chapter 1; first use will be streams and fibers in chapter 3.

### Critical Files for Implementation

- `modern-sicp/sicp-pocket.texi` (lines 953-5722): sole source for all chapter 1 statements and listings
- `editions/rust/ch1/` (proposed): Rust chapter 1 modules; hosts the nine A-row loop rewrites and rand usage
- `editions/ocaml/ch1/` (proposed): OCaml chapter 1 modules; near-verbatim, needs Zarith only for 1.19
- `editions/ts/ch1/` (proposed): TypeScript chapter 1 modules; bigint and performance.now adaptations
- `editions/kotlin/ch1/` (proposed): Kotlin chapter 1 modules; tailrec markers and BigInteger for 1.19

## Chapter 2 exercise classification (2.1 to 2.97)

### Verification

- Read Texinfo lines 5724 to 7124, 7731 to 7836, 8352 to 8503, 8652 to 8763, 9378 to 9473, 9584 to 9673, 9769 to 9848, 10145 to 10298, 10332 to 10403, 10603 to 10708, 11113 to 11235, plus a full anchor enumeration (`Exercise 2.N` with ~3 lines of statement each) over lines 5724 to 13969. Every statement anchor 2.1 to 2.97 was seen verbatim; long statements were read in full where the classification hinged on wording (2.29, 2.37, 2.38, 2.42, 2.43, 2.49, 2.52 to 2.58, 2.60, 2.63, 2.64, 2.67 to 2.70).
- Language facts grounded this session: Rust `i64` is 64-bit signed with checked `BITS` (https://doc.rust-lang.org/std/primitive/i64.html), so 2^a·3^b overflows and needs `num-bigint`; ECMAScript `BigInt` is built in (https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt); Kotlin stdlib has no big integers, so `java.math.BigInteger` (https://kotlinlang.org/api/core/kotlin-stdlib/).
- Not read in full this session (grounded on anchor excerpts that match the canonical SICP text verbatim): 2.61 to 2.62, 2.65 to 2.66, 2.71 to 2.72, 2.77 to 2.97 beyond the anchor lines. Classifications for these rest on the dispatch/tower/term-list machinery the excerpts confirm; nothing turned on unread details.

### Criteria

`T` = statement and solution survive with syntax changes. `A` = idea survives, statement changes (representation forced by types, no symbols/quotation, closures as data, printed-list reading). `R` = exercise is about Scheme itself; same-number tailored replacement. Counts are identical across the four languages; only the `A` reason wording differs.

### Section 2.1 (exercises 2.1 to 2.16, 16 rows)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 2.1 | Sign-normalizing make-rat | T | T | T | T | allow mixed-sign make-rat inputs (chosen: rust) |
| 2.2 | Line segments from points | T | T | T | T | |
| 2.3 | Two rectangle representations | T | T | T | T | |
| 2.4 | Procedural cons, car, cdr | A: closures as pairs need function traits | A: closures as pairs need function records | A: closure pairs need function pair type | A: closures as pairs need function types | verify cons identity law property test (chosen: kotlin) |
| 2.5 | Pairs as 2^a 3^b | A: 2^a 3^b overflows i64, needs bigint | A: 2^a 3^b overflows int, needs Z | A: 2^a 3^b overflows number, needs bigint | A: 2^a 3^b overflows Long, needs BigInteger | |
| 2.6 | Church numerals | A: church numerals need function-typed traits | A: church numerals need function encoding | A: church numerals need function-encoded zero | A: church numerals need function-encoded zero | arithmetic on three plus successor |
| 2.7 | Interval selectors | T | T | T | T | |
| 2.8 | Sub-interval | T | T | T | T | |
| 2.9 | Interval width algebra | A: width forces interval invariant decision | A: width forces interval invariant decision | A: width forces interval invariant decision | A: width forces interval invariant decision | forbid zero-width interval construction |
| 2.10 | Divide by zero-spanning interval | T | T | T | T | |
| 2.11 | Nine-case multiplication | T | T | T | T | |
| 2.12 | Center-percent constructor | T | T | T | T | compare endpoint and center-width interval interfaces (chosen: ocaml) |
| 2.13 | Percentage tolerance formula | T | T | T | T | |
| 2.14 | Repeated uncertain variables | T | T | T | T | track interval dependency through formulas |
| 2.15 | Par1 versus par2 | T | T | T | T | |
| 2.16 | Equivalent expressions, different answers | T | T | T | T | |

### Section 2.2 (exercises 2.17 to 2.52, 36 rows)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 2.17 | Last-pair | T | T | T | T | |
| 2.18 | Reverse | T | T | T | T | |
| 2.19 | Currency as list | T | T | T | T | add pound and euro denominations |
| 2.20 | Same-parity variadic | A: variadic rest parameters replace dotted-tail notation | A: variadic rest arguments replace dotted-tail notation | A: rest tuple parameters replace dotted-tail notation | A: vararg parameters replace dotted-tail notation | |
| 2.21 | Square-list two ways | T | T | T | T | |
| 2.22 | Iterative square-list bug | T | T | T | T | |
| 2.23 | For-each | T | T | T | T | implement map via for-each |
| 2.24 | List structure tree | R: draw nested structure in edition syntax | R: draw nested structure in edition syntax | R: draw nested structure in edition syntax | R: draw nested structure in edition syntax | |
| 2.25 | Car and cdr combinations | R: accessor chains over edition tree type | R: accessor chains over edition tree type | R: accessor chains over edition tree type | R: accessor chains over edition tree type | |
| 2.26 | Append results | R: predict collection printing instead | R: predict collection printing instead | R: predict collection printing instead | R: predict collection printing instead | |
| 2.27 | Deep-reverse | A: deep-reverse needs recursive nested-list type | A: deep-reverse needs recursive nested-list type | A: deep-reverse needs recursive nested union type | A: deep-reverse needs recursive sealed type | |
| 2.28 | Fringe leaves | A: fringe needs recursive tree type | A: fringe needs recursive tree type | A: fringe needs recursive tree union | A: fringe needs recursive sealed tree | |
| 2.29 | Binary mobile | A: branch structure needs recursive variant type | A: branch structure needs recursive variant type | A: branch structure needs recursive union type | A: branch structure needs recursive sealed type | balance test with torque tolerance |
| 2.30 | Square-tree | A: recursive tree type replaces nested lists | A: recursive tree type replaces nested lists | A: recursive tree union replaces nested lists | A: recursive sealed tree replaces nested lists | |
| 2.31 | Tree-map | A: generic tree map needs recursive type | A: generic tree map needs recursive type | A: generic tree map needs recursive union | A: generic tree map needs sealed tree | |
| 2.32 | Subsets | T | T | T | T | subsets via bit masks |
| 2.33 | Accumulate-based map, filter, append | T | T | T | T | |
| 2.34 | Horner evaluation | T | T | T | T | |
| 2.35 | Count-leaves accumulation | A: count-leaves over recursive tree type | A: count-leaves over recursive tree type | A: count-leaves over recursive tree union | A: count-leaves over recursive sealed tree | |
| 2.36 | Accumulate-n | T | T | T | T | |
| 2.37 | Matrix operations | T | T | T | T | static shapes for matrix operations |
| 2.38 | Fold-left versus fold-right | T | T | T | T | |
| 2.39 | Reverse via folds | T | T | T | T | |
| 2.40 | Unique-pairs | T | T | T | T | |
| 2.41 | Ordered triples summing | T | T | T | T | |
| 2.42 | Eight queens | T | T | T | T | column-index positions instead of lists |
| 2.43 | Swapped mapping order | T | T | T | T | |
| 2.44 | Up-split | A: painter is a boxed frame-to-frame function | A: painter is a first-class function | A: painter needs function type, adapts | A: painter needs function type, adapts | |
| 2.45 | Split combinator | A: split returns generic higher-order constructor | A: split returns polymorphic combinator | A: split returns generic higher-order function | A: split returns generic higher-order function | |
| 2.46 | Vector abstraction | T | T | T | T | vectors as data class or tuple chosen: kotlin |
| 2.47 | Frame constructors | T | T | T | T | |
| 2.48 | Segment from vectors | T | T | T | T | |
| 2.49 | Primitive painters | T | T | T | T | painter output as SVG strings |
| 2.50 | Flip and rotations | T | T | T | T | |
| 2.51 | Below two ways | A: below needs painter function, two constructions | A: below needs painter function, adapts | A: below needs painter function type | A: below needs painter function type | below via rotate composition check (chosen: rust) |
| 2.52 | Square limit variations | T | T | T | T | euler square-limit variant chosen: ocaml |

### Section 2.3 (exercises 2.53 to 2.72, 20 rows)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 2.53 | Interpreter printing | R: predict printed union values instead | R: predict printed variant values instead | R: predict printed union values instead | R: predict printed sealed values instead | |
| 2.54 | Equal? recursively | A: recursive equality over symbol-leaf trees | A: recursive equality over variant trees | A: recursive equality over nested unions | A: recursive equality over sealed trees | |
| 2.55 | Double quote | R: explain quote in edition AST terms | R: explain quote in edition AST terms | R: explain quote in edition AST terms | R: explain quote in edition AST terms | nested quote pair evaluation |
| 2.56 | Exponentiation rule | A: exponentiation case added to expression enum | A: exponentiation variant added to expression type | A: exponentiation case added to union | A: exponentiation case added to sealed hierarchy | |
| 2.57 | N-ary sums products | A: n-ary sums products change enum shapes | A: n-ary sums products change variants | A: n-ary sums products widen unions | A: n-ary sums products widen constructors | prefix sums as variadic union |
| 2.58 | Infix notation | A: infix parsing changes expression constructors | A: infix notation needs new variant layer | A: infix notation needs parser into union | A: infix notation needs parser into hierarchy | |
| 2.59 | Union-set unordered | T | T | T | T | set union via Set operations (chosen: rust) |
| 2.60 | Duplicate-allowed sets | T | T | T | T | |
| 2.61 | Adjoin ordered | T | T | T | T | |
| 2.62 | Ordered union | T | T | T | T | |
| 2.63 | Tree-to-list comparison | T | T | T | T | |
| 2.64 | List-to-tree | A: partial-tree returns pair, needs tuple | A: partial-tree returns pair, needs tuple | A: partial-tree returns pair, needs tuple | A: partial-tree returns pair, needs Pair | |
| 2.65 | Tree set operations | T | T | T | T | tree sets as balanced maps (chosen: kotlin) |
| 2.66 | Tree lookup | T | T | T | T | |
| 2.67 | Sample decode | T | T | T | T | decode via pattern match |
| 2.68 | Encode-symbol | T | T | T | T | |
| 2.69 | Successive merge | T | T | T | T | |
| 2.70 | Rock song encoding | T | T | T | T | bit savings assertion test (chosen: ocaml) |
| 2.71 | Skewed tree shape | T | T | T | T | |
| 2.72 | Encode order of growth | T | T | T | T | |

### Section 2.4 (exercises 2.73 to 2.76, 4 rows)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 2.73 | Data-directed deriv | A: enum for expression variants replaces type tags | A: variant types replace tags and dispatch tables | A: discriminated union replaces type tags | A: sealed hierarchy replaces type tags | add atan rule via new variant (chosen: rust) |
| 2.74 | Division records | A: division record schemas need per-division adapters | A: first-class modules model division-specific records | A: Schema variants model division record formats | A: sealed interfaces model division record formats | add salary-records division with schema (chosen: ocaml) |
| 2.75 | Message-passing make-from-mag-ang | A: message dispatch needs closure objects per instance | A: message passing natural with closures | A: message dispatch needs closure returning dispatch | A: message dispatch needs function types per instance | |
| 2.76 | Adding types versus operations | A: compare trait objects, enums, visitor dispatch | A: compare variants, functors, first-class modules | A: compare unions, Schema tags, method objects | A: compare sealed hierarchies, extension dispatch, interfaces | add type via module extension (chosen: kotlin) |

### Section 2.5 (exercises 2.77 to 2.97, 21 rows)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 2.77 | Nested apply-generic | A: nested tagged values need layered enums | A: nested variants need constructor-level dispatch | A: nested tagged values need layered unions | A: nested tagged values need layered sealed types | |
| 2.78 | Primitive type tags | A: untagged numbers need enum with plain variant | A: plain number variant replaces attached-tag hack | A: untagged numbers need union with bare case | A: bare-number case in sealed hierarchy | |
| 2.79 | Generic equ? | T | T | T | T | equ? across tower levels |
| 2.80 | Generic =zero? | T | T | T | T | |
| 2.81 | Self-coercion | A: coercion table needs heterogeneous entry types | A: coercion table uses hash on type keys | A: coercion map keyed by type tuples | A: coercion map keyed by KClass pairs | |
| 2.82 | Multi-argument coercion | A: multi-arg coercion needs conversion search | A: multi-arg coercion needs conversion search | A: multi-arg coercion needs conversion search | A: multi-arg coercion needs conversion search | coerce to common supertype example |
| 2.83 | Raise operation | A: raise needs tower enum with levels | A: raise over variant tower levels | A: raise over union tower levels | A: raise over sealed tower levels | |
| 2.84 | Successive raising | A: successive raising needs tower ordering map | A: tower ordering via raise chain search | A: tower ordering via raise chain search | A: tower ordering via raise chain search | |
| 2.85 | Drop projection | A: drop needs project plus equ? dispatch | A: lowering needs project and equality dispatch | A: lowering needs project and equality dispatch | A: lowering needs project and equality dispatch | simplify complex zero to real (chosen: ocaml — implemented as 2.85a, coercion-graph cycle detection, not the literal "simplify complex zero to real" text; see report) |
| 2.86 | Generic complex parts | A: generic coefficients force numeric abstraction layer | A: generic coefficients need numeric module signatures | A: generic coefficients need numeric abstraction union | A: generic coefficients need numeric abstraction interface | |
| 2.87 | Polynomial =zero? | T | T | T | T | test =zero? on nested polys |
| 2.88 | Polynomial subtraction | A: negation and subtraction as generic ops | A: negation and subtraction as generic ops | A: negation and subtraction as generic ops | A: negation and subtraction as generic ops | |
| 2.89 | Dense term lists | A: dense term lists as Vec with invariants | A: dense term lists as int list | A: dense term lists as readonly arrays | A: dense term lists as PersistentList | |
| 2.90 | Sparse and dense | A: two term-list packages behind traits | A: two term-list modules behind signatures | A: two term-list providers behind interface | A: two term-list providers behind interface | benchmark sparse versus dense multiply (chosen: rust) |
| 2.91 | Polynomial division | A: division needs generic sub mul on terms | A: polynomial division via generic operations | A: polynomial division via generic operations | A: polynomial division via generic operations | |
| 2.92 | Multi-variable polynomials | A: variable ordering needs symbolic comparison keys | A: variable ordering needs symbolic comparison keys | A: variable ordering needs symbolic comparison keys | A: variable ordering needs symbolic comparison keys | |
| 2.93 | Generic rational functions | A: rational of generic values needs trait bounds | A: rational over generic coefficients needs signatures | A: rational over union coefficients | A: rational over generic coefficients interface | reduce rational functions lazily (chosen: kotlin — implemented as 2.93a, `LazyRatF` defers reduction to first access via `by lazy`) |
| 2.94 | Polynomial gcd | A: gcd needs division loop over term lists | A: gcd-terms via Euclid on term lists | A: gcd-terms via Euclid on term lists | A: gcd-terms via Euclid on term lists | |
| 2.95 | Integer arithmetic factors | T | T | T | T | factor P2 coefficients explanation |
| 2.96 | Pseudoremainder | A: integer division on coefficients needs generic div | A: pseudoremainder scales coefficients generically | A: pseudoremainder scales coefficients generically | A: pseudoremainder scales coefficients generically | |
| 2.97 | Reduce-terms | A: reduce-terms plus package-wide generic rewire | A: reduce-terms plus system-wide install | A: reduce-terms plus system-wide install | A: reduce-terms plus system-wide install | drop gcd-terms after reduce test |

### Counts (rows sum to 97 per language)

| Section | Rust T/A/R | OCaml T/A/R | TypeScript T/A/R | Kotlin T/A/R | Rows |
|---|---|---|---|---|---|
| 2.1 | 11 / 4 / 1 | 11 / 4 / 1 | 11 / 4 / 1 | 11 / 4 / 1 | 16 |
| 2.2 | 23 / 10 / 3 | 23 / 10 / 3 | 23 / 10 / 3 | 23 / 10 / 3 | 36 |
| 2.3 | 13 / 5 / 2 | 13 / 5 / 2 | 13 / 5 / 2 | 13 / 5 / 2 | 20 |
| 2.4 | 0 / 4 / 0 | 0 / 4 / 0 | 0 / 4 / 0 | 0 / 4 / 0 | 4 |
| 2.5 | 4 / 17 / 0 | 4 / 17 / 0 | 4 / 17 / 0 | 4 / 17 / 0 | 21 |
| **Total** | **51 / 40 / 6** | **51 / 40 / 6** | **51 / 40 / 6** | **51 / 40 / 6** | **97** |

31 tailored addition ideas appear (about one per three exercises).

### Hard spots

- Tagged-data machinery (2.73 onward, and the 2.4/2.5 text it builds on): `attach-tag`, `type-tag`, `contents`, and the two-dimensional `apply-generic` table must be re-expressed. Tags become enum variants / OCaml constructors / discriminated unions / sealed subclasses; the table is keyed by tag tuples; `put`/`get` becomes a map in each edition. Exercises 2.77 (nested tags), 2.78 (untagged base case), 2.81 to 2.85 (coercion and tower) all sit on this one redesign and must share it.
- Painter language (2.44 to 2.52): painters are first-class frame-to-frame closures. OCaml is native; Rust needs boxed closures or a `Painter` trait; TypeScript needs an explicit function type; Kotlin needs function types. `transform-painter` returning a painter must work in all four.
- Closures as data (2.4, 2.6): church-encoded pairs and numerals need function types everywhere; in Rust they need trait objects (`Rc<dyn Fn>`), which changes the definitions' shape.
- Symbols and quotation (2.53 to 2.58, 2.55, 2.67 to 2.72): no symbols in any host language. The differentiation program needs an expression type (enum/union/sealed variant) with a symbol case as strings, plus constructor helpers replacing quoting. `memq`/`eq?` comparisons in 2.53/2.54 become structural comparisons on a symbol type.
- Printed-list reading: 2.24 to 2.26 and 2.53 read Scheme's box-and-pointer or printed output; the editions must restate predictions over their own tree/collection syntax or drop the reading.
- Recursive heterogeneous data: every tree over "number or sub-list" (2.27, 2.28, 2.29 to 2.31, 2.35) needs a recursive sum type where Scheme needs nothing; the same type is reused by the differentiation expressions (2.56 to 2.58) and Huffman trees (2.67 to 2.72).
- Exact integers and overflow: 2.5 needs big integers; 2.95's observation about integer-exact gcd arithmetic assumes unbounded integers in every edition.
- Iterative accumulation without mutable pairs: 2.18, 2.22, 2.39, 2.63's `copy-to-list` must thread accumulators functionally (Rust/OCaml folds, TS readonly arrays, Kotlin `PersistentList`); 2.22's lesson about building the list in reverse survives but the mechanics differ.
- Error signaling: 2.10 and 2.68 say "signal an error"; each edition must map this to its error type (Result, exceptions, Effect failures, Kotlin requires/null or exceptions) consistently.
- Efficiency statements (2.43, 2.63, 2.72): comparisons of growth need a uniform step-counting or timing harness across editions, or the estimates stay on paper.

### Exercises needing something outside the standard library

| Exercise | Rust | OCaml | TypeScript | Kotlin |
|---|---|---|---|---|
| 2.5 | `num-bigint` | `zarith` | `BigInt` (ECMAScript built-in, not a library) | `java.math.BigInteger` (JDK, not Kotlin stdlib) |
| 2.86 | `num-traits` for a generic numeric abstraction over the tower | none (module signature suffices) | none (numeric union case suffices) | none (interface suffices) |
| 2.74, 2.75 | none (plain structs and closures) | none (modules and closures) | Effect v4 `Schema` for the per-division record formats | none (data classes and interfaces) |

Everything else uses only each language's standard library (or the edition's declared stack: Effect for TypeScript, Arrow for Kotlin). If the TypeScript edition counts Effect as outside stdlib, rows 2.73 to 2.97 broadly lean on `Effect.Match` for tag dispatch; listed only where it is load-bearing (2.74, 2.75).

## Chapter 3 exercise classification (3.1 to 3.82) for the four editions

Source read: `/home/alpha/book/modern-sicp/sicp-pocket.texi` lines 13971 to 22509, in twelve ~700-line chunks. All 82 `@anchor{Exercise 3.N}` statements located by grep (3.1 at 14354 … 3.82 at 22329) and each confirmed read.

Language facts grounded this session:
- Rust std: `Rc`, `RefCell`, `Arc`, `Mutex`, `std::thread`, `std::sync::atomic`, `mpsc`: https://doc.rust-lang.org/std/
- OCaml 5.x manual: physical equality `(==)` (Repr.phys_equal), `Mutex.lock/try_lock/unlock`, `Mutex.protect` (since 5.1), Stdlib modules: https://ocaml.org/manual/latest/api/Stdlib.html, https://ocaml.org/manual/latest/api/Mutex.html
- Effect v4: `Effect` type, fibers, structured concurrency, streaming guides: https://effect.website/docs/v4/onboarding, https://effect.website/docs/v4/getting-started, https://effect.website/docs/v4/concurrency/basic-concurrency (Ref/Semaphore/STM module names are the project's shared-spec vocabulary within the documented v4 Concurrency guides)
- kotlin-stdlib: lazy sequences, `kotlin.concurrent.atomics`, `kotlin.coroutines`, `synchronized`: https://kotlinlang.org/api/core/kotlin-stdlib/

Editorial assumption (changes output if reversed): all four editions share one chapter 3 specification: a mutable-pair analog is defined in the main text (Rust `Rc<RefCell<Node>>`, OCaml mutable record cells, TS/Kotlin objects with mutable fields), `parallel-execute` and serializers are translated as main-text library constructs, and streams keep the book's memoized `delay` semantics (Rust hand-rolled `memo-proc`, OCaml `Lazy.t`, TS hand-rolled over thunks/`Effect.suspend`, Kotlin `lazy{}`). If an edition instead adopts native non-memoized streams (OCaml `Seq`, Kotlin `Sequence`, Effect `Stream`), flip 3.51, 3.57, 3.63 to A for that language.

Cell key: `T` translate; `A: reason` adapt; `R: topic` replace.

### Section 3.1: Assignment and local state (8 rows)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 3.1 | accumulator keeps running sum | T | T | T | T | accumulator returning transaction history |
| 3.2 | monitored procedure counts and resets | T | T | T | T | monitored reports min and max |
| 3.3 | password-protected account dispatch | T | T | T | T |  |
| 3.4 | lockout after seven bad passwords | T | T | T | T | lockout with audit log (chosen: kotlin — implemented as 3.4a, an audit log recording every access attempt with its operation and outcome; see report) |
| 3.5 | Monte Carlo integration via rand | T | T | T | T | seeded rand for reproducible tests (chosen: rust — implemented as 3.5a, seeded stream snapshot plus Cesaro estimates at 100/1,000/10,000 trials) |
| 3.6 | rand with generate, reset messages | T | T | T | T |  |
| 3.7 | make-joint shares one account | T | T | T | T | multi-signature joint account (chosen: ocaml — implemented as 3.7a, read-only account capability by record projection, not the literal "multi-signature joint account" text; see report) |
| 3.8 | expose operand evaluation order | A: order defined; exercise verifies it | T | A: order defined; exercise verifies it | A: order defined; exercise verifies it | operand order in nested calls |

Notes: local state in a single closure works everywhere (Rust `impl FnMut`, OCaml `ref`, TS `let`, Kotlin `var`). OCaml keeps 3.8 as T because OCaml operand order is unspecified, matching Scheme's premise. Rust 3.7 returns `Box<dyn FnMut>` for wrappability.

### Section 3.2: Environment model (3 rows)

All three are environment-diagram exercises about Scheme frames, so all four editions need replacements.

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 3.9 | environment structures of two factorials | R: diagram call stack, no TCO | R: diagram stack growth, tail calls guaranteed | R: diagram call stack growth, no TCO | R: diagram tailrec compilation stack behavior | stack frames you can measure: recursive versus tailrec factorial on a small stack (chosen: kotlin — implemented as 3.9a, a small-stack thread run that overflows on the recursive version and completes on the tailrec version; see report) |
| 3.10 | let desugaring adds a frame | R: trace closure capture of balance | R: trace ref cell captured lifetime | R: trace captured binding lifetimes | R: trace captured var closure semantics | shared-cell capture: move copies the pointer, not the binding (chosen: rust — implemented as 3.10a, two withdrawal processors built over clones of one Rc Cell balance share state while the same processors over a copied integer stay independent; see report) |
| 3.11 | where account state lives | R: diagram closure state sharing between accounts | R: diagram per-account closure state | R: diagram per-factory closure state | R: diagram per-factory closure state | object identity versus structural equality; chosen: ocaml |

### Section 3.3: Modeling with mutable data (26 rows)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 3.12 | append! mutates tail; missing car | A: shared mutable cells need Rc RefCell | A: mutable record cells replace pairs | T | T |  |
| 3.13 | make-cycle builds circular structure | A: shared mutable cells need Rc RefCell | A: mutable record cells replace pairs | T | T | cycle-safe printing; chosen: ocaml |
| 3.14 | mystery reverses pointers in place | A: in-place pair mutation via RefCell | A: mutable records replace set-car! | T | T |  |
| 3.15 | set-to-wow diagrams show sharing | A: aliasing through Rc RefCell cells | A: aliasing through mutable records | T | T |  |
| 3.16 | count-pairs double counts shared pairs | T | T | T | T |  |
| 3.17 | count distinct pairs with history | T | A: identity set needs physical-equality scan | T | A: no identity set in stdlib | count distinct nodes in DAG |
| 3.18 | detect cycle in list | T | A: identity set needs physical-equality scan | T | A: no identity set in stdlib |  |
| 3.19 | tortoise-hare constant-space detection | T | T | T | T | shared suffix detection; chosen: rust |
| 3.20 | environment diagram of pair mutators | R: trace aliasing through closure dispatch | R: trace aliasing through closure dispatch | R: trace object aliasing via closures | R: trace shared state via closures |  |
| 3.21 | print-queue for pointer-pair queue | A: queue nodes need Rc RefCell | A: mutable record nodes replace pairs | T | T | render queue as list view |
| 3.22 | queue as message-passing closure | A: shared nodes and state via RefCell | A: mutable record nodes replace pairs | T | T |  |
| 3.23 | deque with constant-time ends | A: doubly linked cells need RefCell | A: mutable records with parent links | T | T | compare with standard deque library |
| 3.24 | table keyed by same-key? predicate | A: table state shared via RefCell | A: mutable record backbone table | T | T |  |
| 3.25 | table under arbitrary key lists | A: table state shared via RefCell | A: mutable record backbone table | T | T |  |
| 3.26 | binary-tree table records | A: in-place tree mutation via RefCell | A: mutable nodes for in-place tree | T | T | ordered key iteration table |
| 3.27 | memoized fib via table | A: RefCell memo table; diagram becomes trace | A: Hashtbl memo; diagram becomes trace | A: Map memo; diagram becomes trace | A: HashMap memo; diagram becomes trace |  |
| 3.28 | or-gate primitive | T | T | T | T |  |
| 3.29 | or-gate from and plus inverter | T | T | T | T |  |
| 3.30 | ripple-carry adder composition | T | T | T | T | verify adder against integer addition; chosen: kotlin |
| 3.31 | why accept-action runs immediately | T | T | T | T |  |
| 3.32 | agenda segment FIFO order | T | T | T | T |  |
| 3.33 | averager constraint network | T | T | T | T | product constraint with division |
| 3.34 | squarer via multiplier flawed | T | T | T | T |  |
| 3.35 | squarer as primitive constraint | T | T | T | T |  |
| 3.36 | environment diagram of connector wiring | R: trace connector closure call graph | R: trace connector closure call graph | R: trace connector observer wiring | R: trace connector observer wiring |  |
| 3.37 | expression-style constraint combinators | T | T | T | T |  |

Notes: `eq?` translates to `Rc::ptr_eq`, OCaml `(==)`, JS `===`, Kotlin `===` (syntax-level, so 3.16/3.19 are T). 3.17/3.18 need an identity-keyed set: Rust `HashSet<*const Node>` is stdlib (T); OCaml and Kotlin stdlib lack one (A). Circuit and constraint exercises (3.28 to 3.35, 3.37) are gate/constraint composition over the translated simulator, so statements survive.

### Section 3.4: Concurrency (12 rows)

Statements quote the book's `parallel-execute` and serializers; under the shared spec these are main-text constructs in every edition, so most rows survive. Real threads/fibers/domains/coroutines make the interleavings real but the asked-for enumerations and timing diagrams are unchanged.

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 3.38 | enumerate interleaved balance outcomes | T | T | T | T | run interleavings with real threads (chosen: rust) |
| 3.39 | which serialized outcomes remain | T | T | T | T |  |
| 3.40 | all values of concurrent x squared | T | T | T | T |  |
| 3.41 | should balance reads serialize | T | T | T | T | stale reads under eventual consistency (chosen: kotlin — implemented as 3.41a, a stale authorization on a balance read before a concurrent withdraw commits; see report) |
| 3.42 | serialize once outside dispatch | T | T | T | T |  |
| 3.43 | exchange preserves multiset of balances | T | T | T | T |  |
| 3.44 | transfer needs no joint lock | T | T | T | T |  |
| 3.45 | double serialization deadlocks | T | T | T | T |  |
| 3.46 | test-and-set race window | T | T | T | T | demonstrate race via stress test |
| 3.47 | semaphore from mutex or test-and-set | T | T | A: Effect provides Semaphore; build anyway | A: kotlinx provides Semaphore; build anyway | blocking bounded semaphore with try-acquire; chosen: ocaml — implemented as 3.47a, a bounded semaphore on Mutex and Condition; see report |
| 3.48 | deadlock avoidance by lock ordering | T | T | T | T |  |
| 3.49 | ordering avoidance fails scenario | T | T | T | T | resource acquisition without known set |

Notes: Rust 3.47 is a natural fit: `AtomicBool::compare_exchange` is literally the atomic test-and-set the exercise asks for; OCaml has Stdlib `Mutex.try_lock` and `Atomic`. TS/Kotlin both ship ready-made semaphores, so the tailored note is that the exercise becomes a reimplementation exercise next to a library one.

### Section 3.5: Streams (33 rows)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 3.50 | multi-stream map completion | A: variadic becomes stream list parameter | A: variadic becomes stream list parameter | T | T |  |
| 3.51 | show reveals memoized delay timing | T | T | T | T | count force operations explicitly; chosen: ocaml — implemented as 3.51a, an instrumented delay whose counters separate tail accesses from tail bodies; see report |
| 3.52 | accum traces assignment plus laziness | A: RefCell sum plus lazy timing | A: ref sum plus Lazy timing | A: Ref sum plus suspend timing | A: var sum plus lazy timing |  |
| 3.53 | predict self-referential doubling stream | T | T | T | T |  |
| 3.54 | mul-streams and factorial stream | T | T | T | T | stream of Catalan numbers (chosen: rust) |
| 3.55 | partial-sums combinator | T | T | T | T |  |
| 3.56 | Hamming numbers via merge | T | T | T | T | Hamming numbers with uniqueness proof |
| 3.57 | fib additions with memoized delay | T | T | T | T |  |
| 3.58 | expand computes long division digits | T | T | T | T |  |
| 3.59 | integrate-series, exp sin cos | T | T | T | T | differentiate series termwise; chosen: kotlin |
| 3.60 | mul-series convolution | T | T | T | T |  |
| 3.61 | invert-unit-series | T | T | T | T |  |
| 3.62 | div-series and tangent series | T | T | T | T |  |
| 3.63 | sqrt-stream memoization locality | T | T | T | T |  |
| 3.64 | stream-limit convergence helper | T | T | T | T | compare sequence accelerators convergence |
| 3.65 | ln 2 approximation streams | T | T | T | T |  |
| 3.66 | pairs ordering analysis | T | T | T | T |  |
| 3.67 | all pairs via extra interleave | T | T | T | T |  |
| 3.68 | Louis pairs infinite recursion | T | T | T | T |  |
| 3.69 | triples and Pythagorean stream | T | T | T | T | filter triples by coprimality |
| 3.70 | merge-weighted, weighted-pairs | T | T | T | T |  |
| 3.71 | Ramanujan numbers via weighted pairs | T | T | T | T | taxicab numbers beyond two ways |
| 3.72 | sums of two squares thrice | T | T | T | T |  |
| 3.73 | RC circuit signal processor | T | T | T | T | plot capacitor discharge curve |
| 3.74 | zero crossings via stream-map | T | T | T | T |  |
| 3.75 | buggy smoothing detector fix | T | T | T | T |  |
| 3.76 | smooth as reusable combinator | T | T | T | T |  |
| 3.77 | integral with delayed integrand | T | T | T | T |  |
| 3.78 | solve-2nd feedback loop | T | T | T | T | damped oscillator solution stream |
| 3.79 | general second-order solver | T | T | T | T |  |
| 3.80 | RLC coupled streams | T | T | T | T |  |
| 3.81 | rand request stream, no assignment | T | T | T | T | replayable request stream tests |
| 3.82 | streaming Monte Carlo integration | T | T | T | T | streaming estimates with confidence |

### Per-section counts (rows T/A/R)

| Section | Rows | Rust | OCaml | TypeScript | Kotlin |
|---|---|---|---|---|---|
| 3.1 | 8 | 7/1/0 | 8/0/0 | 7/1/0 | 7/1/0 |
| 3.2 | 3 | 0/0/3 | 0/0/3 | 0/0/3 | 0/0/3 |
| 3.3 | 26 | 13/11/2 | 11/13/2 | 23/1/2 | 21/3/2 |
| 3.4 | 12 | 12/0/0 | 12/0/0 | 11/1/0 | 11/1/0 |
| 3.5 | 33 | 31/2/0 | 31/2/0 | 32/1/0 | 32/1/0 |
| **Chapter 3** | **82** | **63/14/5** | **62/15/5** | **73/4/5** | **71/6/5** |

Each row sums to 82 per language.

### Hard spots

- Shared mutable pairs are the chapter's real divergence. Rust needs `Rc<RefCell<Node>>` for every queue, table, and deque cell; reference cycles (3.13) leak by design and printing cycles can hang a naive `Display`. OCaml lists are immutable, so the main text must define mutable record cells up front; after that the exercises read as translation.
- Identity: `eq?` maps to `Rc::ptr_eq` / `(==)` / `===` / `===`, but identity-keyed collections exist only in Rust (`HashSet<*const T>`) and TS (`Set` is reference-keyed for objects). OCaml needs a physical-equality scan or a hand-rolled table; Kotlin needs `java.util.IdentityHashMap` or id-tagging.
- Self-referential stream definitions (`ones`, `integers`, `fibs`, `primes`, `integral`'s `int`): OCaml permits `let rec` over `Lazy`-celled streams; Kotlin self-reference works inside `by lazy`; TS needs a thunk/`Effect.suspend` closure; Rust needs function-based streams or `Rc::new_cyclic`/`OnceCell` cycles. Statements survive, but this is the chapter's hardest Rust design decision.
- Memoized `delay`: only OCaml `Lazy.t` memoizes out of the box; Rust/TS/Kotlin editions must hand-roll the book's `memo-proc`. 3.51, 3.57, 3.63 answers depend on this editorial choice (see assumption above).
- Environment-diagram exercises (3.9 to 3.11, 3.20, 3.27, 3.36) lean on Scheme frames; each edition needs the replacement trace/diagram named in the R cells.
- Concurrency: the busy-wait mutex of 3.46 burns cores under real threads; recommend implementing `test-and-set!` over atomics (Rust `AtomicBool`, OCaml `Atomic`) or blocking primitives. Real nondeterminism makes exhaustive "list all values" answers paper exercises; a stress-test tailored addition (3.46) covers the gap.
- REPL interactions: the book's `;Value:` transcripts (3.21, 3.51, 3.52) have no counterpart; each edition must define a printing convention, and quoted printed forms change in every edition.

### Outside-standard-library needs

- Rust: none. `Rc`, `RefCell`, `Arc`, `Mutex`, `std::thread`, `std::sync::atomic`, `HashSet` are all std (https://doc.rust-lang.org/std/).
- OCaml: none. `ref`, `Lazy`, `Seq`, `Hashtbl`, `Mutex`, `Atomic`, `Domain`/`Thread` are Stdlib (https://ocaml.org/manual/latest/api/Stdlib.html, https://ocaml.org/manual/latest/api/Mutex.html). The Jane Street Base/Core appendix needs no additions for chapter 3.
- TypeScript: everything concurrency- and state-related comes from the `effect` package (npm, outside the TS standard library): `Effect.Ref`, `SynchronizedRef`, `Semaphore`, `STM`, `Stream`, fibers (https://effect.website/docs/v4/getting-started). A bare-TS fallback for 3.4 would need `node:worker_threads`, also outside TS stdlib.
- Kotlin: `kotlinx.coroutines` (external library) for `Mutex`, `Semaphore`, `Channel`, `Flow`, and coroutine builders: needed for section 3.4 and any coroutine-based streams. Stdlib-only `Sequence`, `lazy`, `kotlin.concurrent.atomics`, `synchronized` cover 3.1 to 3.3 and 3.5 (https://kotlinlang.org/api/core/kotlin-stdlib/). Identity collections (3.17/3.18) also reach for `java.util.IdentityHashMap` (JDK, beyond kotlin-stdlib) unless an id-tagging workaround is chosen.

## Chapter 4 exercise classification (4.1 to 4.79)

Legend: `T` = translate, syntax changes only. `A: reason` = adapt, the statement changes because the host language differs. `R: topic` = replace, the exercise is about Scheme itself; the replacement extends the same number. Tailored additions are numbered as the row plus a letter; empty cells mean no addition.

Reading coverage: 65 of the 79 statements were read verbatim from the Texinfo this session. For 14 rows (4.11, 4.12, 4.13, 4.27, 4.44, 4.45, 4.46, 4.50, 4.51, 4.60, 4.61, 4.62, 4.63, and the tail of 4.69) only the surrounding section text and cross-references were read; their topics follow from those anchors and none of them changes classification under the default.

### Section 4.1 (lines 22511 to 24871, 24 exercises)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 4.1 | operand evaluation order in list-of-values | A: host fixes order, reword premise | T | A: host fixes order, reword premise | A: host fixes order, reword premise | |
| 4.2 | dispatch order and call-prefixed applications | T | T | T | T | 4.2a: measure dispatch checks saved by reordering |
| 4.3 | data-directed dispatch in eval | T | T | T | T | |
| 4.4 | and and or as special forms | T | T | T | T | |
| 4.5 | cond arrow clauses | T | T | T | T | 4.5a: add cond clause with empty body |
| 4.6 | let as derived expression | T | T | T | T | |
| 4.7 | let* as nested lets | T | T | T | T | |
| 4.8 | named let | T | T | T | T | 4.8a: add named let with default bindings |
| 4.9 | design iteration constructs | T | T | T | T | |
| 4.10 | new syntax, unchanged eval | T | T | T | T | |
| 4.11 | frame as association list | T | T | T | T | 4.11a: benchmark lookup on both frame representations |
| 4.12 | abstract environment traversals | T | T | T | T | |
| 4.13 | make-unbound! removes binding | T | T | T | T | |
| 4.14 | host map as primitive fails | T | T | T | T | 4.14a: install host sort as primitive, diagnose (chosen: ocaml) |
| 4.15 | halting problem diagonal argument | T | T | T | T | |
| 4.16 | scan out internal definitions | T | T | T | T | |
| 4.17 | extra frame from scan-out | T | T | T | T | 4.17a: print environment structure during evaluation |
| 4.18 | alternative scan-out strategy | T | T | T | T | |
| 4.19 | internal definition scoping debate | T | T | T | T | |
| 4.20 | letrec as derived expression | T | T | T | T | 4.20a: add nested letrec shadowing error test (chosen: kotlin) |
| 4.21 | recursion without define | T | T | T | T | |
| 4.22 | let in analyze evaluator | T | T | T | T | |
| 4.23 | analyze-sequence comparison | T | T | T | T | 4.23a: count analysis invocations with a counter (chosen: rust) |
| 4.24 | benchmark analysis versus execution | T | T | T | T | |

Counts, summing to 24 per language: Rust 23 T / 1 A / 0 R; OCaml 24 / 0 / 0; TypeScript 23 / 1 / 0; Kotlin 23 / 1 / 0.

### Section 4.2 (lines 24872 to 25621, 10 exercises)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 4.25 | unless breaks under applicative order | T | T | T | T | |
| 4.26 | unless as special form debate | T | T | T | T | 4.26a: add when as derived expression |
| 4.27 | lazy id with set! | T | T | T | T | |
| 4.28 | forcing the operator | T | T | T | T | |
| 4.29 | memoization speed difference | T | T | T | T | 4.29a: toggle memoization and count forcings (chosen: ocaml) |
| 4.30 | forcing in eval-sequence | T | T | T | T | |
| 4.31 | lazy and memo parameter declarations | T | T | T | T | |
| 4.32 | chapter 3 streams versus lazy lists | T | T | T | T | 4.32a: build lazy tree, force selectively (chosen: rust) |
| 4.33 | quote produces lazy lists | T | T | T | T | |
| 4.34 | printing lazy pairs | T | T | T | T | |

Counts, summing to 10 per language: Rust 10 T / 0 A / 0 R; OCaml 10 / 0 / 0; TypeScript 10 / 0 / 0; Kotlin 10 / 0 / 0.

### Section 4.3 (lines 25622 to 27115, 20 exercises)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 4.35 | an-integer-between and triples | T | T | T | T | 4.35a: count amb choices per triple |
| 4.36 | unbounded Pythagorean triples | T | T | T | T | |
| 4.37 | Ben's triple generator efficiency | T | T | T | T | |
| 4.38 | multiple dwelling minus Smith-Fletcher clause | T | T | T | T | 4.38a: rank solutions by violated constraints |
| 4.39 | restriction order effects | T | T | T | T | |
| 4.40 | prune assignments before restrictions | T | T | T | T | |
| 4.41 | ordinary program solves puzzle | A: ordinary host program, reword language | A: ordinary host program, reword language | A: ordinary host program, reword language | A: ordinary host program, reword language | 4.41a: compare host solver nodes with amb |
| 4.42 | liars puzzle | T | T | T | T | |
| 4.43 | yacht puzzle | T | T | T | T | |
| 4.44 | eight queens puzzle | T | T | T | T | 4.44a: count backtracks per board size (chosen: ocaml) |
| 4.45 | five parses of ambiguous sentence | T | T | T | T | |
| 4.46 | left-to-right operands in amb | T | T | T | T | |
| 4.47 | Louis's recursive parse-verb-phrase | T | T | T | T | 4.47a: compare parse counts before after change |
| 4.48 | extend grammar | T | T | T | T | |
| 4.49 | sentence generation | T | T | T | T | |
| 4.50 | ramble random choice | T | T | T | T | 4.50a: seed host RNG for reproducible ramble (chosen: rust) |
| 4.51 | permanent-set! | T | T | T | T | |
| 4.52 | if-fail | T | T | T | T | |
| 4.53 | permanent-set! with if-fail | T | T | T | T | 4.53a: predict, then verify, permanent-set! interaction |
| 4.54 | require as special form | T | T | T | T | |

Counts, summing to 20 per language: Rust 19 T / 1 A / 0 R; OCaml 19 / 1 / 0; TypeScript 19 / 1 / 0; Kotlin 19 / 1 / 0.

### Section 4.4 (lines 27116 to 30278, 25 exercises)

| Exercise | Topic | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 4.55 | simple Microshaft queries | T | T | T | T | |
| 4.56 | compound queries | T | T | T | T | 4.56a: add negated compound query examples |
| 4.57 | can-replace rule | T | T | T | T | |
| 4.58 | big shot rule | T | T | T | T | |
| 4.59 | meeting-time rule | T | T | T | T | 4.59a: write rule with two body clauses |
| 4.60 | lives-near duplicate names | T | T | T | T | |
| 4.61 | last-pair rule | T | T | T | T | |
| 4.62 | logic gates as rules | T | T | T | T | 4.62a: implement not-gate as a rule |
| 4.63 | family relations rules | T | T | T | T | |
| 4.64 | outranked-by infinite loop | T | T | T | T | |
| 4.65 | wheel listed four times | T | T | T | T | 4.65a: deduplicate wheel answers with same rule |
| 4.66 | accumulation over frames | T | T | T | T | |
| 4.67 | query loop detector | T | T | T | T | |
| 4.68 | reverse as rules | T | T | T | T | 4.68a: write palindrome rule using reverse |
| 4.69 | great-grandson rules | T | T | T | T | |
| 4.70 | let binding in add-assertion! | T | T | T | T | |
| 4.71 | explicit delay in simple-query | T | T | T | T | 4.71a: find query where undelayed version diverges |
| 4.72 | interleave versus append | T | T | T | T | |
| 4.73 | delay in flatten-stream | T | T | T | T | |
| 4.74 | simple-stream-flatmap | T | T | T | T | 4.74a: measure frame counts, old versus simple (chosen: ocaml) |
| 4.75 | unique special form | T | T | T | T | |
| 4.76 | merging frames for and | T | T | T | T | |
| 4.77 | delayed filtering for not | T | T | T | T | 4.77a: test delayed filter on partial bindings |
| 4.78 | query language on amb evaluator | T | T | T | T | |
| 4.79 | environments instead of renaming | T | T | T | T | |

Counts, summing to 25 per language: Rust 25 T / 0 A / 0 R; OCaml 25 / 0 / 0; TypeScript 25 / 0 / 0; Kotlin 25 / 0 / 0.

Overall over 79 exercises: Rust 77 T / 2 A / 0 R; OCaml 78 / 1 / 0; TypeScript 77 / 2 / 0; Kotlin 77 / 2 / 0.

Why so few A rows: under the shared-specification default every edition evaluates the same Scheme subset, so the object language never changes and an exercise that adds a Scheme special form (4.2, 4.4, 4.5, 4.13, 4.26, 4.31, 4.50 to 4.54, 4.75) stays T in all four editions. Only two statements name the host's role: 4.1 ("inherited from the underlying Lisp") and 4.41 ("ordinary Scheme program"), hence their A. No chapter 4 exercise needs the host and object language to coincide (no REPL redefinition, no quasiquote trick, no evaluator reading itself as code), so there are zero R rows.

### Hard spots

- The amb evaluator's backtracking is host-mechanism-bound, and exercises 4.50 to 4.54 sit on top of it. Rust has no continuations, so the evaluator carries an explicit choice-point stack of captured thunks; OCaml uses effect handlers (Effect.perform with a handler that resumes a stored continuation); TypeScript uses Effect v4 fibers with the try-again driver state; Kotlin uses the sequence builder's suspension. 4.51 (permanent-set!) and 4.52 (if-fail) must keep their semantics aligned with whichever mechanism the edition chose.
- The 4.1.7 analyze evaluator makes execution procedures host closures over the parsed expression. 4.23's comparison of analysis-time versus run-time work must be made observable in each host, and 4.24's benchmark needs a host timer with warmup to be meaningful.
- The query system's explicit delay (4.70 to 4.74) needs delayed stream cells in every host: Rust closures under Rc, OCaml lazy, Effect v4 lazy thunks or Stream, Kotlin sequence. The cyclic assertion stream in 4.70 must not force eagerly.
- Environment mutation (set-variable-value!, define-variable!, add-binding-to-frame!) mutates shared frames: Rust needs Rc<RefCell<Frame>> with try_borrow discipline, OCaml a ref-based frame, TypeScript a mutable frame object behind Ref, Kotlin a mutable frame class. This is background for 4.11 to 4.13 and 4.16 but changes no statement.
- The two A rows are statement-level, not solution-level: 4.1's premise about unspecified underlying order holds only in OCaml (argument order unspecified); 4.41's "ordinary Scheme" means the reader's working language, which is now the host.

### Out-of-standard-library needs

| Language | Exercises | Need | Evidence |
|---|---|---|---|
| Rust | 4.50 | pseudorandom choice for ramble | rand crate, or keep std-only with an inlined xorshift (https://doc.rust-lang.org/std/cell/index.html confirms std has no RNG) |
| OCaml | none | | |
| TypeScript | none beyond the Effect v4 edition baseline | | |
| Kotlin | 4.11, 4.12, 4.13, 4.16 | typed unbound-variable and unassigned errors as Either | arrow-core (https://arrow-kt.io/learn/typed-errors/working-with-typed-errors/) |

Grounding notes: OCaml effect handlers from https://ocaml.org/manual/latest/effects.html (read this session). Rust interior mutability and Rc<RefCell> from https://doc.rust-lang.org/std/cell/index.html (read this session). Arrow Either, Raise, and fold from https://arrow-kt.io/learn/typed-errors/working-with-typed-errors/ (read this session). Effect v4 Data.TaggedError, Effect.fail, catchTag from https://effect.website/docs/v4/error-management/expected-errors.md (read this session). The Kotlin sequence builder URL (https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.sequences/sequence.html) returned a JetBrains media-kit PDF instead of the API page, so that citation is unverified; the builder is standard-library knowledge. The claim that Rust, TypeScript, and Kotlin specify left-to-right argument evaluation, and that OCaml leaves argument order unspecified, is unverified as a citation this session. Effect v4's Schema.TaggedError name was not confirmed this session (the v4 error-management page shows Data.TaggedError); use Data.TaggedError in the edition conventions.

Alternative impact: if the four editions instead evaluated a subset of their own host language, 29 of the 79 rows would change classification: the T rows whose statements quote Scheme program text or transcripts (4.2, 4.5, 4.6, 4.7, 4.8, 4.15, 4.18, 4.19, 4.20, 4.21, 4.23, 4.25, 4.27, 4.29, 4.30, 4.31, 4.33, 4.35, 4.37, 4.47, 4.52, 4.53, 4.54, 4.70, 4.71, 4.73, 4.74, 4.79) would become A, and 4.3 would become A because taking the car of a compound expression stops being a list operation; the two A rows keep their class and the R count stays zero.

## Chapter 5 exercise classification (5.1 to 5.52) for the four editions

Source: `/home/alpha/book/modern-sicp/sicp-pocket.texi`, lines 30281 to 37359 (the `@node References` line), read in chunks of about 700 lines. All 52 exercise statements were read this session. Section boundaries verified against node lines: 5.1 has exercises 5.1 to 5.6, 5.2 has 5.7 to 5.19, 5.3 has 5.20 to 5.22, 5.4 has 5.23 to 5.30, 5.5 has 5.31 to 5.52. Section 5.3.2 (garbage collection) contains no exercises. Anchors run from 5.1 at line 30519 to 5.52 at line 37353.

Cell key: `T` translate, `A: reason` adapt, `R: topic` replace.

Classification stance, from the assignment defaults: every edition implements the register-machine simulator, the explicit-control evaluator, and the compiler for the shared SICP Scheme subset in its host language. The subject of chapter 5 is the book's register-machine language, which is the same object of study in all four editions, so statements about machine design, controller sequences, and simulator or compiler extensions survive almost everywhere. The typed-representation changes named in the assignment (instruction enums instead of list structure, a typed memory model for `the-cars` and `the-cdrs`, typed compiler instruction sequences, the host stack, host GC) land in the main text and in the idiom notes, not in exercise statements, because the chapter 5 statements are phrased at the interface level. The two exceptions need the chapter 4 evaluator to exist as object-language source.

Basis: language facts are the assignment's given set (Rust enums, `Vec`, no GC, `Rc<RefCell>`; OCaml variants, arrays, GC; TypeScript discriminated unions, `Ref`, arrays, GC; Kotlin sealed classes, arrays, GC). No new API claims were introduced, so no documentation URLs are cited. Plan decisions D9 (shared object language with grammar and program spec), D20 (pair representation), and D23 (given reader and printer) were read from `modern-sicp-editions-plan.md` in this session's local directory.

Editorial calls that would change the output if reversed:

- 5.50 and 5.52 are `A` because the editions' chapter 4 evaluator is host code, so "compile the metacircular evaluator" needs a new given example: the evaluator written in the object language, for example `examples/ch5/metacircular.scm`. If the plan declines to ship that source, flip both to `R` with replacement topic "compile a provided substantial object-language program". Whether such a source exists in the plan is unverified.
- No exercise depends on the host REPL. `compile-and-go`, `compile-and-run`, and the EC-Eval driver loop translate fully: the driver loop is part of the simulated machine, and machine operations are host functions. A plan that treats the whole interfacing subsection as host dependent would flip 5.45, 5.47, 5.48, and 5.49 to `A`; the statements as written survive.

### Section 5.1, register machines (6 rows)

| Exercise | Topic (8 words max) | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 5.1 | design factorial machine, draw diagrams | T | T | T | T | loop the controller over repeated inputs |
| 5.2 | write iterative factorial controller sequence | T | T | T | T |  |
| 5.3 | sqrt machine via Newton's method, two stages | T | T | T | T |  |
| 5.4 | controller sequences for two expt machines | T | T | T | T |  |
| 5.5 | hand-simulate factorial and fib machines | T | T | T | T | annotate each restore with its matching save |
| 5.6 | remove redundant save and restore | T | T | T | T |  |

### Section 5.2, the simulator (13 rows)

| Exercise | Topic (8 words max) | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 5.7 | test designed machines on the simulator | T | T | T | T | compare simulator result with direct computation |
| 5.8 | duplicate label detection in assembler | T | T | T | T |  |
| 5.9 | forbid labels as operation operands | T | T | T | T |  |
| 5.10 | new surface syntax, isolated syntax procedures | T | T | T | T |  |
| 5.11 | three save and restore disciplines | T | T | T | T |  |
| 5.12 | assembler collects instruction and register summary | T | T | T | T | report instruction counts by type |
| 5.13 | derive register set from controller text | T | T | T | T |  |
| 5.14 | measure pushes and depth for factorial | T | T | T | T |  |
| 5.15 | instruction counting in machine model | T | T | T | T | alert when count exceeds a budget |
| 5.16 | instruction tracing on and off | T | T | T | T |  |
| 5.17 | print labels preceding traced instruction | T | T | T | T |  |
| 5.18 | per-register tracing in make-register | T | T | T | T |  |
| 5.19 | breakpoints with proceed and cancel | T | T | T | T | conditional breakpoint on register contents |

### Section 5.3, storage allocation and garbage collection (3 rows)

| Exercise | Topic (8 words max) | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 5.20 | draw pair and memory-vector representations | T | T | T | T | show free pointer after three conses |
| 5.21 | count-leaves machines over list memory | T | T | T | T |  |
| 5.22 | append and append! machines | T | T | T | T |  |

### Section 5.4, the explicit-control evaluator (8 rows)

| Exercise | Topic (8 words max) | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 5.23 | derived expressions via transformer operations | T | T | T | T |  |
| 5.24 | cond as basic controller form | T | T | T | T | add and and or as special forms |
| 5.25 | normal-order evaluation in the controller | T | T | T | T |  |
| 5.26 | stack behavior of iterative factorial | T | T | T | T | plot maximum depth against n |
| 5.27 | stack behavior of recursive factorial | T | T | T | T |  |
| 5.28 | remove tail recursion, rerun experiments | T | T | T | T |  |
| 5.29 | stack formulas for tree-recursive fib | T | T | T | T |  |
| 5.30 | error signaling inside the evaluator | T | T | T | T | trap division by zero and bad car |

### Section 5.5, compilation (22 rows)

| Exercise | Topic (8 words max) | Rust | OCaml | TypeScript | Kotlin | Tailored addition idea |
|---|---|---|---|---|---|---|
| 5.31 | which evaluator saves are superfluous | T | T | T | T |  |
| 5.32 | symbol-operator fast path and design opinion | T | T | T | T |  |
| 5.33 | compile factorial-alt, explain differences | T | T | T | T | hand-optimize the alt version's code |
| 5.34 | compile iterative factorial, annotate stack | T | T | T | T |  |
| 5.35 | reverse-engineer source from Figure 5.18 | T | T | T | T |  |
| 5.36 | operand evaluation order in compiler | T | T | T | T | measure code size after reordering |
| 5.37 | preserving disabled, compare stack waste | T | T | T | T |  |
| 5.38 | open-code primitives with arg registers | T | T | T | T | open-code comparison predicates too |
| 5.39 | lexical-address-lookup machine operation | T | T | T | T |  |
| 5.40 | compile-time environment threading | T | T | T | T | dump compile-time environment while compiling |
| 5.41 | find-variable returns lexical address | T | T | T | T |  |
| 5.42 | lexical addressing in code generators | T | T | T | T |  |
| 5.43 | scan out internal definitions | T | T | T | T |  |
| 5.44 | open-coding respects shadowing names | T | T | T | T | warn when open-coded name is rebound |
| 5.45 | stack ratios compiled versus interpreted | T | T | T | T |  |
| 5.46 | fib stack ratios, compiler effectiveness | T | T | T | T |  |
| 5.47 | compiled code calls interpreted procedures | T | T | T | T |  |
| 5.48 | compile-and-run primitive inside evaluator | T | T | T | T | compile-and-run a whole begin block |
| 5.49 | read-compile-execute-print loop machine | T | T | T | T |  |
| 5.50 | compile the metacircular evaluator | A: needs evaluator source in object language | A: needs evaluator source in object language | A: needs evaluator source in object language | A: needs evaluator source in object language | time each interpretation level |
| 5.51 | translate evaluator into C runtime | T | T | T | T |  |
| 5.52 | compiler emits C, compile evaluator | A: needs object-language evaluator source provided | A: needs object-language evaluator source provided | A: needs object-language evaluator source provided | A: needs object-language evaluator source provided |  |

### Counts per section, per language

No row splits by language in this chapter: every statement is host neutral or host neutral after the same provision (the object-language evaluator source).

| Section | T | A | R | Rows |
|---|---|---|---|---|
| 5.1 | 6 | 0 | 0 | 6 |
| 5.2 | 13 | 0 | 0 | 13 |
| 5.3 | 3 | 0 | 0 | 3 |
| 5.4 | 8 | 0 | 0 | 8 |
| 5.5 | 20 | 2 | 0 | 22 |
| Total | 50 | 2 | 0 | 52 |

Counts sum to the 52 rows in every language.

### Hard spots

- Machine words versus object values. Registers hold object-language values and also internal machine values: instruction locations, compiled procedure entries, condition codes. Every edition needs a machine word type wrapping the object `Value` type (a Rust or OCaml enum, a TypeScript union, a Kotlin sealed class). The main text must introduce it before 5.5.7; it surfaces in 5.9, 5.30, and 5.45 to 5.49.
- Execution procedures. The book's assembler builds Scheme closures once at assembly time. OCaml, TypeScript, and Kotlin keep closures naturally; Rust needs boxed closures or a compiled instruction representation with an execute step. This shapes the solutions of 5.8 to 5.19 but changes no statement.
- 5.25 is the hardest 5.4 exercise. Normal-order evaluation spreads across the controller (thunk tests at application time), the operations table, and thunks as tagged object-language data. Plan the controller changes before assigning it.
- 5.45 and 5.46 need one measurement harness that runs the interpreted, compiled, and special-purpose versions under the same monitored machine. Editions should script these sessions deterministically instead of relying on interactive runs.
- 5.50 and 5.52 need the chapter 4 evaluator as object-language source, roughly 300 lines checked against the shared grammar, with its primitives supplied as machine operations. This source is a tailored main-text addition and must exist before either exercise is assignable.
- 5.51 and 5.52 produce C programs. The C runtime reimplements pairs, symbols, and environments from the machine description; nothing in the four hosts helps. Budget an appendix-style walkthrough.
- The book's assembler uses continuation-passing style (`extract-labels` with a `receive` continuation). Rust translations return a struct instead. This colors the 5.8 solution but the statement survives.

### Outside the standard library

- 5.51: a C toolchain is required to build and run the translated evaluator, in all four editions.
- 5.52: the same C toolchain, to compile and run the compiler's C output, in all four editions.

No other chapter 5 exercise needs anything beyond the four standard libraries.
