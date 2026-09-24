// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.1.2, the benefits of introducing assignment

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.PI
import kotlin.math.abs
import kotlin.math.sqrt

private const val XORSHIFT_MULTIPLIER: ULong = 0x2545F4914F6CDD1DUL

/**
 * The book's `rand-update`: one step of the seeded xorshift64* generator
 * (Vigna 2016) that `sicp.runtime.Random` also uses, reimplemented here as
 * a section-local pure function. The same word in, the same word out,
 * every time, with no state anywhere but the argument.
 */
public fun randUpdate(x: ULong): ULong {
    var y = x
    y = y xor (y shr 12)
    y = y xor (y shl 25)
    y = y xor (y shr 27)
    return y * XORSHIFT_MULTIPLIER
}

/**
 * The book's `rand`: a closure over one hidden word, updated by
 * `randUpdate` on every call. Two calls to `makeRand` never share a cell.
 */
public fun makeRand(seed: ULong): () -> ULong {
    var x = seed
    return {
        x = randUpdate(x)
        x
    }
}

private tailrec fun gcd(
    a: ULong,
    b: ULong,
): ULong = if (b == 0UL) a else gcd(b, a % b)

/** The book's `cesaro-test`: true when two random words are coprime. */
public fun cesaroTest(rand: () -> ULong): Boolean = gcd(rand(), rand()) == 1UL

/**
 * The book's `monte-carlo`: the fraction of `trials` runs of `experiment`
 * that returned true. `experiment` never learns where its randomness
 * comes from.
 */
public fun monteCarlo(
    trials: Int,
    experiment: () -> Boolean,
): Double {
    tailrec fun iter(
        remaining: Int,
        passed: Int,
    ): Double =
        when {
            remaining == 0 -> passed.toDouble() / trials
            experiment() -> iter(remaining - 1, passed + 1)
            else -> iter(remaining - 1, passed)
        }
    return iter(trials, 0)
}

/** The book's `estimate-pi`: a Cesaro estimate of pi built on `monteCarlo`. */
public fun estimatePi(
    trials: Int,
    rand: () -> ULong,
): Double = sqrt(6.0 / monteCarlo(trials) { cesaroTest(rand) })

/**
 * The book's `random-gcd-test`: the same Cesaro experiment with no local
 * state for the generator. The random words are threaded through the loop
 * by hand, and `x2` must be recycled as the next round's `x`.
 */
public fun randomGcdTest(
    trials: Int,
    initialX: ULong,
): Double {
    tailrec fun iter(
        remaining: Int,
        passed: Int,
        x: ULong,
    ): Double {
        val x1 = randUpdate(x)
        val x2 = randUpdate(x1)
        return when {
            remaining == 0 -> passed.toDouble() / trials
            gcd(x1, x2) == 1UL -> iter(remaining - 1, passed + 1, x2)
            else -> iter(remaining - 1, passed, x2)
        }
    }
    return iter(trials, 0, initialX)
}

/** The book's stateless `estimate-pi`, which must also name the generator's starting word. */
public fun estimatePiStateless(
    trials: Int,
    initialX: ULong,
): Double = sqrt(6.0 / randomGcdTest(trials, initialX))

public class S3_1_2BenefitsOfIntroducingAssignmentTest :
    FunSpec({
        test("estimatePi with a fixed seed lands close to pi") {
            val estimate = estimatePi(100_000, makeRand(1UL))
            (abs(estimate - PI) < 0.05) shouldBe true
        }

        test("randomGcdTest computes the same kind of estimate with no closure") {
            val estimate = estimatePiStateless(100_000, 1UL)
            (abs(estimate - PI) < 0.05) shouldBe true
        }

        test("two rand closures seeded alike produce the same sequence") {
            val r1 = makeRand(7UL)
            val r2 = makeRand(7UL)
            r1() shouldBe r2()
            r1() shouldBe r2()
        }
    })
