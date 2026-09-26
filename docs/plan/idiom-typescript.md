# TypeScript edition (Effect v4): idiom map and build-plan input

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
| 0.1 Values, bindings, and numbers | 3 | `const`, primitives, `number` vs `bigint` and the 2^53 boundary, template strings as symbols | — |
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
| symbol / quote | `string` | quoted metacircular data is `"x"` |
| truthiness | `boolean` only | removes Scheme truth puzzles |
| pair (immutable) | `readonly [A, B]` | 2.1, 2.2 |
| list | hand-written `List<A>` union | no v4 `List` module; `Chunk` shown once as library answer |
| mutable pair (3.3.1) | `PairRef<A, B>` class with `Ref` fields | no parameter properties |
| `map`/`filter`/`accumulate` | reader-built `List` functions; `Array` contrast | book order preserved |
| tree / symbolic data | discriminated unions | 2.2.2, 2.3 |
| message passing (2.1.3, 2.4.3) | closure returning a record of functions, or `(msg) => Result` via `Match` | pure in ch2 |
| data-directed dispatch (2.4) | `Map<string, Record<string, Fn>>` with `put`/`get` | mirrors the book's table; `Match` for closed unions |
| generic tower (2.5) | tagged union integer / rational / real / complex | `raise`/`project`/`drop` |
| `set!` (3.1) | `Ref` | the pure-to-Effect switch |
| environment | persistent `Env`: `HashMap` frame chain | 3.2 re-cut |
| queue / table (3.3) | `Ref`-backed; `Queue` module shown after hand build | |
| circuits / agenda (3.3.4) | `Ref` wires, agenda `Ref`, pure time counter | never wall clock |
| constraints (3.3.5) | `Ref` connectors + informer lists | |
| streams (3.5) | thunk `LazyList` in 3.5.1-3.5.2; Effect `Stream` from 3.5.3 | |
| concurrency (3.4) | fibers, `Effect.all({concurrency})`, `Semaphore`, `Effect.tx` + `TxRef` | |
| evaluator (4.1) | `evaluate: (expr, env) => Effect<Value, EvalError>` | `Schema.TaggedError` errors |
| lazy eval (4.2) | `Value` variant holding a memoized thunk | |
| amb (4.3) | search `Stream` with an explicit fair merge | CPS version is tailored 4.35a |
| query language (4.4) | `Stream<Frame>` | |
| register machine (5.1-5.4) | `Ref` registers, array memory, index step loop | |
| compiler (5.5) | `compile` returning instruction sequences for the ch5 machine | |

Hard-mapping sketches (strict-clean: no `any`, no `!`, no casts, no enums, no namespaces, no parameter properties):

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

