// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.5.3, exploiting the stream paradigm

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail
import sicp.runtime.take

/** The book's `average` of 1.3.3, private because only the Newton step
 * below needs it in this file. */
private fun average(
    x: Double,
    y: Double,
): Double = (x + y) / 2.0

/** The book's `sqrt-improve`: one Newton step toward [x]. */
public fun sqrtImprove(
    guess: Double,
    x: Double,
): Double = average(guess, x / guess)

/** The book's `sqrt-stream` of 3.5.3 with the local `guesses` stream that
 * exercise 3.63 turns on: the stream is built once per call, so every
 * caller walks the same memoized nodes. Kotlin locals cannot appear in
 * their own initializer, so the tail thunk reads the cell that the last
 * line fills before anything can force a tail. */
public fun sqrtStream(x: Double): LStream<Double> {
    var guesses: LStream<Double>? = null
    val tied: LStream<Double> =
        consStream(1.0) {
            streamMap({ g -> sqrtImprove(g, x) }, checkNotNull(guesses) { "guesses not yet tied" })
        }
    guesses = tied
    return tied
}

/** The book's `partial-sums` of exercise 3.55, the procedure the pi
 * stream needs: each element is the sum of the elements of [s] up to
 * that point, in the implicit style of the `integers`. The running sum
 * feeds back into its own tail, so the tail thunk reads the cell that
 * the last line fills before anything can force a tail. Kept private so
 * the exercise's own answer stays the public one. */
private fun partialSums(s: LStream<Double>): LStream<Double> {
    val head = s.streamHead() ?: return LStream.Empty
    var ps: LStream<Double>? = null
    val tied: LStream<Double> =
        consStream(head) {
            addStreams(s.streamTail(), checkNotNull(ps) { "ps not yet tied" })
        }
    ps = tied
    return tied
}

/** The book's `pi-summands`: the reciprocals of the odd integers with
 * alternating signs. */
public fun piSummands(n: Long): LStream<Double> = consStream(1.0 / n.toDouble()) { streamMap({ x -> -x }, piSummands(n + 2L)) }

/** The book's `pi-stream`: 4 times the partial sums of the alternating
 * odd series, a stream of better and better pi approximations. */
public val piStream: LStream<Double> = scaleStream(partialSums(piSummands(1L)), 4.0)

/** The book's `euler-transform`: one acceleration step per element,
 * correcting S_(n+1) by the square of its last step over the difference
 * of steps. */
public fun eulerTransform(s: LStream<Double>): LStream<Double> {
    val s0 = s.streamHead() ?: return LStream.Empty
    val s1 = s.streamTail().streamHead() ?: return LStream.Empty
    val s2 = s.streamTail().streamTail().streamHead() ?: return LStream.Empty
    val denominator = s0 - 2.0 * s1 + s2
    val head = if (denominator == 0.0) s2 else s2 - (s2 - s1) * (s2 - s1) / denominator
    return consStream(head) { eulerTransform(s.streamTail()) }
}

/** The book's `make-tableau`: the stream of streams where each row is
 * [transform] applied to the previous one. */
public fun makeTableau(
    transform: (LStream<Double>) -> LStream<Double>,
    s: LStream<Double>,
): LStream<LStream<Double>> = consStream(s) { makeTableau(transform, transform(s)) }

/** The book's `accelerated-sequence`: the first element of every tableau
 * row. */
public fun acceleratedSequence(
    transform: (LStream<Double>) -> LStream<Double>,
    s: LStream<Double>,
): LStream<Double> =
    streamMap<LStream<Double>, Double>(
        { checkNotNull(it.streamHead()) { "empty tableau row" } },
        makeTableau(transform, s),
    )

/** The book's `stream-append` of 3.5.3: all of [s1], then all of [s2].
 * The text keeps it as the step on the way to `interleave`, since an
 * append over an infinite first stream never reaches the second. */
