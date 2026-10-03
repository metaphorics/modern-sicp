# TypeScript edition (Effect v4): idiom map and build-plan input

> Guest contract: `spec/host-subsets/typescript/grammar.md` is normative for guest source. A unit is admitted only when the subset parser, the pinned `tsc` check under `typescript/tsconfig.base.json`, and the observable behavior of grammar §10 agree in order (grammar §1). Chapters 4 and 5 share one typed source contract and one tagged AST across `read`/`readAll`/`readProgram` (grammar §1.2, §3); chapter differences live in evaluators, query/machine domain data, and named experiment modes. Core evaluation is strict eager TypeScript; lazy and search are separately named experimental modes with independent oracles (grammar §6). No chapter evaluates the old shared guest language, quoted executable lists, or old reader/printer behavior.

## 0. Version pin, grounding record, and API ledger

### 0.1 Pins and sources

| Package / tool | Pin | Source and status |
|---|---|---|
| `effect` | `4.0.0-rc.117` | dist-tags `rc`, read at https://registry.npmjs.org/effect (`latest` 3.22.2 is v3, unused; no `next` tag exists) |
| `@effect/vitest` | `4.0.0-rc.117` | dist-tags `rc`, read at https://registry.npmjs.org/@effect/vitest (the `latest` 0.30.0 peer range was not read this session: unverified; the rc pin rests on the dist-tag) |
| `vitest` | `5.0.1` | inside the verified peer range `>=5.0.0 <6.0.0` and equal to the devDependency of `@effect/vitest@4.0.0-rc.117`, read at https://registry.npmjs.org/@effect/vitest/4.0.0-rc.117 |
| Node | `24.21.0` (`.node-version`) | shared-plan pin; not re-verified against a registry this session (compatible with the docs floor of Node 22.18+) |
| TypeScript / Biome / pnpm | `7.0.2` / `2.5.14` / `12.5.1` | shared-plan pins; not re-verified this session |

Release page: https://github.com/Effect-TS/effect/releases/tag/effect%404.0.0-rc.117, read this session. Patch-only release, tag `14a3f14` (matches the API pages' source links), 12 bullets: #8331 cluster sends to unregistered entity types fail with a defect at the registration deadline; #8319 `NetAddress.toCanonical` accepts internet addresses; #8317 cluster workflows no longer stall after request resets; #8323 (two items) multicast address refinements and branded `NetAddress` classifications; #8310 memory-leak fix in `Pool.makeWithTTL` (usage strategy, TTL queue); #8326 MCP output schemas normalized to object roots; #8318 `Queue.shutdownUnsafe` added, `Queue.shutdown` returns `false` when already shut down or completed; #8325 `NetAddress.formatNativeHost`/`formatMulticastInterface`; #8308 SQL savepoints released after nested transactions (PostgreSQL, PGlite, MySQL, libSQL, SQLite) with a `releaseSavepoint` opt-in; #8305 `Decision.probability` `criteria` optional; #8329 warning on conflicting cluster workflow tags. None of these change the APIs the edition's sketches rely on (`Queue` is named in one sketch only as the library contrast after a hand-built queue; its sketched members are unverified anyway). V4 API decisions come from the v4 docs and https://unpkg.com/effect@4.0.0-rc.117/AGENTS.md.

