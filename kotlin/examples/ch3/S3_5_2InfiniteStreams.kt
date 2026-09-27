// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.5.2, infinite streams

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamTail
import sicp.runtime.take

/** The book's `integers-starting-from`: one memoized tail per step. */
public fun integersStartingFrom(n: Long): LStream<Long> = consStream(n) { integersStartingFrom(Math.addExact(n, 1L)) }

/** The book's `integers`: 1, 2, 3, ... */
public val integers: LStream<Long> = integersStartingFrom(1L)

/** The book's `divisible?`: does [y] divide [x] exactly. */
private fun divisible(
    x: Long,
    y: Long,
): Boolean = x % y == 0L

/** The book's `no-sevens`: the integers the sieve of 7 does not catch. */
public val noSevens: LStream<Long> = streamFilter({ x -> !divisible(x, 7L) }, integers)

/** The book's `fibs`, the self-referential Fibonacci stream: each tail is
 * forced once and memoized, which is what exercise 3.57 counts. */
public val fibs: LStream<Long> =
    consStream(0L) {
        consStream(1L) { zipStream(fibs, fibs.streamTail()) { a, b -> Math.addExact(a, b) } }
    }

/** The 3.5.2 sieve over a cold host `Sequence`: each recursive stage
 * filters a fresh cold sequence, so no self-reference is needed and the
 * pipeline shows the stream idea on the stdlib regime. */
public fun sieve(s: Sequence<Long>): Sequence<Long> =
    sequence {
        val p = s.first()
        yield(p)
        yieldAll(sieve(s.drop(1).filter { !divisible(it, p) }))
    }

/** The book's sieve as a memoized stream: `primes` is defined in terms of
 * its own filtered tail. */
public fun sieveStream(s: LStream<Long>): LStream<Long> =
    when (s) {
        is LStream.Empty -> LStream.Empty
        is LStream.Cons -> consStream(s.head) { sieveStream(streamFilter({ x -> !divisible(x, s.head) }, s.tail)) }
    }

/** The book's `primes`, the memoized sieve cascade over the integers. */
public val primes: LStream<Long> = sieveStream(integersStartingFrom(2L))

/** The book's `ones`, defined in terms of its own tail. */
public val ones: LStream<Long> = consStream(1L) { ones }

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

/** The book's second definition of `integers`, the implicit one: each
 * tail is the sum of `ones` and the stream itself. Kotlin forbids
 * redefining a top-level val, so the book's redefinition of `integers`
 * carries this distinct name; the two agree element for element. */
public val integersImplicit: LStream<Long> = consStream(1L) { addStreams(ones, integersImplicit) }

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

/** The book's `double`, defined implicitly: every tail is the stream
 * scaled by 2, so the elements are the powers of two. */
public val double: LStream<Long> = consStream(1L) { scaleStream(double, 2L) }

/** Builds the finite stream with exactly the heads [xs], so a test can
 * display a prefix of an infinite stream without displaying forever. */
private fun <T> finiteStream(xs: List<T>): LStream<T> =
    if (xs.isEmpty()) {
        LStream.Empty
    } else {
        consStream(xs.first()) { finiteStream(xs.drop(1)) }
    }

public class S3_5_2InfiniteStreamsTest :
    FunSpec({
        test("the integers go on forever") {
            integers.take(7) shouldBe listOf(1L, 2L, 3L, 4L, 5L, 6L, 7L)
        }

        test("no-sevens: element 100 is 117") {
            streamRef(noSevens, 100) shouldBe 117L
        }

        test("fibs: the self-referential Fibonacci stream") {
            fibs.take(10) shouldBe listOf(0L, 1L, 1L, 2L, 3L, 5L, 8L, 13L, 21L, 34L)
        }

        test("the stream-cdr of fibs displays 1 1 2 3 5 8 13 21") {
            displayStream(finiteStream(fibs.streamTail().take(8))) shouldBe "1\n1\n2\n3\n5\n8\n13\n21\n"
        }

        test("the sieve over a cold Sequence yields the primes") {
            sieve(generateSequence(2L) { it + 1L }).take(10).toList() shouldBe
                listOf(2L, 3L, 5L, 7L, 11L, 13L, 17L, 19L, 23L, 29L)
        }

        test("the memoized sieve cascade: element 50 of primes is 233") {
            primes.take(10) shouldBe listOf(2L, 3L, 5L, 7L, 11L, 13L, 17L, 19L, 23L, 29L)
            streamRef(primes, 50) shouldBe 233L
        }

        test("ones: a stream defined in terms of its own tail") {
            ones.take(3) shouldBe listOf(1L, 1L, 1L)
        }

        test("the implicit integers agree with the explicit ones") {
            integersImplicit.take(7) shouldBe listOf(1L, 2L, 3L, 4L, 5L, 6L, 7L)
        }

        test("add-streams of the integers with themselves gives the even numbers") {
            addStreams(integers, integers).take(10) shouldBe
                listOf(2L, 4L, 6L, 8L, 10L, 12L, 14L, 16L, 18L, 20L)
        }

        test("double: the powers of two, defined in terms of itself") {
            double.take(10) shouldBe
                listOf(1L, 2L, 4L, 8L, 16L, 32L, 64L, 128L, 256L, 512L)
        }
    })
