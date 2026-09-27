# Kotlin edition, functional style: idiom map and plan inputs

## 0. Grounded toolchain pins

| Component | Pin | Status and source |
|---|---|---|
| Kotlin (JVM) | 2.4.20 (2026-09-07) | verified, https://kotlinlang.org/docs/releases.html |
| Gradle | 9.7.1 | verified, https://docs.gradle.org/current/userguide/version_catalogs.html (guide current version) |
| JDK | 25, LTS, GA 2025-09-16, via `jvmToolchain(25)` | verified, https://openjdk.org/projects/jdk/25/ ; https://kotlinlang.org/docs/gradle-configure-project.html |
| Arrow (arrow-core, arrow-fx-coroutines) | 2.2.3 stable (2.3.0 alphas exist, do not use) | verified, https://github.com/arrow-kt/arrow/releases ; https://central.sonatype.com/artifact/io.arrow-kt/arrow-fx-coroutines |
| Raise / either | `arrow.core.raise.Raise`, `arrow.core.raise.either` | verified, https://raw.githubusercontent.com/arrow-kt/arrow/main/arrow-libs/core/arrow-core/src/commonMain/kotlin/arrow/core/raise/Raise.kt and https://arrow-kt.io/learn/typed-errors/ |
| kotlinx.coroutines | 1.11.0 | verified, https://github.com/Kotlin/kotlinx.coroutines/releases |
| Mutex, Semaphore | `kotlinx.coroutines.sync` in core; fair, non-reentrant Mutex with `withLock` | verified, https://raw.githubusercontent.com/Kotlin/kotlinx.coroutines/master/kotlinx-coroutines-core/common/src/sync/Mutex.kt |
| kotlinx.collections.immutable | 0.5.2 | verified, https://github.com/Kotlin/kotlinx.collections.immutable/releases and README |
| Kotest | 6.2.5; runner `kotest-runner-junit5`, property `kotest-property` | verified, https://github.com/kotest/kotest/releases ; https://kotest.io/docs/quickstart ; https://kotest.io/docs/framework/project-setup.html |
| ktlint | engine 1.8.0 via `ktlint { version.set("1.8.0") }`; wrapper plugin `org.jlleitschuh.gradle.ktlint` 14.2.0 (unverified wrapper version, pin mechanism verified) | https://raw.githubusercontent.com/JLLeitschuh/ktlint-gradle/master/README.md |
| detekt | dropped | parent decision; lint gate is ktlint plus `allWarningsAsErrors` plus `explicitApi()` |
| Kotlin features | context parameters, Experimental, opt-in `-Xexplicit-context-arguments`; `DeepRecursiveFunction` stable since 1.7; sequences; fun interfaces | verified, https://kotlinlang.org/docs/context-parameters.html (raw page shows `freeCompilerArgs.add("-Xexplicit-context-arguments")`) ; https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-deep-recursive-function/ ; https://kotlinlang.org/docs/sequences.html ; https://kotlinlang.org/docs/fun-interfaces.html |

Unverified this session: the Kotest property-testing page path under the new docs layout (`checkAll` and the `kotest-property` artifact are verified via quickstart; `Arb` generators verify at implementation time); `explicitApi()` docs URL (parent-directed pin); Gradle compatibility for Kotlin 2.4.20 starts at 7.6.3 per the Kotlin release page.

## 1. Concept map

| SICP concept | Kotlin construct | Notes |
|---|---|---|
| `define` of a procedure | `fun name(args): T` | Top level or local; expression bodies preferred. |
| `lambda` | function type `{ a: T -> ... }` | First-class procedures are plain function values in chapters 1 to 3. |
| `if`, `cond` | expression `if` / `when` | `when` is exhaustive over sealed types. |
| `let` | local `val` | Sequential, same as Scheme body order. |
| internal `define`, mutual recursion | local `fun` | Local funs cannot forward-reference; sketch 3. |
| `set!` on a captured variable | captured `var` in a closure | The 3.1 decision, sketch 2; allowed only in chapter 3 state sections and chapter 5 simulator internals. |
| `begin` | block `{ ... }` | Last expression is the value. |
| `quote`, symbols | `VSym(name)` inside `Value` | Symbols only where dynamic data exists (2.3.1, chapters 4 and 5); structural code uses sealed classes. |
| `cons`, `car`, `cdr` | `VPair(car, cdr)` or typed `Pair`/data classes | Three regimes, see conventions. |
| lists | `List<T>` / `PersistentList<T>` | `PersistentList` where structural sharing or persistence is load-bearing. |
| `cons-stream`, `delay`, `force` | `Lazy` inside `SCons`; `lazy` | Memoized call-by-need, sketch 4. |
| streams as pipelines | `Sequence<T>` | `generateSequence`, `sequence { }`, lazy and cold. |
| iteration (named `let`, `do`) | `tailrec fun` or `while` | No implicit TCO; mutual tail calls use a dispatch loop, sketch 5. |
| `eq?` on symbols | `==` on `VSym` | `===` only where identity matters (3.3, 5.3). |
| `equal?` | `==` | Data class equality. |
| `error` | `raise(...)` with Arrow `Raise<E>` | Per-domain sealed errors; no exceptions across library code. |
| `#t`, `#f` | `Boolean` | Scheme truthiness is `isTruthy(Value)` in the evaluators. |
| higher-order numerics | `(Double) -> Double` | 1.3 maps one to one. |
| message passing | ONE default, see sketch 6 | Returned lambda for single-behavior objects; interface object for named messages; `fun interface` only for bare procedure objects. |
| `put` / `get` tables | immutable `OpTable` over `PersistentMap` | The 2.4 pick, sketch 7. |
| environment frames | `Env(var frame: PersistentMap<String, Value>, val parent: Env?)` | `define` replaces the frame map on the same `Env` object, so sharing is preserved; `set!` mutates explicit `Cell`s. |
| registers, stack, memory | `Register` with `var content`, `ArrayDeque<Value>`, `Array` pair vectors | Chapter 5. |
| `display`, `newline` | `print`, `println` | Transcripts as comments, see conventions. |

Sketch 1, pairs as data (`Value`), for quoted and interpreted data:

```kotlin
sealed interface Value
@JvmInline value class VNum(val n: Long) : Value
@JvmInline value class VSym(val name: String) : Value
data class VBool(val b: Boolean) : Value
data class VPair(val car: Value, val cdr: Value) : Value
data object VNil : Value
fun vlist(vararg xs: Value): Value = xs.foldRight(VNil as Value, ::VPair)
fun Value.toList(): List<Value> = generateSequence(this as? VPair) { it.cdr as? VPair }.map { it.car }.toList()
```

