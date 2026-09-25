// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.5, the stream machinery shared by the exercises:
// the prose definitions of 3.5.1 to 3.5.5 (stream-map, add-streams, the
// integers, the sieve, sqrt-stream, euler-transform, the delayed
// integral, solve, and the Monte Carlo streams) appear once here so the
// exercise files build on them instead of redefining each other.

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/** The book's `stream-ref`: the element at index [n], forcing memoized
 * tails on the way. */
public fun <T> streamRef(
    s: LStream<T>,
    n: Int,
): T {
    var cursor = s
    repeat(n) { cursor = cursor.streamTail() }
    return checkNotNull(cursor.streamHead()) { "stream-ref $n past the end of the stream" }
}

/** Folds the stream's elements into [initial] with [f], forcing each
 * memoized tail once; infinite streams are the caller's responsibility
 * to bound first. */
public fun <T, R> streamFold(
    s: LStream<T>,
    initial: R,
    f: (R, T) -> R,
): R {
    var acc = initial
    var cursor = s
    while (cursor is LStream.Cons) {
        acc = f(acc, cursor.head)
        cursor = cursor.tail
    }
    return acc
}

/** The book's single-stream `stream-map`: [f] over every element, one
 * delayed cons per node. */
public fun <T, R> streamMap(
    f: (T) -> R,
    s: LStream<T>,
): LStream<R> =
    when (s) {
        is LStream.Empty -> LStream.Empty
        is LStream.Cons -> consStream(f(s.head)) { streamMap(f, s.tail) }
    }

/** The book's two-stream map body: walks both streams in step, the
 * shape exercise 3.50 generalizes to n streams. */
public fun <A, B, R> zipStream(
    s1: LStream<A>,
    s2: LStream<B>,
    f: (A, B) -> R,
): LStream<R> =
    if (s1 is LStream.Cons && s2 is LStream.Cons) {
        consStream(f(s1.head, s2.head)) { zipStream(s1.tail, s2.tail, f) }
    } else {
        LStream.Empty
    }

/** The book's `stream-filter`. */
public fun <T> streamFilter(
    p: (T) -> Boolean,
    s: LStream<T>,
): LStream<T> =
    when (s) {
        is LStream.Empty -> {
            LStream.Empty
        }

        is LStream.Cons -> {
            if (p(s.head)) {
                consStream(s.head) { streamFilter(p, s.tail) }
            } else {
                streamFilter(p, s.tail)
            }
        }
    }

/** The book's `stream-for-each`, applying [action] until the stream
 * runs out. */
public fun <T> streamForEach(
    action: (T) -> Unit,
    s: LStream<T>,
) {
    var cursor = s
    while (cursor is LStream.Cons) {
        action(cursor.head)
        cursor = cursor.tail
    }
}

/** The book's `display-stream`, rendered as newline-separated lines so a
 * test can assert the printed output; finite streams only, callers take
 * a prefix of infinite ones first. */
public fun displayStream(s: LStream<*>): String =
    buildString {
        var cursor: LStream<*> = s
        while (cursor is LStream.Cons) {
            append(cursor.head.toString()).append('\n')
            cursor = cursor.tail
        }
    }

/** The book's `show`, as a recording probe: appends the element the way
 * [displayStream] would print it and returns the element, so a pipeline
 * can be watched while it is consumed. */
public fun <T> show(
    x: T,
    log: MutableList<String>,
): T {
    log.add(x.toString())
    return x
}

/** The book's `stream-enumerate-interval`, both ends inclusive. */
public fun streamEnumerateInterval(
    low: Long,
    high: Long,
): LStream<Long> =
    if (low > high) {
        LStream.Empty
    } else {
        consStream(low) { streamEnumerateInterval(low + 1L, high) }
    }

/** The book's `add-streams` over integer streams, failing loudly rather
 * than wrapping on overflow. */
public fun addStreams(
    s1: LStream<Long>,
    s2: LStream<Long>,
): LStream<Long> = zipStream(s1, s2) { a, b -> Math.addExact(a, b) }

/** The book's `add-streams` over real streams. */
@JvmName("addStreamsR")
public fun addStreams(
    s1: LStream<Double>,
    s2: LStream<Double>,
): LStream<Double> = zipStream(s1, s2) { a, b -> a + b }