public fun <T> streamAppend(
    s1: LStream<T>,
    s2: LStream<T>,
): LStream<T> =
    when (s1) {
        is LStream.Empty -> s2
        is LStream.Cons -> consStream(s1.head) { streamAppend(s1.tail, s2) }
    }

/** The book's `interleave`: alternating heads, so every element of both
 * streams eventually appears even when the first stream is infinite. */
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

/** The book's `integral` of 3.5.3, the signal integrator: the internal
 * `int` stream is defined in terms of itself, the feedback loop of
 * figure 3.32. The integrand is an ordinary argument here; the delayed
 * variant in 3.5.4 is the one that closes `solve`'s loop. */
public fun integral(
    integrand: LStream<Double>,
    initialValue: Double,
    dt: Double,
): LStream<Double> {
    var int: LStream<Double>? = null
    val tied: LStream<Double> =
        consStream(initialValue) {
            addStreams(scaleStream(integrand, dt), checkNotNull(int) { "int not yet tied" })
        }
    int = tied
    return tied
}

/** Builds the finite stream with exactly the heads [xs], so a test can
 * display a prefix of an infinite stream without displaying forever. */
private fun finiteStream(xs: List<Double>): LStream<Double> =
    if (xs.isEmpty()) {
        LStream.Empty
    } else {
        consStream(xs.first()) { finiteStream(xs.drop(1)) }
    }

private fun displayLines(s: LStream<Double>): String = displayStream(finiteStream(s.take(8)))

public class S3_5_3StreamParadigmTest :
    FunSpec({
        test("sqrt-stream: the guesses converge on the square root of 2") {
            sqrtStream(2.0).take(5) shouldBe
                listOf(1.0, 1.5, 1.4166666666666665, 1.4142156862745097, 1.4142135623746899)
        }

        test("pi-stream: the alternating series of odd reciprocals, scaled by 4") {
            displayLines(piStream) shouldBe
                "4.0\n2.666666666666667\n3.466666666666667\n2.8952380952380956\n" +
                "3.3396825396825403\n2.9760461760461765\n3.2837384837384844\n3.017071817071818\n"
        }

        test("euler-transform accelerates the pi sequence") {
            displayLines(eulerTransform(piStream)) shouldBe
                "3.166666666666667\n3.1333333333333337\n3.1452380952380956\n3.13968253968254\n" +
                "3.1427128427128435\n3.1408813408813416\n3.142071817071818\n3.1412548236077655\n"
        }

        test("the accelerated sequence: eight terms give pi to 14 places") {
            displayLines(acceleratedSequence(::eulerTransform, piStream)) shouldBe
                "4.0\n3.166666666666667\n3.142105263157895\n3.141599357319005\n" +
                "3.1415927140337785\n3.1415926539752927\n3.1415926535911765\n3.141592653589778\n"
        }

        test("stream-append drains the first stream before the second") {
            streamAppend(
                consStream(1L) { consStream(2L) { LStream.Empty } },
                consStream(3L) { consStream(4L) { LStream.Empty } },
            ).take(4) shouldBe listOf(1L, 2L, 3L, 4L)
        }

        test("interleave alternates heads and reaches both streams") {
            interleave(
                consStream(1L) { consStream(3L) { consStream(5L) { LStream.Empty } } },
                consStream(2L) { consStream(4L) { LStream.Empty } },
            ).take(5) shouldBe listOf(1L, 2L, 3L, 4L, 5L)
        }

        test("pairs of the integers, diagonal first, rows interleaved") {
            pairs(integers, integers).take(10) shouldBe
                listOf(
                    1L to 1L,
                    1L to 2L,
                    2L to 2L,
                    1L to 3L,
                    2L to 3L,
                    1L to 4L,
                    3L to 3L,
                    1L to 5L,
                    2L to 4L,
                    1L to 6L,
                )
        }

        test("integral of a constant signal integrates like the implicit integers") {
            integral(streamMap({ it.toDouble() }, ones), 0.0, 1.0).take(5) shouldBe
                listOf(0.0, 1.0, 2.0, 3.0, 4.0)
        }
    })