Sketch 2, the 3.1 decision: captured `var` for local state. Reason: closures over `var` reproduce Scheme's shared mutable cell exactly, and section 3.1 exists to teach that mechanism, so wrapping it in a class would hide the lesson; classes enter only where the book gives the object a protocol, and 3.4 replaces raw `var` with locks. The insufficient-funds string becomes a typed error (the one such case, see conventions):

```kotlin
sealed interface WithdrawError { data object InsufficientFunds : WithdrawError }
fun makeWithdraw(balance: Long): (Long) -> Either<WithdrawError, Long> {
    var b = balance                                  // one mutable cell, captured by the lambda
    return { amount ->
        if (amount > b) Either.Left(WithdrawError.InsufficientFunds)
        else { b -= amount; Either.Right(b) }
    }
}
```

Sketch 3, mutual internal defines (Kotlin local funs do not forward-reference):

```kotlin
fun block(): Int {
    fun f(): Int = g()   // compile error: g is not yet declared
    fun g(): Int = 1
    return f()
}
// Edition rule: hoist mutually recursive internal defines to private top-level
// funs in the same file, or bundle them in a local object when they close over state.
```

Sketch 4, self-referential memoized stream (3.5.3, `fibs`, `solve`):

```kotlin
sealed interface LStream<out A>
data class SCons<out A>(val head: A, val tail: Lazy<LStream<A>>) : LStream<A>
data object SNil : LStream<Nothing>
fun tailS(s: LStream<Long>): LStream<Long> = (s as SCons).tail.value
val fibs: LStream<Long> by lazy {
    SCons(0L, lazy { SCons(1L, lazy { zip(Long::plus, fibs, tailS(fibs)) }) })
}
// lazy {} memoizes, so the shared tail is computed once: exercise 3.57 semantics.
```

Sketch 5, no mutual tail calls, so the evaluators use a dispatch loop:

```kotlin
var env = currentEnv
while (true) {
    when (val e = nextExpr) {
        is VarE -> return lookup(e.name, env)
        is IfE  -> nextExpr = if (isTruthy(eval(e.pred, env))) e.conseq else e.alt
        is AppE -> { env = extend(e, env); nextExpr = bodyHead(e) }   // tail position
        // remaining arms follow the book's eval dispatch order
    }
}
```

Sketch 6, message passing, one default: a returned lambda when the object has a single behavior (make-withdraw, make-accumulator, rand); an interface implemented by an object expression when the object has named messages (make-account, 2.4.3); `fun interface` appears exactly where the book returns a bare procedure object standing in for the whole interface.

```kotlin
fun interface Account { fun dispatch(msg: Msg): Any }      // SAM conversion from lambdas
fun makeAccount(balance: Long): Account = object : Account {
    var b = balance
    override fun dispatch(msg: Msg) = when (msg) {
        is Withdraw -> withdrawish(msg.amount)
        is Deposit  -> depositish(msg.amount)
    }
}
```

Sketch 7, data-directed `put`/`get` as an immutable registry (the 2.4 pick; static sealed `when` cannot register packages at runtime, which exercises 2.73 to 2.97 require):

```kotlin
typealias Op = (List<Value>) -> Value
class OpTable(private val m: PersistentMap<Pair<String, String>, Op> = persistentMapOf()) {
    fun put(op: String, type: String, f: Op) = OpTable(m.put(op to type, f))
    operator fun get(op: String, type: String): Op? = m[op to type]
}
// Package install threads the table; applyGeneric looks up, then coerces.
```

Sketch 8, 4.3 amb as `Sequence` search over persistent environments (decision final: `Sequence` composition with a stored top-level iterator; the `sequence { }` delimited-continuation variant was considered and rejected because choice points need no suspension; hand-written CPS success/failure continuations are rejected as inside-out):

```kotlin
typealias Exec = (AmbEnv) -> Sequence<Pair<Value, AmbEnv>>   // value with resulting env
fun analyzeAmb(choices: List<Expr>): Exec =
    { env -> choices.asSequence().flatMap { analyze(it)(env) } }      // choice point
val require: (Boolean) -> Exec =
    { ok -> { env -> if (ok) sequenceOf(VSym("ok") to env) else emptySequence() } }
// Conjunction is flatMap over the first's frames feeding the second; assignments
// write a NEW AmbEnv (persistent map), so abandoning a branch abandons its effects.
// permanent-set! writes a shared cell store instead (see 4.3 notes).
// Driver: val it = analyze(program)(globalEnv).iterator(); tryAgain() = it.next()
```

## 2. Per-section notes, 1.1 to 5.5

Format per section: what changes against the Scheme text, the hard spot, the representative program, two tailored exercises (anchors verified in the Texinfo).

### 1.1 The elements of programming

- Changes: `define` becomes `fun` or `val`; `cond` becomes `when`; REPL sessions become demo `main` functions in the examples source set.
- Hard spot: exercise 1.5 (applicative order). Kotlin always evaluates arguments, so `(test 0 (p))` becomes a choice between an eager `Int` parameter (hangs) and a `() -> Int` parameter (returns), which is exactly the teaching point.
- Representative program: `sqrt` by Newton's method (section 1.1.7): `sqrtIter`, `improve`, `goodEnough` as local funs.
- Tailored ideas: 1.5a, repeat the probe with an eager parameter and a lambda parameter and state which matches Scheme. 1.7a, reimplement `goodEnough` with a relative tolerance, property-tested over `Arb.double` for small and large inputs.

### 1.2 Procedures and the processes they generate

- Changes: iterative processes become `tailrec` funs (`factorial`, `gcd`, `expt`, `findDivisor`, `contFrac`); `runtime` becomes `measureNanoTime`; tree recursion stays plain recursion.
- Hard spot: no automatic tail-call optimization; every iterative process must be `tailrec` or a `while`; mutual tail calls do not exist (sketch 5).
- Representative program: `countChange`, `fastExpt` with `square`, Euclid `gcd` as `tailrec`, `timedPrimeTest` with `expmod`.
- Tailored ideas: 1.19a, drive the logarithmic Fibonacci transformation with `BigInteger` state so `fib(1000)` works, exposing the `Long` boundary. 1.28a, property-test Miller-Rabin against `BigInteger.isProbablePrime` on the Carmichael numbers of 1.27.

### 1.3 Formulating abstractions with higher-order procedures