/** The book's `scale-stream` over integer streams, overflow checked. */
public fun scaleStream(
    s: LStream<Long>,
    factor: Long,
): LStream<Long> = streamMap({ Math.multiplyExact(factor, it) }, s)

/** The book's `scale-stream` over real streams. */
@JvmName("scaleStreamR")
public fun scaleStream(
    s: LStream<Double>,
    factor: Double,
): LStream<Double> = streamMap({ factor * it }, s)

/** The book's `integers-starting-from`, one memoized tail per step. */
public fun integersStartingFrom(n: Long): LStream<Long> = consStream(n) { integersStartingFrom(Math.addExact(n, 1L)) }

/** The book's `integers`: 1, 2, 3, ... */
public val integers: LStream<Long> = integersStartingFrom(1L)

/** The book's `ones`, defined in terms of its own tail. */
public val ones: LStream<Long> = consStream(1L) { ones }

/** The book's `fibs`, the self-referential Fibonacci stream: each tail
 * is forced once and memoized, which is what exercise 3.57 counts. */
public val fibs: LStream<Long> =
    consStream(0L) {
        consStream(1L) { zipStream(fibs, fibs.streamTail()) { a, b -> Math.addExact(a, b) } }
    }

/** The book's `prime?` of 1.2.6: trial division. */
public fun isPrime(n: Long): Boolean {
    if (n < 2L) {
        return false
    }
    var d = 2L
    while (d * d <= n) {
        if (n % d == 0L) {
            return false
        }
        d += 1L
    }
    return true
}

/** The 3.5.2 sieve over a cold host `Sequence`: each recursive stage
 * filters a fresh cold sequence, so no self-reference is needed and the
 * pipeline shows the stream idea on the stdlib regime. */
public fun sieve(s: Sequence<Long>): Sequence<Long> =
    sequence {
        val p = s.first()
        yield(p)
        yieldAll(sieve(s.drop(1).filter { it % p != 0L }))
    }

/** The book's sieve as a memoized stream: `primes` is defined in terms
 * of its own filtered tail. */
public fun sieveStream(s: LStream<Long>): LStream<Long> =
    when (s) {
        is LStream.Empty -> LStream.Empty
        is LStream.Cons -> consStream(s.head) { sieveStream(streamFilter({ it % s.head != 0L }, s.tail)) }
    }

/** The book's `primes`, the memoized sieve cascade over the integers. */
public val primes: LStream<Long> = sieveStream(integersStartingFrom(2L))

/** The book's `stream-append` of 3.5.3: all of [s1], then all of [s2]. */
public fun <T> streamAppend(
    s1: LStream<T>,
    s2: LStream<T>,
): LStream<T> =
    when (s1) {
        is LStream.Empty -> s2
        is LStream.Cons -> consStream(s1.head) { streamAppend(s1.tail, s2) }
    }

/** The book's `interleave` of 3.5.3: alternating heads so every element
 * of both streams eventually appears. */
public fun <T> interleave(
    s1: LStream<T>,
    s2: LStream<T>,
): LStream<T> =
    when (s1) {
        is LStream.Empty -> s2
        is LStream.Cons -> consStream(s1.head) { interleave(s2, s1.tail) }
    }

/** The book's `pairs` of 3.5.3: the diagonal pair, then the rest of the
 * first row interleaved with the pairs of the tails. */
public fun pairs(
    s: LStream<Long>,
    t: LStream<Long>,
): LStream<Pair<Long, Long>> {
    val sh = s.streamHead() ?: return LStream.Empty
    val th = t.streamHead() ?: return LStream.Empty
    return consStream(sh to th) {
        interleave(
            streamMap({ x -> sh to x }, t.streamTail()),
            pairs(s.streamTail(), t.streamTail()),
        )
    }
}

/** The book's `sqrt-improve`: one Newton step toward [x]. */
public fun sqrtImprove(
    guess: Double,
    x: Double,
): Double = (guess + x / guess) / 2.0

