// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.1.8

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.abs
import kotlin.math.exp
import kotlin.math.ln

// The `square` of this file is the real-valued one of section 1.1.7; the
// book redefines the name, and the alternative implementation below takes
// a name of its own so the two can coexist in one package.

public fun double(x: Double): Double = x + x

/** The same contract through logarithms; the value carries rounding error. */
public fun squareByLog(x: Double): Double = exp(double(ln(x)))

/**
 * Block structure: the helpers are local functions and the radicand `x` is
 * a free variable of each. Named `sqrtLocal`, not `sqrt` as in the text,
 * because the flat spelling of section 1.1.7 already owns the name in this
 * package; the book simply redefines the name.
 */
private fun sqrtLocal(x: Double): Double {
    fun goodEnough(guess: Double): Boolean = abs(square(guess) - x) < 0.001

    fun improve(guess: Double): Double = average(guess, x / guess)

    fun sqrtIter(guess: Double): Double = if (goodEnough(guess)) guess else sqrtIter(improve(guess))
    return sqrtIter(1.0)
}

public class S1_1_8BlackBoxTest :
    FunSpec({
        test("two implementations of one contract are interchangeable") {
            square(3.0) shouldBe 9.0
            squareByLog(3.0) shouldBe 9.000000000000002
            squareByLog(1.5) shouldBe 2.25
        }
        test("the local names of a definition stay invisible outside it") {
            sqrtLocal(9.0) shouldBe 3.00009155413138
        }
    })
