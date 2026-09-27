// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.1.4

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe

/**
 * An interval: a lower bound and an upper bound, representing every real
 * number an inexact quantity's measurement error could put it at. [Alyssa's
 * postulate][makeInterval] is left as unremarkable as the book leaves it;
 * exercise 2.7 is the one that has to justify it.
 */
public data class Interval(
    val lowerBound: Double,
    val upperBound: Double,
)

public fun makeInterval(
    a: Double,
    b: Double,
): Interval = Interval(a, b)

/** The minimum value the sum could take is the sum of the minimums, and likewise for the maximum. */
public fun addInterval(
    x: Interval,
    y: Interval,
): Interval = makeInterval(x.lowerBound + y.lowerBound, x.upperBound + y.upperBound)

/** [minOf] and [maxOf] find the minimum or maximum of any number of arguments; that is all `mul-interval` needs. */
public fun mulInterval(
    x: Interval,
    y: Interval,
): Interval {
    val p1 = x.lowerBound * y.lowerBound
    val p2 = x.lowerBound * y.upperBound
    val p3 = x.upperBound * y.lowerBound
    val p4 = x.upperBound * y.upperBound
    return makeInterval(minOf(p1, p2, p3, p4), maxOf(p1, p2, p3, p4))
}

/** Multiplies the first interval by the reciprocal of the second, bounds swapped in that order. */
public fun divInterval(
    x: Interval,
    y: Interval,
): Interval = mulInterval(x, makeInterval(1.0 / y.upperBound, 1.0 / y.lowerBound))

/** Alyssa's fix for the user who thinks in a center value and an additive tolerance, `3.5 +/- 0.15` rather than `[3.35, 3.65]`. */
public fun makeCenterWidth(
    c: Double,
    w: Double,
): Interval = makeInterval(c - w, c + w)

public fun center(i: Interval): Double = (i.lowerBound + i.upperBound) / 2.0

public fun width(i: Interval): Double = (i.upperBound - i.lowerBound) / 2.0

/** Ben Bitdiddle's colleague Lem E. Tweakit's first parallel-resistance formula: `(R1 R2) / (R1 + R2)`. */
public fun par1(
    r1: Interval,
    r2: Interval,
): Interval = divInterval(mulInterval(r1, r2), addInterval(r1, r2))

/** The algebraically equivalent second formula: `1 / (1/R1 + 1/R2)`. */
public fun par2(
    r1: Interval,
    r2: Interval,
): Interval {
    val one = makeInterval(1.0, 1.0)
    return divInterval(one, addInterval(divInterval(one, r1), divInterval(one, r2)))
}

public class S2_1_4IntervalArithmeticTest :
    FunSpec({
        test("addInterval sums both bounds") {
            addInterval(makeInterval(4.0, 8.0), makeInterval(1.0, 2.0)) shouldBe makeInterval(5.0, 10.0)
        }
        test("mulInterval takes the min and max of the four boundary products") {
            mulInterval(makeInterval(4.0, 8.0), makeInterval(1.0, 2.0)) shouldBe makeInterval(4.0, 16.0)
        }
        test("divInterval multiplies by the reciprocal interval") {
            divInterval(makeInterval(4.0, 8.0), makeInterval(1.0, 2.0)) shouldBe makeInterval(2.0, 8.0)
        }
        test("makeCenterWidth(3.5, 0.15) is the interval [3.35, 3.65] of the tolerance example") {
            val i = makeCenterWidth(3.5, 0.15)
            i shouldBe makeInterval(3.35, 3.65)
            center(i) shouldBe 3.5
            width(i) shouldBe (0.15 plusOrMinus 1e-9)
        }
        test("Lem is right: par1 and par2 disagree on the same two resistors") {
            val r1 = makeInterval(6.12, 7.48)
            val r2 = makeInterval(4.465, 4.935)
            par1(r1, r2) shouldNotBe par2(r1, r2)
        }
    })