Manifest facts (https://registry.npmjs.org/effect/4.0.0-rc.117): `type: module`; exports `"./*": "./dist/*.js"` plus `./testing` and `./unstable/*` subpaths (including `./unstable/arbitrary`; `./unstable/schema` is a separate unstable schema-compiler surface, while core `Schema` is stable). No `engines` field: the Node floor is the workspace's own choice (`.node-version` above).

### 0.2 Exhaustive API ledger

Status: **V** = verified at that URL this session; **U** = unverified at function level (module existence cited; pin exact names at implementation).

Guest boundary: the guest evaluator kernel may use only the language forms and library members named in the guest contract; existing `effect` operations remain in a driver/error-boundary module only with their typed error channel preserved and are not guest evaluator builtins (grammar §4.4). The kernel returns transcript data instead of calling `console.log`; `Ref`/`Stream`/`TxRef` rows below are host driver/harness vocabulary, not core guest syntax. Where an exact migrated module path is not yet published, the semantic contract and the grammar section govern; no new dependency is added.

| Name | Status | Source |
|---|---|---|
| `Effect.gen`, `Effect.runPromise` | V | https://effect.website/docs/v4/onboarding |
| `Effect.succeed`, `Effect.fail`, `Effect.sync`, `Effect.try` (+ `{try, catch}` overload), `Effect.runSync`, `Effect.runPromiseExit`, `Effect.flip` | V | https://effect.website/docs/v4/getting-started/creating-effects |
| `Effect.all` (with `concurrency: number \| "unbounded"`), `Effect.forEach`, `Effect.sleep`, `Effect.interrupt`, `Effect.onInterrupt`, `Effect.forkChild` | V | https://effect.website/docs/v4/concurrency/basic-concurrency; https://effect.website/docs/v4/api/effect/Effect |
| `Effect.catch`, `Effect.catchTag` (array form), `Effect.catchDefect`, `Effect.catchCause`, `Effect.exit`, `Exit.succeed`, `Exit.fail`, `Cause` | V | https://effect.website/docs/v4/error-management/two-error-types; https://unpkg.com/effect@4.0.0-rc.117/AGENTS.md |
| `Effect.fn("name")`, `Effect.fnUntraced`, `Effect.fn.Return<A, E, R>` | V | https://unpkg.com/effect@4.0.0-rc.117/AGENTS.md; https://effect.website/docs/v4/api/effect/Effect `#fn-variable` |
| `Effect.tx`, `Effect.txRetry`, `Effect.Transaction` | V | https://effect.website/docs/v4/api/effect/Effect `#tx`, `#txRetry` |
| `TxRef.make`, `TxRef.get`, `TxRef.set`, `TxRef.TxRef` type | V | Effect API page transaction examples |
| `Schema.TaggedError<Err>()("Tag", { field: Schema.String })`, `Schema.Defect()`, `Schema.String`, `Schema.Int` | V | https://unpkg.com/effect@4.0.0-rc.117/AGENTS.md |
| `Schema.Number` (field constructor) | U | `Schema` module confirmed at https://effect.website/docs/v4/api/effect |
| `Data.TaggedError("Tag")<{ ... }>` | V | https://effect.website/docs/v4/getting-started/creating-effects |
| `Context.Service<Name, Iface>()("pkg/path/Name")`, `Name.of`, `Name["Service"]`, `Context.Reference`, `Context.make/get/add/merge/empty/getOption` | V | https://unpkg.com/effect@4.0.0-rc.117/AGENTS.md; https://effect.website/docs/v4/api/effect/Context |
| `Layer.effect`, `Layer.provide`, `Layer.provideMerge`, `Layer.unwrap`, `Layer.launch` | V | https://unpkg.com/effect@4.0.0-rc.117/AGENTS.md |
| `Effect.acquireRelease`, `Effect.withSpan`, `Effect.retry`, `Effect.repeat`, `NodeRuntime.runMain`, `ManagedRuntime`, `Effect.log` | V | https://unpkg.com/effect@4.0.0-rc.117/AGENTS.md |
| `Stream.fromIterable`, `Stream.fromEffectSchedule`, `Stream.paginate`, `Stream.fromAsyncIterable`, `Stream.callback`, `Stream.map`, `Stream.flatMap`, `Stream.filter`, `Stream.mapEffect`, `Stream.run*` | V (existence; `flatMap` concatenation semantics expected but unverified) | https://unpkg.com/effect@4.0.0-rc.117/AGENTS.md |
| `Effect.andThen` (used in the 3.1 accumulator sketch) | U | module exists (API index) |
| `Ref.make`, `Ref.get`, `Ref.update`, `Ref.updateAndGet`, `SynchronizedRef` members | U | `Ref`, `SynchronizedRef` modules confirmed at https://effect.website/docs/v4/api/effect |
| `Semaphore.make`, `Match.type`/`when`/`orElse`, `Schedule.spaced`, `Random.nextInt`, `Stream.scan`/`take`/`interleave`, `Result` constructors, `Chunk` members | U | modules confirmed at the same index |
| `Queue` members; `Queue.shutdownUnsafe` added and shutdown ops return `false` when already shut down (rc.117 #8318) | U (members) / V (release note) | https://effect.website/docs/v4/api/effect; https://github.com/Effect-TS/effect/releases/tag/effect%404.0.0-rc.117 |
| `Effect.log`, `Console.log`, `Duration.toMillis`, `Deferred.succeed`, `Deferred.await` | V | pages and AGENTS.md as above |
| Modules `Result`, `Option`, `Chunk`, `Match`, `Ref`, `SynchronizedRef`, `Semaphore`, `Latch`, `Queue`, `Deferred`, `Schedule`, `Layer`, `Scope`, `Stream`, `Fiber`, `Context`, `Schema`, `Data`, `HashMap`, `HashSet`, `Random`, `Clock`, `Pool`, `PubSub`, `TestClock`, `TestConsole`, `TestSchema`, `Tx*` family | V (existence) | https://effect.website/docs/v4/api/effect |
| `effect/unstable/arbitrary` (`Arbitrary`) | V (exists), unstable, not depended on | https://effect.website/docs/v4/api/effect/unstable/arbitrary/Arbitrary |
| `@effect/vitest` package; its `it.effect` helper | V (package) / U (helper) | https://effect.website/docs/v4/api/vitest |

Unverified / not found in v4, never to appear in the edition: `List` (persistent), `STM`, `TRef`, `Either`, `FastCheck` (not an Effect export), `Schema.Arbitrary` (no such export; the arbitrary module is `effect/unstable/arbitrary`). Property tests use the `fast-check` npm package directly.

## 1. Chapter 0 primer (about thirty pages, exercises 0.1-0.5)

| Section | Pages | What it teaches | Exercises |
|---|---|---|---|
| 0.1 Values, bindings, and numbers | 3 | `const`, primitives, guest `number` (binary64) with the `Number.MAX_SAFE_INTEGER` exactness limit, template strings; `bigint` appears only as an earlier host-only lesson and is never guest source (grammar §4.1) | — |
| 0.2 Functions, recursion, closures | 5 | arrow functions, recursion, closures, higher-order functions; V8 no-guaranteed-TCO note and how iterations are written | — |
| 0.3 Compound data | 5 | readonly tuples, records, discriminated unions with exhaustive `switch`, `Option` and `Result`, structural equality | 0.1, 0.2 |
| 0.4 The cons list | 4 | the hand-written `List<A>` union, `cons`/`car`/`cdr`, recursive `map`/`filter`/`accumulate`; why no library `List` exists in v4 | 0.1 (continued) |
| 0.5 Errors as values | 3 | no `throw`; `Schema.TaggedError`; `Result<A, E>`; preview of the `Effect` error channel | 0.3 |
| 0.6 A first taste of Effect | 4 | `Effect.gen`, `Effect.succeed/fail`, `Effect.fn`, `runSync`/`runPromise`; the chapters 1-2 purity rule and the 3.1 switch | 0.4 |
| 0.7 Tooling | 3 | the pnpm workspace, tsconfig flags, vitest with `@effect/vitest`, fast-check, examples/exercises/solutions | 0.5 |
| 0.8 Reading this book | 3 | listing naming (`NN-name.ts`), `;Value:` lines as `// =>` comments, exercises vs solutions, tailored `N.Ma` exercises | — |

Exercises: **0.1** implement `cons`/`car`/`cdr`/`list` for `List<A>`, fast-check round-trip `toList(list(...xs))`; **0.2** a `Shape` union with exhaustive `area`, delete a case, record the compiler error; **0.3** `safeDiv` as `Result<number, DivByZero>` then as `Effect<number, DivByZero>`, run both with `Effect.runSync`; **0.4** a `Ref`-based counter `makeCounter: () => Effect<() => Effect<number>>`, explain in three sentences why the pure data of 0.1-0.3 could not do this; **0.5** a `ch0` package with one `@effect/vitest` file and one fast-check property, red-then-green.

## 2. Part 1. Concept map

| SICP concept | Construct in this edition | Notes |
|---|---|---|
| procedure / lambda | arrow function; `const` | first-class closures |
| higher-order procedure | generic function | |
| linear/tree recursion | recursion; iteration = accumulators | V8 lacks guaranteed TCO; stated once in 1.2.1 |
| symbol / quoted program data | typed discriminated unions | guest syntax is explicit tagged data with source spans, never quoted executable lists (grammar §3) |
| truthiness | `boolean` only | conditions/operands must be `boolean`; no truthiness (grammar §2) |
| pair (immutable) | `readonly [A, B]` | 2.1, 2.2 |
| list | hand-written `List<A>` union | no v4 `List` module; `Chunk` shown once as library answer |
| mutable pair (3.3.1) | `PairRef<A, B>` class with `Ref` fields | no parameter properties |
| `map`/`filter`/`accumulate` | hand-built `List` functions over the tagged union; `Array` contrast | book order preserved |
| tree / symbolic data | discriminated unions | 2.2.2, 2.3 |
| message passing (2.1.3, 2.4.3) | closure returning a record of functions, or `(msg) => Result` via `Match` | pure in ch2 |
| data-directed dispatch (2.4) | `Map<string, Record<string, Fn>>` with `put`/`get` | mirrors the book's table; `Match` for closed unions |
| generic tower (2.5) | tagged union integer / rational / real / complex | `raise`/`project`/`drop` |
| `set!` (3.1) | `Ref` | the pure-to-Effect switch |
| environment | persistent `Env`: `HashMap` frame chain | 3.2 re-cut |
| queue / table (3.3) | `Ref`-backed; `Queue` module shown after hand build | |
| circuits / agenda (3.3.4) | `Ref` wires, agenda `Ref`, pure time counter | never wall clock |
| constraints (3.3.5) | `Ref` connectors + informer lists | |
| streams (3.5) | host memoized `Stream<A>` cells throughout the main lesson; cold Effect streams are a comparison | technical plan D18 preserves sharing; guest lazy/search experiments have separate contracts (grammar §6) |
| concurrency (3.4) | fibers, `Effect.all({concurrency})`, `Semaphore`, `Effect.tx` + `TxRef` | |
| evaluator (4.1) | `evaluate` over the shared tagged `Expr`/`Value`/`Env` AST returning `Outcome` data | three gates pass before effects; `Effect` only at the driver boundary (grammar §1-§3, §8) |
| lazy eval (4.2) | `lazy-memoized-experiment` / `lazy-recompute-experiment` with explicit delay/force AST extension | independent reference model; rejected by the core parser (grammar §6) |
| amb (4.3) | `amb-depth-first-experiment` with explicit choice/failure continuations; seeded `ramb` separately named | independent oracle; direct JS execution is never the oracle (grammar §6, §10) |
| query language (4.4) | typed query/rule/term constructors and `QueryError` data | domain values, not parser productions (grammar §7) |
| register machine (5.1-5.4) | `Machine`/`Instruction`/`Source` tagged unions with constructor functions; typed assembly and `MachineError` variants | labels/instructions stay ordered arrays (grammar §7-§8) |
| compiler (5.5) | exhaustive source-union compiler emitting typed instruction/controller data for the 5.4 machine; no JS generation, reflection, or `eval` | self-evaluator (5.50) and C backends (5.51-5.52) per grammar §11 |

Hard-mapping sketches (strict-clean: no `any`, no `!`, no casts, no enums, no namespaces, no parameter properties). The `Schema.TaggedError` and `TxRef` sketches below are host/driver teaching for chapters 1-3 and the process boundary; the guest evaluator kernel itself returns `Outcome`/`MachineError` data and performs no I/O (grammar §4.4, §8):

Cons list (built in 0.4 and 2.2):

```ts
export type List<A> = { readonly _tag: "Nil" } | { readonly _tag: "Cons"; readonly head: A; readonly tail: List<A> }
export const nil: List<never> = { _tag: "Nil" }
export const cons = <A>(head: A, tail: List<A>): List<A> => ({ _tag: "Cons", head, tail })
export const list = <A>(...xs: ReadonlyArray<A>): List<A> =>
  xs.reduceRight<List<A>>((t, h) => cons(h, t), nil)
```

Errors (from 1.7 on wherever the book says "signal an error"):

```ts
import { Schema } from "effect"
export class UnboundVariable extends Schema.TaggedError<UnboundVariable>()("UnboundVariable", {
  name: Schema.String
}) {}
```

State, the 3.1 switch (update, then read, then return the number; `Ref.*` and `Effect.andThen` are in the U set):

```ts
import { Effect, Ref } from "effect"
export const makeAccumulator = (n: number): Effect.Effect<(x: number) => Effect.Effect<number>> =>
  Effect.gen(function* () {
    const sum: Ref.Ref<number> = yield* Ref.make(n)
    return (x: number): Effect.Effect<number> =>
      Ref.update(sum, (s) => s + x).pipe(Effect.andThen(Ref.get(sum)))
  })
```

Mutable pair, 3.3.1, generic and without parameter properties:

```ts
import { Ref } from "effect"
export class PairRef<A, B> {
  readonly car: Ref.Ref<A>
  readonly cdr: Ref.Ref<B>
  constructor(car: Ref.Ref<A>, cdr: Ref.Ref<B>) {
    this.car = car
    this.cdr = cdr
  }
}
```

Concurrency, 3.4 bank exchange (all names in the V set):

```ts
import { Effect, TxRef } from "effect"
const exchange = (a: TxRef.TxRef<number>, b: TxRef.TxRef<number>): Effect.Effect<void> =>
  Effect.tx(Effect.gen(function* () {
    const from: number = yield* TxRef.get(a)
    const to: number = yield* TxRef.get(b)
    yield* TxRef.set(a, from - 1)
    yield* TxRef.set(b, to + 1)
  }))
```

Chapter 3+ reusable functions use `Effect.fn("name")(function* ...)` for tracing, `Effect.fnUntraced` for hot paths.

## 3. Part 2. Per-section notes (all 22 sections)

**1.1 The elements of programming (1107-2629).** Changes: applications are infix calls; bindings are `const`, procedure definitions are `const` arrow functions or `function` declarations; substitution-model prose unchanged. Representative: `sqrt` by Newton's method (1.1.7). Hard spot: ex 1.5 (evaluation order) reframed around thunks and TS's left-to-right argument evaluation. Tailored: 1.5a (a `lazyArg` thunk wrapper showing eager vs lazy argument passing under core strictness, grammar §6); 1.1a (which host-valid forms the subset gate rejects, e.g. `eval`, classes, assertions, with `UnsupportedSyntax`/`ForbiddenHostPrimitive` categories, grammar §8).

**1.2 Procedures and processes (2630-4191).** Changes: none structural. Representative: `fib`, `countChange`, `fastExpt`, `gcd`, Fermat/Miller-Rabin. Hard spot: 1.9-1.20 substitution counting stays. Number note: every primality example printed in the book (199, 1999, 19999, the Carmichael numbers) stays far below 2^53; `bigint` is chosen in these chapter 1-2 host lessons anyway so modular exponentiation stays exact for arbitrary n (Miller-Rabin). These host numeric choices live outside the guest core, which is binary64 `number` only with exactness bounded by `Number.MAX_SAFE_INTEGER` (grammar §4.1; no `bigint`, no number/bigint conversions). Tailored: 1.28a (Miller-Rabin with `bigint`, property-tested against a sieve); 1.19a (log-step Fibonacci as 2x2 tuple matrix power).

**1.3 Higher-order procedures (4192-5904).** Changes: `sum`/`product`/`accumulate` as generics; `fixedPoint`, `newtonsMethod` take `(x: number) => number`. Representative: average-damped `fixedPoint`. Hard spot: 1.41-1.43 procedure-returning procedures are the typing moment. Tailored: 1.37a (continued fraction with a `Result` stopping at k terms); 1.46a (a generic `IterativeImprovement<A>`).

**2.1 Data abstraction (5905-6780).** Changes: rationals on `readonly [bigint, bigint]` with gcd normalization (host lesson preserving the exact-arithmetic objective; exact integer pairs live outside the guest core, which is binary64 `number` only per grammar §4.1); intervals on `{ readonly lo: number; readonly hi: number }`. Representative: `makeRat`/`addRat`. Hard spot: 2.4 (procedural pairs) and 2.6 (Church numerals) translate exactly. Tailored: 2.6a (Church numerals typed as `(f: (n: number) => number) => ...`); 2.16a (the interval dependency problem exhibited with a fast-check property).

**2.2 Hierarchical data (6781-9607).** Changes: the `List` union is built here; `map`, `reverse`, `fringe`, `append`. 2.2.1's dotted-tail notation becomes rest parameters. Picture language (2.2.4): `Painter = (frame: Frame) => string` emitting SVG; figures render from `fig/chap2/` snapshots. Representative: `fringe`, `squareTree`, `cornerSplit` rendered to SVG. Hard spot: 2.22's wrong-order accumulator still reproduces. Tailored: 2.42a (eight queens via a generator plus fast-check validator); 2.49a (snapshot-test rendered SVG painters).

**2.3 Symbolic data (9608-11233).** Changes: symbolic programs are discriminated unions with exhaustive `switch` dispatch; syntax is explicit typed data, never hidden in string conventions (grammar §3, §11). Representative: `deriv`. Hard spot: 2.55 becomes the sealed-variant exhaustiveness lesson (see divergences). Tailored: 2.56a (delete a switch arm, let the compiler flag non-exhaustiveness); 2.69a (Huffman with property "decode(encode(m)) = m").

**2.4 Multiple representations (11234-12309).** Decision: the operation table is a `Map<string, Record<string, Fn>>` keyed by op and type with `put`/`get`, because the section teaches open extension; `Match` on closed unions is the prose contrast. 2.4.3 message passing: closure returning `(msg: Message) => Result<Value, string>`. Representative: `applyGeneric` over rational and rectangular/polar complex. Hard spot: 2.77-2.80 coercion tower unchanged. Tailored: 2.76a (add a type under each of the three strategies, count edits, let the compiler count misses); 2.78a (native `number` plus tags as the host-type-system trick).

**2.5 Generic arithmetic (12310-14038).** Changes: tower as a tagged union with `raise`/`project`/`drop` (2.83-2.85); polynomials on `List<Term>` with `bigint` coefficients (host lesson preserving the book's exact-arithmetic objective; the guest core stays binary64 `number` only per grammar §4.1, with stated approximation tolerance where `Math.sqrt` is used). Representative: coercion-searching `applyGeneric`. Hard spot: 2.93-2.97 polynomial GCD and reduction, heaviest in the book. Tailored: 2.93a (property: `rem(p, gcd(p, q))` is zero); 2.97a (polynomial `reduce` with a round-trip property).

**3.1 Assignment and local state (14039-15025).** The pure-versus-Effect switch happens here. Rule (confirmed, refined): chapters 1-2 are pure functions plus immutable data (`Option`, `Result`, unions, the hand `List`); `Effect` appears only in test harnesses. From 3.1 on, assignment, implicit time, randomness, and processes are modeled with `Ref`, `Clock`/`Random`, and fibers: assignment writes a `Ref` cell that every aliasing closure observes, because `Ref` makes shared mutation visible in the type, the chapter's own lesson. 3.1: `Ref`; `SynchronizedRef` where updating requires an `Effect` (used at 3.27 memoization). Representative: password-protected `makeAccount`. Hard spot: ex 3.8 evaluation order is a JS-spec question, demonstrated with two `Ref`s. Tailored: 3.1a (rewrite the accumulator with a closure `let` and show what the type no longer admits); 3.7a (joint accounts sharing one `Ref`, `Result` password errors).

**3.2 The environment model (15026-15998), re-cut.** Becomes "closures and the `Ref` model": environment diagrams are replaced by closure captures and `Ref` identities; `type Env = { readonly vars: HashMap<string, Value>; readonly parent: Env }` is introduced here for chapters 3-4. Exercises 3.9-3.11 keep their numbers; their answers write out `Env`/`Ref` values instead of drawing. Representative: environment chains as ASCII for `makeAccount`. Hard spot: making "two closures over one `Ref`" visually obvious. Tailored: 3.10a (implement and test `Env.lookup` on the 3.10 example); 3.11a (property: two accounts share no `Ref`).

**3.3 Mutable data (15999-18790).** Changes: mutable pairs are `PairRef` with `Ref` fields; queues with front/rear `Ref`s; tables `Ref<HashMap>`; agenda with a pure time counter (3.3.4); constraint network with `Ref` connectors (3.3.5). Representative: half-adder simulation. Hard spot: 3.16-3.19 cycle exercises work as-is; the agenda must never use wall-clock time. Tailored: 3.23a (deque with fast-check model-based tests against an array oracle); 3.31a (agenda stepped by a pure counter, compared against a `Schedule`-driven variant).

**3.4 Concurrency (18791-19854), re-cut.** Becomes "fibers, `Semaphore`, and `Effect.tx` with `TxRef`": processes are fibers run as `Effect.all([...], { concurrency: "unbounded" })`; serializers are `Semaphore`; the mutex-from-atomic-compare-and-set discussion is kept conceptually, then the bank account and `serializedExchange` are rebuilt on `Effect.tx` + `TxRef`, after first exhibiting interleaving anomalies with jittered `Effect.sleep` between read and write. Exercises keep their numbers. Representative: Peter/Paul/Mary, then the transactional exchange. Hard spot: 3.38-3.49 are conceptual and stay. Tailored: 3.43a (run the exchange under jitter 1000 times, count violations, then under `Effect.tx`, count zero); 3.47a (build a size-n semaphore from two `Ref`s, compare against the `Semaphore` module).

**3.5 Streams (19855-22671).** The main lesson uses the existing memoized `Stream<A>` from `packages/ch3/src/05-streams.ts`. A cell has an eager head and a tail whose first force stores the result. Later forces reuse it. Keep this representation through implicit definitions, merging, weighted pairs, and interleaving, as required by technical plan D18. Cold Effect streams are a comparison, not a replacement for shared tails. Fair enumeration must alternate suspended branches rather than exhaust an infinite branch first. This ordinary host-stream lesson does not require a guest search experiment. Representative programs are `fibs` and weighted `pairs`. Tailored ideas: 3.53a distinguishes the safe-integer bound from exact representation of finite powers of two, finds the overflow to `Infinity`, and uses host `bigint` for unbounded exact terms; 3.66a compares finite prefixes from fair pair enumeration. Guest nondeterministic search remains the separately admitted chapter-4 experiment with its independent oracle.

**4.1 Metacircular evaluator (22672-24871).** Changes: guest TypeScript source implements the contract of `spec/host-subsets/typescript/grammar.md` §§1-3: `read`/`readAll`/`readProgram` lower the same source to the shared tagged `Expr` AST with spans and `UnsupportedSyntax` rejections before effects; `evaluate` returns `Outcome` data (`ok`/`error`: unbound name, non-callable, arity, bad operand, unknown node); `Effect` and `NodeRuntime` remain at the driver/error boundary only; `analyze` (4.1.7) returns a precompiled closure. Chapter 5 source checking, the explicit-control evaluator, the compiler, guest self-interpretation (§9 kernel; §11 5.50), and the C exercise objectives (5.51-5.52) remain mandatory. Representative: the `eval` dispatch switch, exhaustive over `Expr`. Hard spot: ex 4.14 (installing host `map` as a primitive) becomes the function-representation mismatch lesson. Tailored: 4.3a (rebuild dispatch on the 2.4-style table, compare extensibility); 4.16a (scan-out-defines as an AST transform tested on the book's examples).

**4.2 Lazy evaluation (24872-25621).** Changes: lazy behavior lives only in `lazy-memoized-experiment` (and distinctly in `lazy-recompute-experiment`): an explicitly marked delay/force AST extension captures its environment, first force stores the value, later forces return it; the extension is rejected by the core parser and carries its own finite reference model and oracle (grammar §6). Representative: lazy lists of 4.2.3. Hard spot: 4.27-4.31 counting forces with an explicit cell. Tailored: 4.29a (memoized vs recompute force counts as a table); 4.31a (`lazy` parameter annotation as an AST flag on `Param`).

**4.3 Nondeterministic computing (25622-27115), partial re-cut.** Amb lives only in `amb-depth-first-experiment`: the separate `choose`, `require`, and failure-continuation AST extension runs alternatives left-to-right depth-first with an explicit backtrackable-state policy; the randomized `ramb` variant needs an injected deterministic seed and a separate name (grammar §6). Expectations come from the independent finite reference model, never from direct JS execution (grammar §10). Representative: Pythagorean triples, multiple dwelling. Hard spot: 4.45-4.49 parsing over a token array. Tailored: 4.35a (solve the same puzzles inside the named experiment and compare against the reference model); 4.49a (sentence generation as six takes from the experiment driver).

**4.4 Logic programming (27116-30357).** Changes: unification over typed query/rule/term constructors with record patterns, arrays, and maps; frames are typed data; unknown operation names, bad operand types, and bounds are `QueryError` variants (grammar §7-§8). Any retained query/controller text parser must consume the same typed model and be removed when its callers migrate. Representative: `simpleQuery`, `not`, `lispValue`. Hard spot: 4.64-4.69 loops; 4.74-4.79 stream mechanics. Tailored: 4.62a (`lastPair` rule in both directions, note the divergent case); 4.77a (delayed filtering via an explicit delayed-filter field on frames).

**5.1 Register machines (30358-31561).** Changes: machines are `Machine`/`Instruction`/`Source` tagged unions built by constructor functions (e.g. `queryAtom`, `register`, `constant`); labels and instructions stay ordered arrays; data-path diagrams stay ASCII (grammar §7). Representative: iterative factorial and Fibonacci controllers. Hard spot: 5.5 hand simulation with stacks. Tailored: 5.4a (an `expt` machine with `number` and integer-domain checks, stating the binary64 limit); 5.6a (confirm the removable save/restore by diffing stack traces of both controllers).

**5.2 Register-machine simulator (31562-32785).** Changes: typed assembly passes resolve labels to indices; registers and stack are explicit mutable state stepped by an index loop; tracing and counting are ordered event arrays; assembly and runtime failures are exhaustive `MachineError` variants (duplicate/unknown label, unknown register/operation, bad target, stack underflow/mismatch, out-of-steps), never exceptions (grammar §7-§8). `Effect` remains at the driver boundary only. Representative: `makeMachine(ops, controller)` running factorial. Hard spot: 5.8-5.13 robustness exercises. Tailored: 5.12a (data-path summary extracted as an immutable record after assembly); 5.14a (pushes and max stack depth as ordered machine outputs, plotted over fast-check-generated n).

**5.3 Storage allocation and GC (32786-33543), re-cut in framing.** The host is garbage-collected, so the section is taught as "rebuilding by hand what V8 does for you": the vector memory (explicit numeric indices, mutable `number[]` vectors, tagged pair/empty-list words, checked allocation and selectors under binary64 rules) and the stop-and-copy collector of 5.3.2 are kept as a simulation because chapter 5's explicit-control evaluator and compiler exercises run on them; the "infinite memory illusion" motivation is restated against V8's real collector. Exercises 5.20-5.22 keep their numbers. Representative: vector-based `cons`/`car`/`cdr`. Hard spot: forcing a collection mid-construction for tests. Tailored: 5.20a (ordered memory-vector index maps for the 5.20 structures); 5.22a (`append!` splicing at the vector level, property: equals pure `append`).

**5.4 Explicit-control evaluator (33544-34640).** Changes: eceval uses the same typed evaluator syntax and value unions as 4.1; the controller is machine data; frames, continuation state, stack, input, and transcript are explicit records/arrays; failures are typed errors including the checked 5.30 register (grammar §11). Source checking before execution remains mandatory. Representative: `eval-dispatch` controller text. Hard spot: tail recursion verified by stack-depth stats (5.26-5.28). Tailored: 5.24a (explicit `cond` handling in the controller data, contrasted with 4.1 derived expressions); 5.30a (typed error register plus a checked `goto`).

**5.5 Compilation (34641-end).** Changes: the exhaustive source-union compiler emits typed instruction-sequence/controller data (`compile`, `preserving`, open coding at 5.38, lexical addresses `{ frame, disp }` at 5.39-5.44) and runs through the 5.4 machine with no JavaScript source generation, reflection, or `eval` in the core compiler; `compile-and-go` feeds the eceval machine. The 5.50 self-evaluator (checked subset source run as a guest program on the compiled machine, result and transcript compared) and the 5.51-5.52 C objectives (C text as a generated artifact string via the boundary harness, never guest source) remain mandatory (grammar §11). Representative: compiled factorial. Hard spot: 5.31-5.32 save/restore analysis. Tailored: 5.35a (render compiled sequences with indented labels, snapshot-tested); 5.51a (translate the explicit-control machine/controller to the exercise C representation; compare the C process transcript with the pinned input/reference cases, grammar §11).

### Re-cut and divergence table (numbering policy: SICP exercise numbers kept 1:1; tailored additions use `N.Ma`)

| Where | Change | Exercise impact |
|---|---|---|
| Chapter 0 (new) | ~30-page primer, sections 0.1-0.8, exercises 0.1-0.5 | none (front matter) |
| 1.2 | `bigint` kept for exact modular arithmetic in host lessons though printed examples stay below 2^53; guest core is `number`-only | statements unchanged; host solvers use `bigint`, guest programs use `number` (grammar §4.1) |
| 2.2.4 | painters render to SVG; figures snapshot under `fig/chap2/` | none |
| 2.55 | quoted-list mechanics replaced by sealed-variant exhaustiveness over typed syntax data | slot kept at 2.55, tailored 2.55a added (grammar §3) |
| 3.2 | re-cut: closures + `Ref` model replace drawn environment diagrams | 3.9-3.11 keep numbers; answer medium changes |
| 3.4 | re-cut: fibers, `Semaphore`, `Effect.tx`/`TxRef` replace the test-and-set mutex path (kept conceptually) | exercises keep numbers |
| 3.5 | memoized host `Stream<A>` retains sharing throughout; cold Effect streams remain a comparison | exercises keep their stream objectives; guest search remains a separate chapter-4 experiment |
| 4.3 | amb via the named `amb-depth-first-experiment` (choice/failure continuations, seeded `ramb` separate) | independent oracle; 4.35a stays inside the experiment (grammar §6, §10) |
| 5.3 | re-cut framing: manual memory as a simulation of V8's GC | 5.20-5.22 keep numbers |
| 5.51, 5.52 | C evaluator/compiler-backend objectives kept: the compiler emits C from typed controller data and the C text is a generated artifact string via the Node filesystem/process boundary harness | exercises keep numbers; no generated C is a guest primitive (grammar §11) |

## 4. Part 3. Edition conventions

- **Numbers**: chapters 1-3 host lessons use `number` by default and `bigint` where exactness demands it: modular arithmetic in 1.24-1.28, the 2^a·3^b pairs of 2.5, polynomial coefficients in 2.5.3, the 3.53a stream. The guest core of grammar §4.1 is binary64 `number` only (no `bigint`, no number/bigint conversions; exactness bounded by `Number.MAX_SAFE_INTEGER`; `1 / 0` infinity, `0 / 0` `NaN`). Admitted guest `Math` calls are `abs`, `floor`, `max`, `min`, `sqrt`, `trunc` plus `Number.isInteger`; `Math.random` and wall-clock/time sources are excluded from guest programs entirely, including the named experiments; randomized search uses only an injected deterministic seed (grammar §4.1, §6). Chapter 3 host `Clock`/`Random` stay separate host teaching. Chapter 5 indices stay `number`. Stated in 1.1.6, recalled at each site.
- **Pairs and lists**: immutable fixed-arity pairs are readonly tuples; the cons list is the hand-written `List<A>` union; mutable pairs (3.3.1) are `PairRef<A, B>` with `Ref` fields, no parameter properties.
- **Pure versus Effect rule**: chapters 1-2 are pure functions returning values, `Result`, or `Option`; `Effect` appears only in test harnesses. **The switch is at section 3.1**: assignment, time, randomness, and concurrency become `Ref`, `Clock`/`Random`, fibers from there on. Rationale: chapters 1-2 have no effects to model, and wrapping `fib` in `Effect.gen` would bury the substitution model; from 3.1 the book changes subject to effects, and `Ref`'s type-visible mutation is the payoff.
- **Errors**: host application and driver code signal failures as values: domain errors are `Schema.TaggedError` classes; `Data.TaggedError` mentioned once as the lighter alternative. Chapters 1-2 failures return `Result<A, E>`; chapter 3+ host code uses the `Effect` error channel. `throw`/`try`/`catch`/`new Error` are admitted only at the host I/O and process boundary (driver assertions, witness tail), never inside the guest evaluator kernel (grammar §1.1, §4.4). Guest evaluator, query, and machine failures are typed data (`Outcome` with unbound-name, non-callable, arity, bad-operand, and unknown-node variants; `QueryError`/`MachineError`), never thrown exceptions (grammar §8).
- **Reusable effectful functions**: `Effect.fn("name")(function* ...)` traced, `Effect.fnUntraced` for hot paths.
- **Tests**: vitest `5.0.1` with `@effect/vitest@4.0.0-rc.117`; chapters 1-2 plain `it`/`expect`, chapter 3+ effect-aware runner. Property tests use `fast-check` with hand-written generators; `effect/unstable/arbitrary` noted in an appendix box, not depended on.
- **Listings**: `packages/chN/src/NN-name.ts` in section order; `;Value:` lines become `// => result` comments; chapter 3+ REPL sessions as `Console` transcript blocks; rendered figures under `fig/chapN/`.
- **Workspace and pins** (inside `typescript/`, aligned with the shared plan's `book/` and root dirs):

```
typescript/
  .node-version              # 24.21.0
  pnpm-workspace.yaml        # packages: ["packages/*", "book"]
  package.json               # engines: { "node": "24.21.0" }, packageManager: "pnpm@12.5.1"
                             # devDeps: typescript@7.0.2, vitest@5.0.1,
                             #   @effect/vitest@4.0.0-rc.117, fast-check, @biomejs/biome@2.5.14
  biome.json
  tsconfig.base.json
  book/                      # shared prose and figure sources (per shared plan)
  examples/                  # root-level runnable demos (per shared plan)
  exercises/                 # root-level exercise index (per shared plan)
  solutions/                 # root-level solution index (per shared plan)
  packages/
    ch0/ ch1/ ch2/ ch3/ ch4/ ch5/
      package.json           # "@sicp-ts/chN", deps: { "effect": "4.0.0-rc.117",
                             #              "@effect/vitest": "4.0.0-rc.117" }
      src/                   # book listings, numbered
      examples/ exercises/ solutions/   # per-chapter tests and worked solutions
  fig/chapN/                 # rendered SVG snapshots
```

CI runs `solutions/` tests and asserts every `exercises/` file exists with matching test names.

- **tsconfig.base.json, exact flags** (the guest type-checking authority, grammar §1):

```json
{
  "compilerOptions": {
    "target": "es2024",
    "lib": ["es2024"],
    "module": "nodenext",
    "moduleResolution": "nodenext",
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "noImplicitOverride": true,
    "noFallthroughCasesInSwitch": true,
    "noPropertyAccessFromIndexSignature": true,
    "noImplicitReturns": true,
    "allowUnreachableCode": false,
    "verbatimModuleSyntax": true,
    "erasableSyntaxOnly": true,
    "allowImportingTsExtensions": true,
    "noEmit": true,
    "skipLibCheck": true
  }
}
```

No `any`, no non-null assertions, no `as unknown as`, no enums, no namespaces anywhere. Biome owns lint and format.

## 5. Part 4. Architecture sketches

Chapter 4 evaluator (`packages/ch4/src/`), conceptual description against the contract (exact migrated paths land with the implementation; the semantic contract and grammar §3/§8 govern):

```ts
import type { List } from "./list.js"

// errors.ts — typed `Outcome`/`EvalError` data (conceptual; exact field shapes land with the implementation)
export type EvalError =
  | { readonly tag: "unbound-name"; readonly name: string }
  | { readonly tag: "not-callable" }
  | { readonly tag: "arity-mismatch"; readonly expected: number; readonly given: number }
  | { readonly tag: "bad-operand"; readonly operator: string }
  | { readonly tag: "unknown-node" };
export type Outcome = { readonly tag: "ok"; readonly value: Value } | { readonly tag: "error"; readonly error: EvalError };

// Published syntax path `packages/ch4/src/syntax/ast.ts` — node shapes below are conceptual; exact migrated node names land with the implementation.
// Minimum core nodes per grammar §3: literals, variable reads, arrays/objects, unary/binary
// operations, conditionals, assignment, blocks/sequences, const/let bindings, functions, calls,
// if/while/for-of/return, and the boundary-only try/throw forms. Each node records its span;
// dispatch switches are exhaustively typed. The §9 kernel (number/variable/add/subtract/multiply/
// lessOrEqual/if/lambda/call/letrec) is the minimum positive witness, not the full node set.
type Value = { readonly tag: "number"; readonly n: number } | { readonly tag: "boolean"; readonly b: boolean } | Closure | { readonly tag: "cons"; readonly head: Value; readonly tail: Value } | { readonly tag: "nil" }
// Closures capture body plus lexical environment; pair/list shapes are recursive tagged objects (grammar §3). No `bigint`, no `unknown` payload, no unchecked casts.
interface Env { readonly bindings: ReadonlyArray<Binding>; readonly parent: Env | null }
// evaluate: (expr, env) => Outcome;  analyze: (expr) => (env) => Outcome
// modules (semantic contract; exact paths land with the implementation): shared tagged syntax, env with shared cells, primitives table, driver boundary,
//          lazy/* (named delay/force experiment only), amb/* (named choice experiment only), query/* (typed constructors)
```

Chapter 5 simulator (`packages/ch5/src/02-simulator.ts`), conceptual description against the contract (exact migrated paths land with the implementation; grammar §7-§8 govern):

```ts
// Assemble/runtime failures are exhaustive `MachineError`/`QueryError` data, never thrown host exceptions

type Instr = { _tag: "Assign"; reg: string; src: Src } | { _tag: "Test"; op: string; args: readonly Arg[] }
  | { _tag: "Branch"; label: string } | { _tag: "Goto"; target: LabelTarget }
  | { _tag: "Save"; reg: string } | { _tag: "Restore"; reg: string }
  | { _tag: "Perform"; op: string; args: readonly Arg[] }
type Arg = { _tag: "Reg"; name: string } | { _tag: "Const"; value: Value } | { _tag: "LabelRef"; label: string }
type TranscriptEvent = { readonly tag: "output"; readonly text: string } | { readonly tag: "counter"; readonly name: string; readonly value: number }; // conceptual: ordered machine-output events (grammar §7-§8); exact published shape lands with the implementation
type Machine = { readonly regs: Map<string, Cell>; readonly stack: Value[]; readonly ops: Map<string, (...a: ReadonlyArray<Value>) => Value>; readonly transcript: ReadonlyArray<TranscriptEvent> }
// assemble(controller) => typed result with `MachineError`; labels resolve to indices
// execution: index-stepped loop; tracing/counting are ordered event arrays; `Effect` only at the driver boundary
// modules (semantic contract): 02-simulator.ts, 03-storage.ts (5.3 numeric indices, `number[]` vectors, tagged words, checked allocation),
//          04-eceval.ts (5.4 controller), 05-compilation.ts (5.5), stack instrumentation in the machine
```

Chapter 5 compiler (`packages/ch5/src/05-compilation.ts`), conceptual description against the contract (grammar §11 governs):

```ts
type Linkage = "Next" | "Return"
type CtEnv = readonly (readonly string[])[]
type InstructionSeq = { readonly instrs: readonly Instr[]; readonly needs: ReadonlySet<string> }
const compile: (expr: Expr, target: string, linkage: Linkage, env: CtEnv) => InstructionSeq
const preserving: (regs: readonly string[], seq: InstructionSeq) => InstructionSeq // save/restore elision
const compileLinkage: (l: Linkage) => InstructionSeq
const compileIf, compileLambda, compileApplication, compileOpenCode // 5.38
type CompileError = { readonly tag: "not-found"; readonly name: string } | { readonly tag: "unsupported"; readonly construct: string }; // conceptual error shape; exact published shape lands with the implementation
const lexicalAddress: (name: string, env: CtEnv) => { readonly tag: "ok"; readonly value: { readonly frame: number; readonly disp: number } } | { readonly tag: "error"; readonly error: CompileError } // 5.39-5.42
// entry: compileAndGo(expr) = assemble(compile(expr, ...)) then eceval.run(); no generated JS, no `eval`, no reflection
```

## 6. Anchor files

- `spec/host-subsets/typescript/grammar.md`: normative guest contract (§§1-11) — three gates, shared tagged AST, named experiments, §9 kernel, §10-§11 oracles and lesson map
- `local://modern-sicp-typescript-consumer-contract.md`: stable engine and exchange paths — `packages/ch4/src/syntax/`, `packages/ch4/src/read.ts`, `packages/ch5/src/04-eceval.ts` (re-exports `readProgram`/`parse`), engines with parse-then-subset-then-pinned-`tsc`, `packages/ch4/src/host-evaluator-witness.ts` (§9 kernel), `spec/host-subsets/typescript/witnesses/metacircular-evaluator.ts`, C backends `eceval_5_51.c` / `metacircular_backend_5_52.c`, ch2 rename table
- `typescript/tsconfig.base.json` with `spec/host-subsets/typescript/tsconfig.json`: pinned checker authority and native typing gate (`tsc --noEmit`, Node 24.21.0 type stripping per grammar §10)
- `typescript/pnpm-workspace.yaml`, `typescript/package.json`, `typescript/.node-version`: workspace root for `packages/ch0..ch5` and `book`; pinned toolchain (TypeScript 7.0.2, Node 24.21.0, pnpm 12.5.1)