- Changes: procedures as arguments and results are plain function types; `lambda` arguments become trailing lambdas.
- Hard spot: `fixedPoint`, `averageDamp`, and `newtonsMethod` return procedures; the iteration inside `fixedPoint` must be `tailrec` or `generateSequence().takeWhile()`.
- Representative program: `sum`, `integral` with `dx`, then `fixedPoint` composed with `averageDamp` (1.3.4).
- Tailored ideas: 1.30a, express iterative `sum` as `fold` over an `IntRange` and prove equality with the `tailrec` version by property test. 1.43a, implement `repeated` as `fold` over function composition and assert `repeated(inc, n)(0) == n` with `checkAll`.

### 2.1 Introduction to data abstraction

- Changes: `make-rat` and selectors become a `Rational`-shaped data class with operator funs; the procedural-pair exercises (2.4 to 2.6) use functions returning lambdas.
- Hard spot: immutability with gcd normalization at construction; exercise 2.4's lambda-pair needs an explicit closure type.
- Representative program: rational arithmetic and the interval extended exercise (2.1.4).
- Tailored ideas: 2.10a, division by a zero-spanning interval raises `Raise<IntervalError>` with a sealed error. 2.12a, `makeCenterPercent` as a value class with a roundtrip property test.

### 2.2 Hierarchical data and the closure property

- Changes: sequence exercises (2.17 to 2.43) use `List` and `PersistentList`; `map`/`filter`/`accumulate` shown twice, hand-rolled over `Value` pairs (2.2.1) and with `Sequence` (2.2.3); mobiles, `fringe`, `squareTree` use sealed classes; the picture language emits SVG.
- Hard spot: `foldRight` argument order and `accumulateN`; painter combinators close over a `Frame`.
- Representative program: `scaleList`, `for-each` (2.2.1), `countLeaves` and `fringe` (2.2.2), `primeSumPairs` and eight-queens `flatmap` (2.2.3), `beside`/`below`/`upSplit` over SVG painters (2.2.4).
- Tailored ideas: 2.28a, `fringe` as a lazy `Sequence<Leaf>` traversal. 2.46a, `Vect` as a `@JvmInline value class` with operator funs.

Painter sketch (SVG output, one `<svg>` per demo `main`):

```kotlin
typealias Painter = (Frame, StringBuilder) -> Unit
data class Frame(val origin: Vect, val e1: Vect, val e2: Vect)
fun beside(p1: Painter, p2: Painter): Painter = { f, out ->
    p1(Frame(f.origin, scale(f.e1, 0.5), f.e2), out)
    p2(Frame(add(f.origin, scale(f.e1, 0.5)), scale(f.e1, 0.5), f.e2), out)
}
```

### 2.3 Symbolic data

- Changes: quotation becomes `VSym` and `vlist`; `equal?` is `==`; symbolic differentiation drops quoted lists for a sealed `Expr` hierarchy with smart constructors; sets are `PersistentList`, ordered `List`, and a sealed binary tree; Huffman trees are a sealed hierarchy.
- Hard spot: quote has no Kotlin analogue, so 2.3.1 is taught on `Value` while 2.3.2 onward switches to typed representations.
- Representative program: `deriv` with simplifying constructors (2.3.2), the three set representations (2.3.3), `generateHuffmanTree` and `encode` (2.3.4).
- Tailored ideas: 2.56a, property-test `deriv` against the hand power rule for generated `x^n`. 2.69a, build the Huffman tree twice, sorted `PersistentList` versus `java.util.PriorityQueue`, and compare merge steps.

Symbolic differentiation, two named sketches:

```kotlin
sealed interface Expr
@JvmInline value class Vr(val name: String) : Expr
data class NLit(val n: Long) : Expr
data class Sum(val terms: PersistentList<Expr>) : Expr
data class Product(val factors: PersistentList<Expr>) : Expr
data class Expt(val base: Expr, val n: Int) : Expr
```

```kotlin
fun deriv(e: Expr, x: String): Expr = when (e) {          // exhaustive over the sealed tree
    is Sum -> Sum(e.terms.map { deriv(it, x) }.toPersistentList())
    is Product -> sumOfOneFactorDerivs(e.factors, x)       // named helper, exercises 2.56/2.57
    is Expt -> mul(mul(NLit(e.n), Expt(e.base, e.n - 1)), deriv(e.base, x))
    is Vr -> NLit(if (e.name == x) 1 else 0)
    is NLit -> NLit(0)
}
```

Huffman sketch:

```kotlin
sealed interface Tree { val weight: Long }
data class Leaf(val sym: String, override val weight: Long) : Tree
data class Branch(val left: Tree, val right: Tree, override val weight: Long,
                  val syms: PersistentSet<String>) : Tree
fun makeCodeTree(l: Tree, r: Tree) = Branch(l, r, l.weight + r.weight, l.symbols + r.symbols)
// encode walks with chooseBranch inside a fold over the message.
```

### 2.4 Multiple representations for abstract data

- Changes: type tags become `Tagged(tag: String, contents: Value)` for 2.4.2; the data-directed subsection installs rectangular and polar packages into the immutable `OpTable` (sketch 7); message passing follows the one default of sketch 6.
- Hard spot: the edition decision point. Sealed `when` dispatch is shown in 2.4.1 and 2.4.2 as the contrast, but `applyGeneric` uses the registry because package registration happens at runtime.
- Representative program: the complex-number system with both packages installed, `applyGeneric` with two-key lookup, `makeFromRealImag` message passing.
- Tailored ideas: 2.73a, make the type tag a `@JvmInline value class` so tagged and plain numbers cannot mix. 2.75a, implement `makeFromMagAng` with a `fun interface` SAM.

### 2.5 Systems with generic operations

- Changes: the tower is sealed `Num` (sketch under conventions); the four packages install into the registry; coercion is a second registry; polynomials keep term lists as `PersistentList<Term>`.
- Hard spot: `applyGeneric` with coercion of mixed arguments (2.82 to 2.85) and `drop`/`project` lowering.
- Representative program: generic `add`/`sub`/`mul`/`div` over the tower, then sparse and dense polynomial term lists.
- Tailored ideas: 2.84a, explicit `towerLevel` on `Num` driving successive raising. 2.89a, dense term lists as `List<Term>` indexed by order, proved equivalent to the sparse form by property test.

### 3.1 Assignment and local state