/** The book's `sqrt-stream` of 3.5.3 with the local [guesses] stream
 * that exercise 3.63 turns on: the stream is built once per call, so
 * every caller walks the same memoized nodes. Kotlin locals cannot
 * appear in their own initializer, so the tail thunk reads the cell
 * that the last line fills before anything can force a tail. */
public fun sqrtStream(x: Double): LStream<Double> {
    var guesses: LStream<Double>? = null
    val tied: LStream<Double> =
        consStream(1.0) {
            streamMap({ g -> sqrtImprove(g, x) }, checkNotNull(guesses) { "guesses not yet tied" })
        }
    guesses = tied
    return tied
}

/** The book's `euler-transform` of 3.5.3: S_{n+1} corrected by the
 * square of its last step over the difference of steps, one acceleration
 * step per element. */
public fun eulerTransform(s: LStream<Double>): LStream<Double> {
    val s0 = s.streamHead() ?: return LStream.Empty
    val s1 = s.streamTail().streamHead() ?: return LStream.Empty
    val s2 = s.streamTail().streamTail().streamHead() ?: return LStream.Empty
    val d21 = s2 - s1
    val d01 = s0 - s1
    val denominator = d21 + d01
    val head = if (denominator == 0.0) s2 else s2 - d21 * d21 / denominator
    return consStream(head) { eulerTransform(s.streamTail()) }
}

/** The book's `make-tableau`: the stream of streams where each row is
 * [transform] applied to the previous one. */
public fun makeTableau(
    transform: (LStream<Double>) -> LStream<Double>,
    s: LStream<Double>,
): LStream<LStream<Double>> = consStream(s) { makeTableau(transform, transform(s)) }

/** The book's `accelerated-sequence`: the first element of every
 * tableau row. */
public fun acceleratedSequence(
    transform: (LStream<Double>) -> LStream<Double>,
    s: LStream<Double>,
): LStream<Double> =
    streamMap<LStream<Double>, Double>(
        { checkNotNull(it.streamHead()) { "empty tableau row" } },
        makeTableau(transform, s),
    )

/** The delayed-integrand `integral` of 3.5.4: the integrand is forced
 * only past the first element, which is what lets `solve`'s feedback
 * loop close. The book's internal `define` of `int` becomes the same
 * tie-before-force cell as [sqrtStream]. */
public fun integral(
    delayedIntegrand: Lazy<LStream<Double>>,
    initialValue: Double,
    dt: Double,
): LStream<Double> {
    var int: LStream<Double>? = null
    val tied: LStream<Double> =
        consStream(initialValue) {
            addStreams(scaleStream(delayedIntegrand.value, dt), checkNotNull(int) { "int not yet tied" })
        }
    int = tied
    return tied
}

/** The book's `solve` of 3.5.4: the equation dy/dt = f(y) as a feedback
 * loop. The delayed integrand is the book's `(delay dy)`, tied back to
 * [y] itself inside the lazy value. */
public fun solve(
    f: (Double) -> Double,
    y0: Double,
    dt: Double,
): LStream<Double> {
    var dy: LStream<Double>? = null
    val y: LStream<Double> = integral(lazy { checkNotNull(dy) { "dy not yet tied" } }, y0, dt)
    dy = streamMap(f, y)
    return y
}

/** Exact rational coefficient for the series exercises: [num]/[den] with
 * [den] > 0 and gcd reduced. `Long` components cover every pinned series
 * term; the exactness is the point, so 1/3 never becomes a `Double`.
 * Build with [of], which reduces; the constructor demands the reduced
 * form. */