**1.1 The elements of programming (1107-2629).** Changes: prefix becomes infix; `define` becomes `const`; substitution-model prose unchanged. Representative: `sqrt` by Newton's method (1.1.7). Hard spot: ex 1.5 (evaluation order) reframed around thunks and TS's left-to-right argument evaluation. Tailored: 1.5a (a `lazyArg` thunk wrapper showing eager vs lazy argument passing); 1.1a (which of 1.1's Scheme prints TS cannot reproduce, e.g. function equality).

**1.2 Procedures and processes (2630-4191).** Changes: none structural. Representative: `fib`, `countChange`, `fastExpt`, `gcd`, Fermat/Miller-Rabin. Hard spot: 1.9-1.20 substitution counting stays. Number note: every primality example printed in the book (199, 1999, 19999, the Carmichael numbers) stays far below 2^53; `bigint` is chosen anyway so modular exponentiation stays exact for arbitrary n (Miller-Rabin) and for the 2.5 and 3.5 sites where exactness genuinely breaks. Tailored: 1.28a (Miller-Rabin with `bigint`, property-tested against a sieve); 1.19a (log-step Fibonacci as 2x2 tuple matrix power).

**1.3 Higher-order procedures (4192-5904).** Changes: `sum`/`product`/`accumulate` as generics; `fixedPoint`, `newtonsMethod` take `(x: number) => number`. Representative: average-damped `fixedPoint`. Hard spot: 1.41-1.43 procedure-returning procedures are the typing moment. Tailored: 1.37a (continued fraction with a `Result` stopping at k terms); 1.46a (a generic `IterativeImprovement<A>`).

**2.1 Data abstraction (5905-6780).** Changes: rationals on `readonly [bigint, bigint]` with gcd normalization; intervals on `{ readonly lo: number; readonly hi: number }`. Representative: `makeRat`/`addRat`. Hard spot: 2.4 (procedural pairs) and 2.6 (Church numerals) translate exactly. Tailored: 2.6a (Church numerals typed as `(f: (n: number) => number) => ...`); 2.16a (the interval dependency problem exhibited with a fast-check property).

**2.2 Hierarchical data (6781-9607).** Changes: the `List` union is built here; `map`, `reverse`, `fringe`, `append`. 2.2.1's dotted-tail notation becomes rest parameters. Picture language (2.2.4): `Painter = (frame: Frame) => string` emitting SVG; figures render from `fig/chap2/` snapshots. Representative: `fringe`, `squareTree`, `cornerSplit` rendered to SVG. Hard spot: 2.22's wrong-order accumulator still reproduces. Tailored: 2.42a (eight queens via a generator plus fast-check validator); 2.49a (snapshot-test rendered SVG painters).

**2.3 Symbolic data (9608-11233).** Changes: quoted expressions become discriminated unions; structural equality implemented by the reader; `deriv` is an exhaustive switch (`Match`-style); Huffman on unions. Representative: `deriv`. Hard spot: 2.55's quote mechanics have no analogue (see divergences). Tailored: 2.56a (delete a switch arm, let the compiler flag non-exhaustiveness); 2.69a (Huffman with property "decode(encode(m)) = m").

**2.4 Multiple representations (11234-12309).** Decision: the operation table is a `Map<string, Record<string, Fn>>` keyed by op and type with `put`/`get`, because the section teaches open extension; `Match` on closed unions is the prose contrast. 2.4.3 message passing: closure returning `(msg: Message) => Result<Value, string>`. Representative: `applyGeneric` over rational and rectangular/polar complex. Hard spot: 2.77-2.80 coercion tower unchanged. Tailored: 2.76a (add a type under each of the three strategies, count edits, let the compiler count misses); 2.78a (native `number` plus tags as the host-type-system trick).

**2.5 Generic arithmetic (12310-14038).** Changes: tower as a tagged union with `raise`/`project`/`drop` (2.83-2.85); polynomials on `List<Term>` with `bigint` coefficients. Representative: coercion-searching `applyGeneric`. Hard spot: 2.93-2.97 polynomial GCD and reduction, heaviest in the book. Tailored: 2.93a (property: `rem(p, gcd(p, q))` is zero); 2.97a (polynomial `reduce` with a round-trip property).

**3.1 Assignment and local state (14039-15025).** The pure-versus-Effect switch happens here. Rule (confirmed, refined): chapters 1-2 are pure functions plus immutable data (`Option`, `Result`, unions, the hand `List`); `Effect` appears only in test harnesses. From 3.1 on, everything the book models with `set!`, implicit time, randomness, or processes becomes `Ref`, `Clock`/`Random`, and fibers, because `Ref` makes shared mutation visible in the type, the chapter's own lesson. 3.1: `Ref`; `SynchronizedRef` where updating requires an `Effect` (used at 3.27 memoization). Representative: password-protected `makeAccount`. Hard spot: ex 3.8 evaluation order is a JS-spec question, demonstrated with two `Ref`s. Tailored: 3.1a (rewrite the accumulator with a closure `let` and show what the type no longer admits); 3.7a (joint accounts sharing one `Ref`, `Result` password errors).

**3.2 The environment model (15026-15998), re-cut.** Becomes "closures and the `Ref` model": environment diagrams are replaced by closure captures and `Ref` identities; `type Env = { readonly vars: HashMap<string, Value>; readonly parent: Env }` is introduced here for chapters 3-4. Exercises 3.9-3.11 keep their numbers; their answers write out `Env`/`Ref` values instead of drawing. Representative: environment chains as ASCII for `makeAccount`. Hard spot: making "two closures over one `Ref`" visually obvious. Tailored: 3.10a (implement and test `Env.lookup` on the 3.10 example); 3.11a (property: two accounts share no `Ref`).

**3.3 Mutable data (15999-18790).** Changes: mutable pairs are `PairRef` with `Ref` fields; queues with front/rear `Ref`s; tables `Ref<HashMap>`; agenda with a pure time counter (3.3.4); constraint network with `Ref` connectors (3.3.5). Representative: half-adder simulation. Hard spot: 3.16-3.19 cycle exercises work as-is; the agenda must never use wall-clock time. Tailored: 3.23a (deque with fast-check model-based tests against an array oracle); 3.31a (agenda stepped by a pure counter, compared against a `Schedule`-driven variant).

**3.4 Concurrency (18791-19854), re-cut.** Becomes "fibers, `Semaphore`, and `Effect.tx` with `TxRef`": processes are fibers; `parallel-execute` is `Effect.all([...], { concurrency: "unbounded" })`; serializers are `Semaphore`; the mutex-from-`test-and-set!` discussion is kept conceptually, then the bank account and `serialized-exchange` are rebuilt on `Effect.tx` + `TxRef`, after first exhibiting interleaving anomalies with jittered `Effect.sleep` between read and write. Exercises keep their numbers. Representative: Peter/Paul/Mary, then the transactional exchange. Hard spot: 3.38-3.49 are conceptual and stay. Tailored: 3.43a (run the exchange under jitter 1000 times, count violations, then under `Effect.tx`, count zero); 3.47a (build a size-n semaphore from two `Ref`s, compare against the `Semaphore` module).

**3.5 Streams (19855-22671), partial re-cut inside the section.** 3.5.1-3.5.2 build a thunk `LazyList` (`Cons(head, () => tail)` with a memoizing `force`, literally the book's `cons-stream`/`delay`/`memo-proc`), covering integers, `map`, `filter`, `add-streams`, and implicit definitions like `fibs`. The switch to Effect `Stream` happens at the start of 3.5.3 (merge, weighted pairs, interleave). Fairness note: default `Stream.flatMap` is expected to concatenate sequentially (semantics unverified; existence verified), so an infinite inner stream would starve later elements; infinite search needs an explicit breadth-first (round-robin) merge helper built by the reader in 3.66a/4.36a. Representative: `fibs` implicit definition; `pairs` with weight ordering. Hard spot: implicit-definition cycles need the memoized thunk cell exactly as in Scheme. Tailored: 3.53a (powers of two stay exactly representable in `number`, so the issue is not representation but range: the stream crosses the safe-integer boundary near element 54 (2^53) and reaches `Infinity` near element 1024 (2^1024); switch to `bigint` to keep values exact); 3.66a (take-first-N on the pairs `Stream` with a fair merge, compared against the hand-rolled thunk version).

**4.1 Metacircular evaluator (22672-24871).** Changes: host TS implements the book's Scheme subset per the shared spec; `evaluate: (expr: Expr, env: Env) => Effect<Value, EvalError>`; errors are `Schema.TaggedError` classes; primitives table is a `Map`; `analyze` (4.1.7) returns a precompiled closure; driver loop enters via `NodeRuntime.runMain` with a readline REPL. Representative: the `eval` dispatch switch, exhaustive over `Expr`. Hard spot: ex 4.14 (installing host `map` as a primitive) becomes the function-representation mismatch lesson. Tailored: 4.3a (rebuild dispatch on the 2.4-style table, compare extensibility); 4.16a (scan-out-defines as an AST transform tested on the book's examples).

**4.2 Lazy evaluation (24872-25621).** Changes: `Value` gains a `Thunk` variant holding `() => Effect<Value, EvalError>` with memoization; `actualValue` forces. Representative: lazy lists of 4.2.3. Hard spot: 4.27-4.31 counting forces with a `Ref`. Tailored: 4.29a (memoized vs unmemoized force counts as a table); 4.31a (`lazy` parameter annotation as an AST flag on `Param`).

**4.3 Nondeterministic computing (25622-27115), partial re-cut.** Amb is a search `Stream` rather than the book's explicit continuation machinery: alternatives enter via `Stream.fromIterable`, composition via `flatMap` plus the explicit fair merge of 3.5.3, `require` filters, the driver takes N results. The CPS implementation is tailored exercise 4.35a; 4.36's infinite search depends on the fair merge. Representative: Pythagorean triples, multiple dwelling. Hard spot: 4.45-4.49 parsing as generator functions over a token stream. Tailored: 4.35a (solve the same puzzles with a CPS `amb` over `Result` continuations, compare); 4.49a (sentence generation as `Stream.take(6)` over the parse search).

**4.4 Logic programming (27116-30357).** Changes: unification on the S-expression union with `Map<string, Value>` frames; frames flow as `Stream<Frame>`; `streamFlatMap` is `Stream.flatMap` over finite frame sets (no fairness issue); rule/assertion store with a renaming counter `Ref`. Representative: `simpleQuery`, `not`, `lispValue`. Hard spot: 4.64-4.69 loops; 4.74-4.79 stream mechanics. Tailored: 4.62a (`lastPair` rule in both directions, note the divergent case); 4.77a (delayed filtering via a promise field on frames).

**5.1 Register machines (30358-31561).** Changes: machines as instruction arrays with labels; data-path diagrams stay ASCII. Representative: iterative factorial and Fibonacci controllers. Hard spot: 5.5 hand simulation with stacks. Tailored: 5.4a (an `expt` machine with `bigint`); 5.6a (confirm the removable save/restore by diffing stack traces of both controllers).

**5.2 Register-machine simulator (31562-32785).** Changes: registers are `Ref<Value>`; stack is `Ref<Array<Value>>`; execution is an index-based `while` loop inside `Effect.runSync` (no fiber per instruction); labels resolve to indices at assembly; tracing and counting via `Effect.log` and a count `Ref`; assembly errors are `Result`, never exceptions. Representative: `makeMachine(ops, controller)` running factorial. Hard spot: 5.8-5.13 robustness exercises. Tailored: 5.12a (data-path summary extracted as an immutable record after assembly); 5.14a (pushes and max stack depth as a returned `Stats` record, plotted over fast-check-generated n).

**5.3 Storage allocation and GC (32786-33543), re-cut in framing.** The host is garbage-collected, so the section is taught as "rebuilding by hand what V8 does for you": the vector memory (`Array<Value | undefined>` with `free`/`new` pointers in `Ref`s), pair allocation, and the stop-and-copy collector of 5.3.2 are kept as a simulation because chapter 5's explicit-control evaluator and compiler exercises run on them; the "infinite memory illusion" motivation is restated against V8's real collector. Exercises 5.20-5.22 keep their numbers. Representative: vector-based `cons`/`car`/`cdr`. Hard spot: forcing a collection mid-construction for tests. Tailored: 5.20a (print live memory-vector index maps for the 5.20 structures); 5.22a (`append!` splicing at the vector level, property: equals pure `append`).

**5.4 Explicit-control evaluator (33544-34640).** Changes: eceval controller as ch5 instructions whose ops wrap the ch4 pure helpers; registers `expr/env/val/proc/argl/continue`; errors as a checked `ErrorValue` register (5.30). Representative: `eval-dispatch` controller text. Hard spot: tail recursion verified by stack-depth stats (5.26-5.28). Tailored: 5.24a (native `cond` loop in the controller, contrasted with 4.1 derived expressions); 5.30a (typed error register plus a checked `goto`).

**5.5 Compilation (34641-end).** Changes: `compile: (expr, target, linkage, ctenv) => InstructionSeq`; `preserving` as a pure function; open coding (5.38); lexical addresses as `{ frame, disp }` (5.39-5.44); `compile-and-go` feeds the eceval machine. Representative: compiled factorial. Hard spot: 5.31-5.32 save/restore analysis. Tailored: 5.35a (pretty-print compiled sequences with indented labels, snapshot-tested); 5.51a (see divergences).

### Re-cut and divergence table (numbering policy: SICP exercise numbers kept 1:1; tailored additions use `N.Ma`)

| Where | Change | Exercise impact |
|---|---|---|
| Chapter 0 (new) | ~30-page primer, sections 0.1-0.8, exercises 0.1-0.5 | none (front matter) |
| 1.2 | `bigint` adopted for exact modular arithmetic though printed examples stay below 2^53 | statements unchanged; solvers change |
| 2.2.4 | painters render to SVG; figures snapshot under `fig/chap2/` | none |
| 2.55 | `(car ''abracadabra)` quote mechanics replaced (string-vs-symbol identity) | slot kept at 2.55, tailored 2.55a added |
| 3.2 | re-cut: closures + `Ref` model replace drawn environment diagrams | 3.9-3.11 keep numbers; answer medium changes |
| 3.4 | re-cut: fibers, `Semaphore`, `Effect.tx`/`TxRef` replace the test-and-set mutex path (kept conceptually) | exercises keep numbers |
| 3.5.3 | switch from hand thunk streams to Effect `Stream`; explicit fair merge for infinite searches | exercises keep numbers; 3.66a, 4.36a build the fair merge |
| 4.3 | amb via search `Stream` instead of continuation closures | CPS preserved as tailored 4.35a |
| 5.3 | re-cut framing: manual memory as a simulation of V8's GC | 5.20-5.22 keep numbers |
| 5.51, 5.52 | mandated host language C replaced by JavaScript (evaluator machine loop in JS; compile Scheme to JS) | the only numbered-exercise divergence, forced by the host language |

## 4. Part 3. Edition conventions

- **Numbers**: `number` by default; `bigint` where exactness demands it: modular arithmetic in 1.24-1.28, the 2^a·3^b pairs of 2.5, polynomial coefficients in 2.5.3, the 3.53a stream. Chapter 5 indices stay `number`. Stated in 1.1.6, recalled at each site.
- **Pairs and lists**: immutable fixed-arity pairs are readonly tuples; the cons list is the hand-written `List<A>` union; mutable pairs (3.3.1) are `PairRef<A, B>` with `Ref` fields, no parameter properties.
- **Pure versus Effect rule**: chapters 1-2 are pure functions returning values, `Result`, or `Option`; `Effect` appears only in test harnesses. **The switch is at section 3.1**: assignment, time, randomness, and concurrency become `Ref`, `Clock`/`Random`, fibers from there on. Rationale: chapters 1-2 have no effects to model, and wrapping `fib` in `Effect.gen` would bury the substitution model; from 3.1 the book changes subject to effects, and `Ref`'s type-visible mutation is the payoff.
- **Errors**: no `throw` in edition code. Domain errors are `Schema.TaggedError` classes; `Data.TaggedError` mentioned once as the lighter alternative. Chapters 1-2 failures return `Result<A, E>`; chapter 3+ use the `Effect` error channel.
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

- **tsconfig.base.json, exact flags** (the assignment's set plus module/target plumbing; `declaration` is required by `isolatedDeclarations`, so no `noEmit`):

```json
{
  "compilerOptions": {
    "target": "es2023",
    "lib": ["es2023"],
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
    "isolatedDeclarations": true,
    "declaration": true,
    "skipLibCheck": true
  }
}
```

No `any`, no non-null assertions, no `as unknown as`, no enums, no namespaces anywhere. Biome owns lint and format.

## 5. Part 4. Architecture sketches

Chapter 4 evaluator (`packages/ch4/src/`):

```ts
import { Effect, HashMap, Option, Ref, Schema } from "effect"
import type { List } from "./list.js"

// errors.ts — real tagged error classes (Schema.TaggedError, verified)
export class UnboundVariable extends Schema.TaggedError<UnboundVariable>()("UnboundVariable", { name: Schema.String }) {}
export class NotAProcedure extends Schema.TaggedError<NotAProcedure>()("NotAProcedure", { value: Schema.String }) {}
export class UnknownSyntax extends Schema.TaggedError<UnknownSyntax>()("UnknownSyntax", { expr: Schema.String }) {}
export class ArityMismatch extends Schema.TaggedError<ArityMismatch>()("ArityMismatch", {
  expected: Schema.Number,
  given: Schema.Number
}) {}
export type EvalError = UnboundVariable | NotAProcedure | UnknownSyntax | ArityMismatch

// core.ts
type Expr = { _tag: "SelfEvaluating" } | { _tag: "Symbol"; name: string } | { _tag: "Quote" }
  | { _tag: "If" } | { _tag: "Lambda" } | { _tag: "Begin" } | { _tag: "Cond" }
  | { _tag: "Let" } | { _tag: "Define" } | { _tag: "Set" }
  | { _tag: "Application"; operator: Expr; operands: readonly Expr[] }
type Value = { _tag: "Number"; n: number | bigint } | { _tag: "Symbol"; name: string }
  | { _tag: "List"; items: List<Value> }
  | { _tag: "Primitive"; fn: (args: readonly Value[]) => Effect.Effect<Value, EvalError> }
  | { _tag: "Compound"; params: List<string>; body: readonly Expr[]; env: Env }
  | { _tag: "Thunk"; cell: Ref.Ref<Option.Option<Value>>; force: () => Effect.Effect<Value, EvalError> } // 4.2
interface Env { readonly vars: HashMap.HashMap<string, Value>; readonly parent: Env }
// evaluate: (expr, env) => Effect<Value, EvalError>;  analyze: (expr) => (env) => Effect<Value, EvalError>
// modules: parse.ts, env.ts, core.ts, primitives.ts (Map<string, Primitive>), driver.ts,
//          lazy.ts (4.2), amb/ (4.3 Stream search with fair merge), query/ (Frame = Map<string, Value>; Stream<Frame>)
```

Chapter 5 simulator (`packages/ch5/src/sim.ts`):

```ts
import { Effect, HashMap, Ref, Schema } from "effect"

export class AssembleError extends Schema.TaggedError<AssembleError>()("AssembleError", { message: Schema.String }) {}
export class SimError extends Schema.TaggedError<SimError>()("SimError", { message: Schema.String }) {}

type Instr = { _tag: "Assign"; reg: string; src: Src } | { _tag: "Test"; op: string; args: readonly Arg[] }
  | { _tag: "Branch"; label: string } | { _tag: "Goto"; target: LabelTarget }
  | { _tag: "Save"; reg: string } | { _tag: "Restore"; reg: string }
  | { _tag: "Perform"; op: string; args: readonly Arg[] }
type Arg = { _tag: "Reg"; name: string } | { _tag: "Const"; value: Value } | { _tag: "LabelRef"; label: string }
type Machine = { readonly regs: HashMap.HashMap<string, Ref.Ref<Value>>; readonly stack: Ref.Ref<Array<Value>>
               ; readonly ops: HashMap.HashMap<string, (...a: readonly Value[]) => Value>
               ; run: Effect.Effect<Value, SimError> }
// assemble(controller) => Result<readonly Instr[], AssembleError>; labels -> indices
// execution: plain while loop over a pc index inside Effect.runSync; tracing/counting via optional Refs
// modules: sim.ts, gc.ts (5.3 vectors: Array<Value | undefined>, root scan, stop-and-copy),
//          eceval.ts (5.4 controller), compiler.ts (5.5), stats.ts (stack instrumentation)
```

Chapter 5 compiler (`packages/ch5/src/compiler.ts`):

```ts
import { Effect, Result } from "effect"

type Linkage = "Next" | "Return"
type CtEnv = readonly (readonly string[])[]
type InstructionSeq = { readonly instrs: readonly Instr[]; readonly needs: ReadonlySet<string> }
const compile: (expr: Expr, target: string, linkage: Linkage, env: CtEnv) => InstructionSeq
const preserving: (regs: readonly string[], seq: InstructionSeq) => InstructionSeq // save/restore elision
const compileLinkage: (l: Linkage) => InstructionSeq
const compileIf, compileLambda, compileApplication, compileOpenCode // 5.38
const lexicalAddress: (name: string, env: CtEnv) => Result.Result<{ frame: number; disp: number }, "not-found"> // 5.39-5.42
// entry: compileAndGo(expr) = assemble(compile(expr, ...)) then eceval.run()
```

## 6. Anchor files

- `modern-sicp/sicp-pocket.texi:1107-37952`: source text; all 22 section notes and exercises 1.1-5.52 grounded from these ranges this session
- `modern-sicp/sicp-pocket.texi:22672-30357`: chapters 4 and 5 text the evaluator/simulator/compiler sketches translate
- `typescript/pnpm-workspace.yaml` (to create): workspace root for `packages/ch0..ch5` and `book`
- `typescript/tsconfig.base.json` (to create): the exact strict flag set above
- `typescript/packages/ch4/src/core.ts` (to create): `Expr`/`Value`/`Env`/`evaluate`, the hub for chapters 4 and 5