- Changes: the captured `var` decision section (sketch 2). `makeWithdraw` and `makeAccumulator` are closures over `var`; `makeAccount` is an interface-implementing object because the book's dispatch gives it a protocol; `rand` with `reset` (3.1.2) captures a seed driving `kotlin.random.Random`.
- Hard spot: stating why `var` here and not elsewhere: 3.1 to 3.3.2 are single-threaded and the mechanism is the lesson; 3.4 locks; 5.x simulators own their state outright.
- Representative program: `makeWithdraw` and `makeAccount`, then `rand` with the `Generate`/`Reset` dispatch.
- Tailored ideas: 3.3a, password failures raise `Raise<AccountError>` with sealed `WrongPassword` and `CallTheCops`. 3.6a, implement `rand` reset with `Random(seed)` and property-test sequence reproduction.

### 3.2 The environment model, re-cut as closures and captured state

- Changes: this section is re-cut per the interview. The heading changes to the closure and captured-`var` model: a `var` captured by a lambda is the frame slot, and the chapter 4 `Env` is cited as the formal model the closures realize. Environment diagrams are kept but drawn against closure cells. The `let` version of `make-withdraw` maps to a local `val` plus lambda.
- Hard spot: shared capture. Two `makeWithdraw` calls produce two cells, while exercise 3.10's `let` version produces the same shape; a one-element holder class makes the boxing explicit.
- Representative program: the two `makeWithdraw` variants and the `makeAccount` walkthrough (3.2.3 and 3.2.4).
- Tailored ideas: 3.10a, compare closure over `var` against closure over a holder object and state which matches the Scheme frame. 3.11a, a shadowing demo where a local fun rebinds an outer name, mirroring frame lookup.

### 3.3 Modeling with mutable data

- Changes: `set-car!`/`set-cdr!` introduce a mutable `MPair` used only here and in 5.3; queues are a class with two `MPair?` pointers, no `!!`; tables are `MPair` chains for faithfulness with `MutableMap` noted as the everyday alternative; the circuit simulator keeps the agenda; constraints are a sealed `Constraint` hierarchy.
- Hard spot: the agenda is a discrete-event loop with no threads, a `MutableList` of `(time, action)` segments processed in order; exercise 3.32's FIFO order gets a test.
- Representative program: `makeWire` with signal plus action list, `afterDelay`, `propagate`, `halfAdder`; the queue and table sections; `celsiusFahrenheitConverter`.
- Tailored ideas: 3.25a, an n-key table as nested maps with `List<Key>` lookup. 3.27a, memoized fib backed by the table, counting steps to prove O(n).

Mutable queue sketch (mutable pairs are themselves `Value`s, so `VNil` is a legal cdr):

```kotlin
class MPair(var car: Value, var cdr: Value) : Value
class Queue {
    var front: MPair? = null; var rear: MPair? = null
    fun insert(v: Value) {
        val p = MPair(v, VNil)
        if (rear == null) front = p else rear?.cdr = p     // rear non-null in this branch
        rear = p
    }
}
```

### 3.4 Concurrency, re-cut as structured coroutines

- Changes: this section is re-cut per the interview. `parallel-execute` becomes `launch` inside `coroutineScope`; the serializer becomes a `Mutex` wrapper returning suspending procedures; `test-and-set!` becomes `AtomicBoolean.compareAndSet`; the semaphore of exercise 3.47 is a token `Channel<Unit>(n)`, receive to acquire, send in `finally` to release, because a Mutex plus counter cannot park waiters; the deadlock discussion keeps ordered `Mutex` acquisition (3.48's numbering scheme).
- Hard spot: deterministic tests. Interleavings are nondeterministic on the JVM, so exercises 3.39 to 3.42 enumerate outcomes in `runTest` with explicit `yield()` points, and the text says so.
- Representative program: the serialized `makeAccount` with suspend funs and `serializedExchange`.
- Tailored ideas: 3.39a, reproduce the five interleavings deterministically by injecting yield points and assert each allowed final value. 3.47a, build the token semaphore and property-test permit conservation against `kotlinx.coroutines.sync.Semaphore`.

Serializer and semaphore sketches:

```kotlin
class Serializer { private val m = Mutex()
    fun <A> serialized(f: suspend () -> A): suspend () -> A = { m.withLock { f() } }
}
// makeAccount: withdraw and deposit are suspend funs wrapped by one shared Serializer
```

```kotlin
class TokenSemaphore(n: Int) {
    private val tokens = Channel<Unit>(n).apply { repeat(n) { trySend(Unit) } }
    suspend fun <A> use(f: suspend () -> A): A =
        try { tokens.receive(); f() } finally { tokens.send(Unit) }   // waiter parks in receive
}
```

### 3.5 Streams

- Changes: two stream regimes. Stdlib `Sequence` carries the pipeline subsections (3.5.2 enumerations, sieve, exercises 3.50 to 3.62 algorithms). The memoized `LStream` (sketch 4) carries the sections where `cons-stream` semantics are load-bearing: 3.5.1's `memo-proc`, 3.5.3 feedback (`solve`, `integral`, exercise 3.63's shared `guesses`), 3.5.4 delayed integrand, 3.5.5 modularity arguments.
- Hard spot: `Sequence` is cold and unmemoized, so self-referential definitions like `fibs` and observable memoization probes cannot be written with it; the edition names that limit at 3.5.1 and switches representation there.
- Representative program: `merge`, `sqrtStream`, `integral` and `solve` with `delay`.
- Tailored ideas: 3.57a, a counting probe on additions showing the memoized versus unmemoized `fibs` difference. 3.81a, the request-driven random stream as `requests.runningFold(seed, ::step)` over a sealed `Generate`/`Reset` request type.

Sequence sieve sketch (3.5.2):

```kotlin
val ints = generateSequence(1L) { it + 1 }
fun sieve(s: Sequence<Long>): Sequence<Long> = sequence {
    val p = s.first(); yield(p)
    yieldAll(sieve(s.drop(1).filter { it % p != 0L }))
}
```

### 4.1 The metacircular evaluator

- Changes: programs are data as `Value` lists built with `vlist` (no Scheme reader; the driver evaluates prebuilt expressions, a stated deviation shared by all four evaluators); expressions parse to the sealed `Expr` hierarchy so 4.1.7's `analyze` returns `(Env) -> Value` closures; errors flow through `Raise<EvalError>`; primitives are a persistent registry; the driver wraps each top-level expression in `either { }`.
- Hard spot: tail calls between `eval` and `apply`, handled by the dispatch loop of sketch 5; `analyze` closures must not capture stale environments.
- Representative program: the evaluator twice, as `eval`/`apply` (4.1.1) and as `analyze` (4.1.7), with `and`/`or` derived.
- Tailored ideas: 4.6a, golden-test `let` expansion through the evaluator. 4.11a, frames as `PersistentMap` and lookup depth versus the pair-of-lists frame.