public data class Rat(
    public val num: Long,
    public val den: Long,
) : Comparable<Rat> {
    init {
        require(den > 0L) { "denominator must be positive" }
        require(num == 0L || gcdReduced(num, den)) { "rational must be gcd-reduced" }
    }

    public operator fun plus(other: Rat): Rat =
        of(
            Math.addExact(Math.multiplyExact(num, other.den), Math.multiplyExact(other.num, den)),
            Math.multiplyExact(den, other.den),
        )

    public operator fun minus(other: Rat): Rat =
        of(
            Math.subtractExact(Math.multiplyExact(num, other.den), Math.multiplyExact(other.num, den)),
            Math.multiplyExact(den, other.den),
        )

    public operator fun times(other: Rat): Rat = of(Math.multiplyExact(num, other.num), Math.multiplyExact(den, other.den))

    public operator fun div(other: Rat): Rat {
        check(other.num != 0L) { "division by the zero rational" }
        return of(Math.multiplyExact(num, other.den), Math.multiplyExact(den, other.num))
    }

    public operator fun unaryMinus(): Rat = of(-num, den)

    public fun toDouble(): Double = num.toDouble() / den.toDouble()

    public override fun compareTo(other: Rat): Int {
        val lhs = Math.multiplyExact(num, other.den)
        val rhs = Math.multiplyExact(other.num, den)
        return lhs.compareTo(rhs)
    }

    public override fun toString(): String = if (den == 1L) num.toString() else "$num/$den"

    public companion object {
        /** The zero rational. */
        public val ZERO: Rat = of(0L)

        /** The one rational. */
        public val ONE: Rat = of(1L)

        /** Builds the reduced rational [num]/[den]; [den] defaults to 1. */
        public fun of(
            num: Long,
            den: Long = 1L,
        ): Rat {
            require(den != 0L) { "zero denominator" }
            if (num == 0L) {
                return Rat(0L, 1L)
            }
            val sign = if (den < 0L) -1L else 1L
            val n = Math.multiplyExact(num, sign)
            val d = Math.multiplyExact(den, sign)
            val g = gcd(Math.abs(n), d)
            return Rat(n / g, d / g)
        }

        private fun gcdReduced(
            a: Long,
            b: Long,
        ): Boolean = gcd(Math.abs(a), b) == 1L

        private fun gcd(
            a: Long,
            b: Long,
        ): Long {
            var x = a
            var y = b
            while (y != 0L) {
                val t = x % y
                x = y
                y = t
            }
            return x
        }
    }
}

/** The 3.1.2 `rand-update` seed word, shared with the earlier section so
 * the 3.5.5 streams draw from the same generator. */
public const val RANDOM_INIT: ULong = 42UL

/** The 3.5.5 `rand-update` step (the 3.1.2 xorshift64* word update) as
 * a pure function; named [randNext] to stay clear of the private copies
 * the 3.1 exercise files carry under the book's name. */
public fun randNext(x: ULong): ULong {
    var y = x
    y = y xor (y shr 12)
    y = y xor (y shl 25)
    y = y xor (y shr 27)
    return y * XORSHIFT_MULTIPLIER_35
}

private const val XORSHIFT_MULTIPLIER_35: ULong = 0x2545F4914F6CDD1DUL

/** The 3.5.5 `random-numbers`: no generator object, just the stream of
 * successive update values, defined in terms of its own tail. */
public val randomNumbers: LStream<ULong> = consStream(RANDOM_INIT) { streamMap(::randNext, randomNumbers) }

/** The book's `map-successive-pairs`: [f] over consecutive pairs of
 * [s], consuming two elements per output. */
public fun <A, R> mapSuccessivePairs(
    f: (A, A) -> R,
    s: LStream<A>,
): LStream<R> {
    val a0 = s.streamHead() ?: return LStream.Empty
    val a1 = s.streamTail().streamHead() ?: return LStream.Empty
    return consStream(f(a0, a1)) { mapSuccessivePairs(f, s.streamTail().streamTail()) }
}

/** The book's streaming `monte-carlo`: the running estimate after each
 * experiment outcome. */
public fun monteCarloStream(
    experiments: LStream<Boolean>,
    passed: Long,
    failed: Long,
): LStream<Double> {
    val outcome = experiments.streamHead() ?: return LStream.Empty
    val rest = experiments.streamTail()
    val p: Long
    val q: Long
    if (outcome) {
        p = Math.addExact(passed, 1L)
        q = failed
    } else {
        p = passed
        q = Math.addExact(failed, 1L)
    }
    return consStream(p.toDouble() / (p + q).toDouble()) { monteCarloStream(rest, p, q) }
}

/** The Euclidean gcd over unsigned words, for the Cesaro experiment. */
public fun gcdULong(
    a: ULong,
    b: ULong,
): ULong {
    var x = a
    var y = b
    while (y != 0UL) {
        val t = x % y
        x = y
        y = t
    }
    return x
}
