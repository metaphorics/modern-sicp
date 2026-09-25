// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.5.5, modularity of functional programs and modularity of objects

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail
import sicp.runtime.take
import kotlin.math.PI
import kotlin.math.abs
import kotlin.math.sqrt

/** The book's `random-numbers`: no generator object, just the stream of
 * successive `randUpdate` values starting from the book's `random-init`
 * (42), defined in terms of its own tail. `randUpdate` is the same pure
 * word step the 3.1.2 section shares. */
public val randomNumbers: LStream<ULong> = consStream(42UL) { streamMap(::randUpdate, randomNumbers) }

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

/** The Euclidean gcd over unsigned words, the `gcd` the Cesaro
 * experiment needs. */
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

/** The book's `cesaro-stream`: true when two consecutive random words
 * are coprime, the outcome of one Cesaro experiment. */
public val cesaroStream: LStream<Boolean> = mapSuccessivePairs({ r1, r2 -> gcdULong(r1, r2) == 1UL }, randomNumbers)

/** The book's streaming `monte-carlo`: the running estimate of the
 * experiment's probability after each outcome. */
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

/** The book's `pi`: the stream of Cesaro estimates of pi, better the
 * farther one looks into it. */
public val piEstimates: LStream<Double> = streamMap({ p -> sqrt(6.0 / p) }, monteCarloStream(cesaroStream, 0L, 0L))

/** The book's `stream-withdraw`: the balance history as a mathematical
 * function of the amounts. One memoized node per step, no assignment and
 * no local state; the tail is `[balance]` once the amounts run out. */
public fun streamWithdraw(
    balance: Long,
    amountStream: LStream<Long>,
): LStream<Long> =
    when (amountStream) {
        is LStream.Empty -> consStream(balance) { LStream.Empty }
        is LStream.Cons -> consStream(balance) { streamWithdraw(balance - amountStream.head, amountStream.tail) }
    }

public class S3_5_5ModularityTest :
    FunSpec({
        test("random-numbers: the rand closure of 3.1.2 becomes a stream") {
            randomNumbers.take(6) shouldBe
                listOf(
                    42UL,
                    6255019084209693600UL,
                    15601610542105701163UL,
                    15816722976141896202UL,
                    3582986960528580676UL,
                    13248748137499627717UL,
                )
        }

        test("the Cesaro monte-carlo stream estimates pi after 200 experiments") {
            val estimate = streamRef(piEstimates, 199)
            (abs(estimate - PI) < 0.5) shouldBe true
        }

        test("stream-withdraw: one balance history, no assignment") {
            val amounts = consStream(25L) { consStream(25L) { consStream(10L) { LStream.Empty } } }
            streamWithdraw(100L, amounts).take(4) shouldBe listOf(100L, 75L, 50L, 40L)
        }

        test("stream-withdraw stops once the amounts run out") {
            streamWithdraw(100L, consStream(25L) { LStream.Empty }).take(5) shouldBe listOf(100L, 75L)
        }
    })