### 4.2 Variations on a Scheme: lazy evaluation

- Changes: thunks become `Lazy<Value>` (memoized, call-by-need) with an unmemoized variant for the exercises probing memoization; `actualValue` forces; the lazy-list subsection (4.2.3) reuses `LStream`.
- Hard spot: which arguments are lazy. Compound-procedure arguments are thunks, primitives stay strict; the operator must be forced (`actualValue`), which exercise 4.28 demonstrates.
- Representative program: `lazyEval`/`lazyApply` with `forceIt`, then `solve` re-run in the lazy language.
- Tailored ideas: 4.27a, an observable memoization counter with memoized versus unmemoized thunks. 4.30a, a sequence side effect that fires only when `evalSequence` forces non-final elements.

Thunk sketch:

```kotlin
data class VThunk(val body: Lazy<Value>) : Value             // memoized call-by-need (default)
data class VThunkNoMemo(val body: () -> Value) : Value       // the 4.27/4.29 probe variant
fun force(v: Value): Value = when (v) {
    is VThunk -> v.body.value
    is VThunkNoMemo -> v.body()
    else -> v
}
```

### 4.3 Variations on a Scheme: nondeterministic computing

- Changes: the amb evaluator keeps the analyze-based architecture but runs the `Sequence` search engine over persistent environments (sketch 8). `try-again` is iterator advance; `require` is the empty sequence; `ramb` shuffles choices; `ifFail` substitutes a fallback sequence on empty.
- Hard spot: side effects under backtracking. Assignments write new persistent environments, so abandoned branches lose them; `permanent-set!` (4.51) writes a shared cell store (`class Cells { private val m = HashMap<String, Cell>() }`) that survives backtracking, which is exactly the observable difference exercises 4.51 to 4.53 test.
- Representative program: `multipleDwelling` and the natural-language `parse` (4.3.2).
- Tailored ideas: 4.50a, `ramb` by shuffling the choice list with `kotlin.random.Random`. 4.52a, `ifFail` by catching the empty sequence and yielding the alternative.

### 4.4 Logic programming

- Changes: frames are persistent maps from variable to `Value`; the stream of frames is `Sequence<Frame>`; `qeval` is `(Query, Sequence<Frame>) -> Sequence<Frame>`; `and` is `flatMap`, `or` concatenates, `not` and `lispValue` filter; rules live in a registry with pattern match before unification; `stream-append-delayed` becomes an explicit `interleave` helper (stdlib lacks one; about 15 lines, listed once).
- Hard spot: infinite streams. `simpleQuery` must interleave rule-derived and assertion-derived streams lazily so recursive rules do not starve assertions, which exercises 4.71 and 4.72 probe.
- Representative program: the Microshaft queries (4.4.1), then `simpleQuery`, `disjoin`, `negate`, `lispValue` (4.4.4).
- Tailored ideas: 4.64a, fix the Louis `outrankedBy` loop with a visited-set of (pattern, frame) pairs. 4.75a, `unique` as exactly-one over the `Sequence<Frame>` with a sealed `UniqueResult`.

Query sketch:

```kotlin
typealias Frame = PersistentMap<String, Value>               // pattern variable -> binding
fun qeval(q: Query, frames: Sequence<Frame>): Sequence<Frame> = when (q) {
    is SimpleQ -> frames.flatMap { f -> findAssertions(q.pat, f) + applyRules(q.pat, f) }
    is AndQ -> q.parts.fold(frames) { fs, p -> qeval(p, fs) }  // interleave inside simpleQuery
    is OrQ -> qeval(q.left, frames) + qeval(q.right, frames)
    is NotQ -> frames.filter { qeval(q.q, sequenceOf(it)).none() }
    is LispValQ -> frames.filter { evalPredicate(q.fn, it) }
}
```

### 5.1 Designing register machines

- Changes: machine descriptions are a Kotlin DSL (`machine { registers(...); controller { label("gcdLoop"); assign(...) } }`) building the same `List<Stmt>` data the 5.2 simulator consumes; recursion via `save`/`restore` maps to the simulator stack.
- Hard spot: keeping code-as-data. The DSL is sugar over the data representation; Kotlin control flow must not execute directly, so 5.4 and 5.5 reuse the same `Stmt` list.
- Representative program: the GCD machine (5.1.1) and recursive factorial with stack (5.1.4).
- Tailored ideas: 5.1a, generate a Mermaid data-path diagram from the machine description inside a test. 5.3a, property-test the sqrt machine's convergence over `Arb.double`.

### 5.2 A register-machine simulator

- Changes: `makeMachine` returns a `Machine` (architecture section); the assembler builds a label-to-index map plus per-instruction execution; stack statistics are counters on the `ArrayDeque`-based stack; operations install into a persistent registry at assembly.
- Hard spot: `restore` semantics (exercise 5.11) and sealed source/dest syntax so the assembler can reject label operands to operations (5.9).
- Representative program: the simulator on the factorial and Fibonacci machines with monitored stack output (5.2.4).
- Tailored ideas: 5.12a, the instruction table collected during assembly, property-tested for consistency. 5.19a, breakpoints as a `Channel<Unit>` rendezvous so the machine parks until resumed.

### 5.3 Storage allocation and garbage collection

- Changes: memory is two `Array`s (`theCars`, `theCdrs`), pairs are `Int` indices, and the slot union is a small sealed type; the stop-and-copy collector is a `while` loop over `free` and `scan` with a `BrokenHeart` marker and forwarding index in the cdr slot.
- Hard spot: the interleave of copying and scanning is pure index juggling; the flip swaps the two array pairs.
- Representative program: `memoryCons`, vector read/write, then the GC with `relocateOld`.
- Tailored ideas: 5.20a, snapshot the vectors after building small structures and compare against a hand-written index table. 5.22a, the `append!` machine asserting identity of the mutated first list via `===`.

Memory sketch:

```kotlin
class Memory(val n: Int) {
    var cars = arrayOfNulls<Any>(n); var cdrs = arrayOfNulls<Int>(n); var free = 0
    data object BrokenHeart                                  // car-slot moved marker
    fun cons(car: Any, cdr: Int): Int { cars[free] = car; cdrs[free] = cdr; return free++ }
}
```

### 5.4 The explicit-control evaluator

