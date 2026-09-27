// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.5.1, streams are delayed lists

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.asSequence
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * The book's first `sum-primes`, written in the standard iterative style:
 * one pass over the interval [a, b] that adds each prime as it is counted.
 * The stream formulation of the same sum, asserted in the test class below,
 * never holds the whole interval in memory.
 */
public fun sumPrimes(
    a: Long,
    b: Long,
): Long {
    var count = a
    var accum = 0L
    while (count <= b) {
        if (isPrime(count)) {
            accum += count
        }
        count += 1L
    }
    return accum
}

/** The book's `prime?` of 1.2.6: trial division up to the square root. */
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

/** The book's `stream-ref`: the element at index [n], forcing delayed
 * tails on the way. */
public fun <T> streamRef(
    s: LStream<T>,
    n: Int,
): T {
    var cursor = s
    repeat(n) { cursor = cursor.streamTail() }
    return checkNotNull(cursor.streamHead()) { "stream-ref $n past the end of the stream" }
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

/** The book's two-stream map body, the shape exercise 3.50 generalizes
 * to n streams: both streams walk in step. */
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
 * test can assert what the display would show; the caller bounds an
 * infinite stream to a prefix first. */
public fun displayStream(s: LStream<*>): String =
    buildString {
        var cursor: LStream<*> = s
        while (cursor is LStream.Cons) {
            append(cursor.head.toString()).append('\n')
            cursor = cursor.tail
        }
    }

/** The book's `show`: appends the element the way [displayStream] would
 * print it and returns the element, so a pipeline can be watched while it
 * is consumed. The lines land in [log]. */
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

/**
 * The book's `memo-proc`, the memoization behind `delay`: the first call
 * runs [proc] and stores the result, every later call returns the stored
 * result. As in the book, where the stored sentinel is `false`, a null
 * result cannot be represented; the stream heads this section memoizes
 * are never null.
 */
public fun <T> memoProc(proc: () -> T): () -> T {
    var alreadyRun = false
    var result: T? = null
    return {
        if (!alreadyRun) {
            result = proc()
            alreadyRun = true
        }
        checkNotNull(result) { "memo-proc cannot store a null result" }
    }
}

public class S3_5_1DelayedListsTest :
    FunSpec({
        test("the second prime of the interval, on demand") {
            streamRef(streamFilter(::isPrime, streamEnumerateInterval(10000L, 1000000L)), 1) shouldBe 10009L
        }

        test("sumPrimes in the standard iterative style") {
            sumPrimes(10000L, 1000000L) shouldBe 37544665627L
        }

        test("the stream formulation sums the same primes incrementally") {
            streamFilter(::isPrime, streamEnumerateInterval(10000L, 1000000L)).asSequence().sum() shouldBe 37544665627L
        }

        test("a cold Sequence pipeline recomputes from scratch but agrees") {
            (10000L..1000000L).asSequence().filter(::isPrime).sum() shouldBe 37544665627L
        }

        test("memo-proc forces its thunk once; a plain lambda forces it every time") {
            var thunkCalls = 0
            val memoized: () -> LStream<Long> =
                memoProc {
                    thunkCalls += 1
                    streamEnumerateInterval(1L, 3L)
                }
            memoized()
            memoized()
            thunkCalls shouldBe 1

            val unmemoized: () -> LStream<Long> = {
                thunkCalls += 1
                streamEnumerateInterval(1L, 3L)
            }
            unmemoized()
            unmemoized()
            thunkCalls shouldBe 3
        }

        test("show interleaved with stream-map: each element printed once") {
            val log = mutableListOf<String>()
            val x = streamMap({ show(it, log) }, streamEnumerateInterval(0, 10))
            log.toList() shouldBe listOf("0")
            streamRef(x, 5)
            log.toList() shouldBe listOf("0", "1", "2", "3", "4", "5")
            streamRef(x, 7)
            log.toList() shouldBe listOf("0", "1", "2", "3", "4", "5", "6", "7")
        }

        test("accum interleaved with stream-map: the sum tracks what was forced") {
            var sum = 0L
            val seq =
                streamMap({ x ->
                    sum += x
                    sum
                }, streamEnumerateInterval(1, 20))
            sum shouldBe 1L
            val y = streamFilter({ it % 2 == 0L }, seq)
            sum shouldBe 6L
            streamRef(y, 0) shouldBe 6L
            val z = streamFilter({ it % 5 == 0L }, seq)
            sum shouldBe 10L
            streamRef(z, 0) shouldBe 10L
            streamRef(y, 7) shouldBe 136L
            sum shouldBe 136L
            displayStream(z) shouldBe "10\n15\n45\n55\n105\n120\n190\n210\n"
            sum shouldBe 210L
        }
    })