- Changes: the controller becomes the same `Stmt` list as 5.2 with an enum of registers (`Exp`, `Env`, `Val`, `Continue`, `Proc`, `Argl`, `Unev`); the dispatch is a `while (true)` `when`; machine operations reuse the chapter 4 registries so both runtimes share code.
- Hard spot: the machine demonstrates tail recursion without language support (5.4.2), which Kotlin cannot show natively; exercises 5.26 to 5.29 measure it through the monitored stack.
- Representative program: the `evalDispatch` core and `evSequence` tail loop, run with the driver loop of 5.4.4.
- Tailored ideas: 5.26a, property-test the stack-depth formula for iterative factorial. 5.30a, route runtime errors through `Raise<EvalError>` into the driver loop instead of crashing the machine.

### 5.5 Compilation

- Changes: instruction sequences are `InstrSeq(needs, modifies, stmts)` values over `PersistentList`; `preserving`, `appendSeq`, and `tackOnInstrSeq` are pure combinators; linkage is a sealed type; compiled statements run on the unmodified 5.2 machine; lexical addressing (5.5.6) is a compile-time environment of lists.
- Hard spot: the needs/modifies algebra; keep `preserving` as a two-line definition over set intersection and show compiled factorial beside the book figure.
- Representative program: `compile` dispatch, `compileCombination` with operand order, the compiled recursive factorial of 5.5.5.
- Tailored ideas: 5.40a, property-test that `findVariable` addressing is bijective for generated compile-time environments. 5.45a, cross-check compiled versus interpreted stack statistics for the chapter 1 programs in one data-driven spec.

## 3. Conventions

### Numbers

| Book object | Kotlin type | Notes |
|---|---|---|
| Integers (default) | `Long` | Covers every value the main text prints. JVM `Long` arithmetic wraps silently on overflow, so the standing rule applies: any listing that can plausibly overflow uses `Math.addExact`/`Math.subtractExact`/`Math.multiplyExact` so overflow throws instead of yielding a wrong answer. |
| Exercises that exceed `Long` | `java.math.BigInteger` | The concrete sites: factorial beyond `20!`, `fib` past 92, exploratory `(expt 2 100)`; tailored 1.19a is the chapter 1 example. |
| Indices, counters, ch5 pointers | `Int` | Register indices, memory addresses, vector subscripts. |
| Reals | `Double` | `sqrt`, `pi-sum`, fixed points, streams of reals. |
| 2.5 numeric tower | sealed `Num` | Integer level raises `ArithmeticException` as an `Overflow` error through the tower's `Raise`, promoting to a `BigZ` package; exact rationals carry `BigInteger` numerators and denominators because `Long` components overflow during multiplication before reduction. |

Tower sketch:

```kotlin
sealed interface Num
@JvmInline value class ZLong(val n: Long) : Num            // exact ops via Math.*Exact
data class QRat(val num: BigInteger, val den: BigInteger) : Num   // den > 0, gcd-reduced
@JvmInline value class Real(val d: Double) : Num
data class Complex(val re: Double, val im: Double) : Num
```

### Error handling

- No exceptions across library code; no `error("...")` strings in translated listings.
- Chapter 4: sealed `EvalError` (`Unbound`, `NotApplicable`, `UnknownExprType`, `UnknownProcType`) raised through `Raise<EvalError>`, caught at the driver with `either { }`.
- Domain errors are per-domain sealed types: `IntervalError` (2.1), `AccountError` (3.1), `WithdrawError` (sketch 2, the one book string that becomes a typed error; the Scheme string behavior is described in prose once), `QueryError` (4.4).
- `Raise` is a context parameter: `context(r: Raise<EvalError>)`, opt-in flag `-Xexplicit-context-arguments` (Experimental; context parameters replace the retired context receivers). A footnote shows the explicit-parameter fallback for readers who avoid experimental features.

### Test idioms

- Kotest 6.2 `FunSpec`; `shouldBe`, `shouldThrow`; `withData` rows for the book's printed session results.
- Property testing with `kotest-property` `checkAll` and `Arb` for numeric invariants (1.3, 2.5), structure roundtrips (2.2, 5.3), and tower coercion (2.5).
- Concurrency tests (3.4) run under `kotlinx-coroutines-test` `runTest` with virtual time and explicit `yield()` interleaving points; the text states that JVM scheduling is nondeterministic and the tests force the interleavings they assert on.
- No wall-clock-dependent tests; 1.2.6 timing exercises compare `measureNanoTime` medians over warmed-up loops and assert only gross ratios.

### Listing file naming

- One file per book subsection: `S1_1_7Sqrt.kt`, `S3_3_4DigitalCircuits.kt`, package `sicp.chNN.examples`.
- Exercises: one file each, `E1_10Ackermann.kt`, package `sicp.chNN.exercises`, statement as KDoc, `TODO("exercise 1.10")` bodies so stubs compile but fail if called.
- Solutions: same names, package `sicp.chNN.solutions` source set.
- Shared runtime in package `sicp.runtime`: `Value`, `LStream`, `OpTable`, `Env`, `Machine`, query engine.

### Interpreter transcripts

The book's `;Value:` sessions are preserved verbatim, the call rendered in Kotlin, the response as a comment:

```kotlin
makeWithdraw(100L)(25L)
// ;Value: Either.Right(75)

square(5L)
// ;Value: 25
```

Compound procedures print as `// ;Value: compound-procedure`; errors as `// ;Value: Unbound(x)` from chapter 4 onward.

### Style contract operationalization

- `val` everywhere; `var` only inside closure-capture state (3.1 to 3.3.2), simulator internals (3.3.4, chapter 5), and table/queue implementations; the text names each allowed site.
- No `!!`; nullability via `?.`, explicit null branches, or `requireNotNull` in machine internals.
- No unscoped `lateinit`; a nullable field is preferred even for two-phase machine setup.
- Sealed hierarchies with exhaustive `when`; `allWarningsAsErrors = true`.
- Context parameters via `-Xexplicit-context-arguments` in `freeCompilerArgs`.
- Lint gate: ktlint engine 1.8.0 through plugin `org.jlleitschuh.gradle.ktlint`, `formatKotlin`/`lintKotlin` across all five source sets, plus `explicitApi()` on the runtime module. detekt is not used.
- JDK 25 via `kotlin { jvmToolchain(25) }` in every subproject.

## 4. Architecture sketches and Gradle layout

### Chapter 4 evaluator

```kotlin
sealed interface Expr                                      // parsed from Value programs
@JvmInline value class VarE(val name: String) : Expr
data class IfE(val p: Expr, val c: Expr, val a: Expr) : Expr
data class LambdaE(val params: PersistentList<String>, val body: PersistentList<Expr>) : Expr
data class DefineE(val name: String, val e: Expr) : Expr
data class AppE(val op: Expr, val args: PersistentList<Expr>) : Expr  // + SelfEval, QuoteE, derived

class Env(var frame: PersistentMap<String, Value>, val parent: Env?)   // define rebinds frame;
class Cell(var v: Value)                                               // set! mutates cells

sealed interface EvalError {
    data class Unbound(val name: String) : EvalError
    data class NotApplicable(val v: Value) : EvalError
}

sealed interface Value { /* VNum, VSym, VBool, VPair, VNil, VThunk, plus: */ }
data class VPrimitive(val f: context(Raise<EvalError>) (List<Value>) -> Value) : Value
data class VProc(val params: PersistentList<String>, val body: PersistentList<Expr>, val env: Env) : Value

typealias Analysis = context(Raise<EvalError>) (Env) -> Value
fun analyze(e: Expr): Analysis
context(r: Raise<EvalError>) fun eval(expr: Expr, env: Env): Value = analyze(expr)(env)
// Tail positions run in the while loop of sketch 5; the driver calls eval inside either { }.
```

### Chapter 5 register-machine simulator

```kotlin
enum class Reg { Exp, Env, Val, Proc, Argl, Unev, Continue }
sealed interface Stmt
data class Assign(val r: Reg, val src: Source) : Stmt      // Source: RegSrc | ConstSrc | OpSrc
data class Perform(val act: Action) : Stmt
data class Test(val cond: Cond) : Stmt
data class Branch(val label: String) : Stmt
data class Goto(val to: GotoTarget) : Stmt
sealed interface GotoTarget
data class Lbl(val name: String) : GotoTarget
data class ByReg(val r: Reg) : GotoTarget
data class Save(val r: Reg) : Stmt
data class Restore(val r: Reg) : Stmt
data class Label(val name: String) : Stmt

class Stack { private val d = ArrayDeque<Value>(); var pushes = 0L; var maxDepth = 0L }
class Register(val name: Reg) { var content: Value = VNil }
class Machine(regs: Set<Reg>, val ops: PersistentMap<String, Op>, controller: PersistentList<Stmt>) {
    val registers = regs.associateWith(::Register)
    val stack = Stack()
    fun run() { val labels = indexLabels(controller); var pc = 0
        while (true) when (val s = controller[pc]) {
            is Assign -> { registers.getValue(s.r).content = evalSource(s.src, ops); pc++ }
            is Goto -> pc = labels.getValue((s.to as Lbl).name)
            is Label -> pc++
            // remaining arms follow the book's instruction order
        } }
}
```

### Chapter 5 compiler

```kotlin
data class InstrSeq(val needs: Set<Reg>, val modifies: Set<Reg>, val stmts: PersistentList<Stmt>)
sealed interface Link { data object Next : Link; data object Return : Link
                       data class GotoL(val label: String) : Link }
typealias CompileEnv = PersistentList<PersistentList<String>>     // 5.5.6 lexical addresses

fun compile(e: Expr, target: Reg, link: Link, cenv: CompileEnv): InstrSeq  // target is any Reg

fun appendSeq(a: InstrSeq, b: InstrSeq) =
    InstrSeq(a.needs + (b.needs - a.modifies), a.modifies + b.modifies, a.stmts + b.stmts)

fun preserving(rs: Set<Reg>, a: InstrSeq, b: InstrSeq): InstrSeq =
    if ((b.needs intersect a.modifies intersect rs).isNotEmpty())
        wrapWith(Save(rs) + Restore(rs), appendSeq(a, b))
    else appendSeq(a, b)
// compileCombination threads operand code through preserving(Proc, Argl, ...);
// tail linkage emits GotoL, so compiled tail calls never grow the simulator stack.
```

### Gradle multi-project layout inside `kotlin/`

```text
kotlin/
  settings.gradle.kts         // include("runtime", "chapter01" .. "chapter05")
  build.gradle.kts            // alias(libs.plugins.kotlin.jvm) apply false; shared ktlint config
  gradle/libs.versions.toml   // the catalog below
  runtime/                    // package sicp.runtime; explicitApi() here
  chapter01/ ... chapter05/   // identical shape:
    build.gradle.kts
    src/main/kotlin/sicp/chNN/
    src/examples/kotlin/sicp/chNN/examples/    // book listings, S*.kt, demo mains
    src/exercises/kotlin/sicp/chNN/exercises/  // E*.kt stubs with TODO("exercise N.M")
    src/solutions/kotlin/sicp/chNN/solutions/  // E*.kt, same FQNs as exercises
    src/test/kotlin/sicp/chNN/                 // Kotest specs, one per subsection
```

Per-chapter build template:

```kotlin
kotlin {
    jvmToolchain(25)
    compilerOptions { freeCompilerArgs.add("-Xexplicit-context-arguments") }
    sourceSets {
        create("examples") { kotlin.srcDir("src/examples/kotlin") }
        create("exercises") { kotlin.srcDir("src/exercises/kotlin") }
        create("solutions") { kotlin.srcDir("src/solutions/kotlin") }
    }
}
dependencies {
    implementation(project(":runtime")); implementation(libs.immutable); implementation(libs.arrow.core)
    testImplementation(libs.kotest.runner.junit5); testImplementation(libs.kotest.property)
    testImplementation(libs.kotest.assertions.core)
}
tasks.withType<Test>().configureEach { useJUnitPlatform() }
```

Mapping rules:

- Exercises and solutions share fully qualified names; the test classpath receives exactly one of them, chosen by the `-Psolutions` property, by adding that source set's output to the test compile and runtime classpaths (normative prose rule; the Provider-based wiring is implementation detail). `gradle test` runs against stubs, `gradle test -Psolutions` against solutions.
- Examples compile in `main` so demo `main` functions run via a JavaExec task and lint like production code.
- Chapter 3 additionally depends on `libs.kotlinx.coroutines.core`, `libs.kotlinx.coroutines.test`, and `libs.arrow.fx.coroutines`; chapter 5 reuses `kotlinx.coroutines.test` for the breakpoint channel test.
- ktlint covers all five source sets and runs in `check` via `lintKotlin`; `explicitApi()` applies to the runtime module.

Version catalog `gradle/libs.versions.toml`:

```toml
[versions]
kotlin = "2.4.20"
arrow = "2.2.3"
coroutines = "1.11.0"
immutable = "0.5.2"
kotest = "6.2.5"
ktlintGradle = "14.2.0"

[libraries]
arrow-core = { module = "io.arrow-kt:arrow-core", version.ref = "arrow" }
arrow-fx-coroutines = { module = "io.arrow-kt:arrow-fx-coroutines", version.ref = "arrow" }
kotlinx-coroutines-core = { module = "org.jetbrains.kotlinx:kotlinx-coroutines-core", version.ref = "coroutines" }
kotlinx-coroutines-test = { module = "org.jetbrains.kotlinx:kotlinx-coroutines-test", version.ref = "coroutines" }
immutable = { module = "org.jetbrains.kotlinx:kotlinx-collections-immutable", version.ref = "immutable" }
kotest-runner-junit5 = { module = "io.kotest:kotest-runner-junit5", version.ref = "kotest" }
kotest-property = { module = "io.kotest:kotest-property", version.ref = "kotest" }
kotest-assertions-core = { module = "io.kotest:kotest-assertions-core", version.ref = "kotest" }

[plugins]
kotlin-jvm = { id = "org.jetbrains.kotlin.jvm", version.ref = "kotlin" }
ktlint = { id = "org.jlleitschuh.gradle.ktlint", version.ref = "ktlintGradle" }
```

The ktlint engine pin goes in the shared build script: `ktlint { version.set("1.8.0") }`.

## 5. Chapter 0 outline: Kotlin for SICP readers (about 30 pages)

The primer teaches only the subset the book uses; every construct appears later, and Chapter 0 is the one place it is explained.

| Section | Pages | Teaches | Feeds into |
|---|---|---|---|
| 0.1 Expressions, names, and control | 5 | `val`, `fun` with expression bodies, `if` and `when` as expressions, blocks, `Long`/`Double` literals, string templates for output | 1.1 |
| 0.2 Functions as values | 5 | function types, lambdas, trailing-lambda style, returning functions, type aliases | 1.3, 3.1 |
| 0.3 Data: products, sums, persistence | 7 | data classes, value classes, sealed hierarchies with exhaustive `when`, `Pair` versus domain types, `PersistentList` basics, `==` versus `===` | chapter 2 |
| 0.4 Errors as values | 5 | `Either`, `Raise`, `either { }`, context parameters with `-Xexplicit-context-arguments`, per-domain sealed errors | 2.1, 3.1, chapter 4 |
| 0.5 Laziness: `lazy` and `Sequence` | 5 | `lazy {}` memoization, `Sequence` pipelines, `generateSequence`, the `sequence { }` builder, cold versus memoized, where `Sequence` is not enough | 3.5, 4.2 |
| 0.6 The test rig and the house style | 3 | Kotest `FunSpec`, `withData`, `checkAll`, `runTest`; the ktlint gate, `allWarningsAsErrors`, `explicitApi()`; the `val` rule and captured-`var` allowed sites; `;Value:` transcripts | all chapters |

Exercises:

- 0.1: translate three Scheme sessions from section 1.1 into Kotlin demo `main` functions producing the matching `// ;Value:` lines.
- 0.2: implement `compose` and `repeated` and property-test them with `checkAll` against hand-computed cases.
- 0.3: model a small binary tree as a sealed hierarchy and write an exhaustive `when` walker computing depth; add a variant and watch the non-exhaustive `when` fail compilation.
- 0.4: refactor a throwing `parseAmount(String): Long` into `Raise<ParseError>` and test both branches through `either { }`.
- 0.5: build the infinite `Sequence` of squares and the memoized `LStream` of squares, then use a counting probe to show which recomputes on a second pass.

## 6. Re-cut list: every divergence from the Scheme text

Exercise numbers stay 1:1 throughout; no renumbering is forced. Additions use the letter suffix (0.1 to 0.5 are new; N.Ma extends exercise N.M). The divergences:

1. Chapter 0 added as a new primer with 0.x numbering (addition).
2. 1.1.5 and exercise 1.5: the applicative-order probe re-cut as an eager-parameter versus lambda-parameter demonstration.
3. 1.2: iterative processes re-cut onto `tailrec` funs or `while`; no implicit TCO; `DeepRecursiveFunction` offered for deep non-tail tree recursion.
4. 1.1 and 1.2 transcripts: `;Value:` rendered as comments; `runtime` re-cut as `measureNanoTime` (exercise 1.22 keeps its number).
5. 2.2.4: the picture language outputs SVG files; combinator names and algebra unchanged.
6. 2.3.1 onward: quotation re-cut onto `VSym`/`vlist` construction (no reader); from 2.3.2 symbolic programs use sealed classes.
7. 2.4 with 3.3.3: data direction re-cut onto the immutable `OpTable` registry; `put`/`get` names and two-key shape kept.
8. 2.5: tower integers `Long` with checked exact arithmetic and `BigInteger` exact rationals; tower levels otherwise 1:1.
9. 3.2 re-cut: retitled around the closure and captured-`var` model; diagrams drawn against closure cells; exercises 3.9 to 3.11 keep numbers and intent.
10. 3.4 re-cut: structured coroutines; `parallel-execute` to `launch`, serializer to `Mutex`, `test-and-set!` to `AtomicBoolean.compareAndSet`, semaphore (3.47) to a token channel; exercises 3.38 to 3.49 keep their numbers.
11. 3.5.1: dual stream representation; the `Sequence` limit is named and the switch to `LStream` taught there.
12. 4.1, 4.2, 4.3, 5.4 drivers: no reader; top-level programs are prebuilt `Value` lists, stated once as a shared deviation.
13. 4.3: amb re-cut to `Sequence` search over persistent environments; continuation machinery replaced by iterator advance; exercises 4.35 to 4.54 keep their numbers.
14. 4.4: frames as `PersistentMap`, `stream-append-delayed` as an explicit `interleave` helper; `never-no-loop` relies on cold sequences.
15. Chapter 5: machine descriptions are a Kotlin DSL building the same `Stmt` data the simulator, explicit-control evaluator, and compiler consume; code-as-data preserved.
16. Tooling: detekt dropped for the ktlint gate; conventions text reflects this.

## 7. Anchor files

- `sicp-pocket.texi`: source of truth for all 22 sections and every exercise anchor in this map
- `kotlin/gradle/libs.versions.toml`: version catalog to create; carries the verified pins of section 0
- `kotlin/settings.gradle.kts`: declares the runtime module and the five chapter subprojects
- `kotlin/chapterNN/build.gradle.kts`: template for the five source sets, toolchain 25, and the `-Psolutions` XOR classpath rule
- `kotlin/runtime/src/main/kotlin/sicp/runtime/`: shared `Value`, `LStream`, `OpTable`, `Env`, `Machine`, and query engine all chapters import