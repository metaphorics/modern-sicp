// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.1.7

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.abs

/** The real-valued square, paired with the integer one of section 1.1.4. */
public fun square(x: Double): Double = x * x

public fun average(
    x: Double,
    y: Double,
): Double = (x + y) / 2.0

public fun improve(
    guess: Double,
    x: Double,
): Double = average(guess, x / guess)

public fun goodEnough(
    guess: Double,
    x: Double,
): Boolean = abs(square(guess) - x) < 0.001

public fun sqrtIter(
    guess: Double,
    x: Double,
): Double = if (goodEnough(guess, x)) guess else sqrtIter(improve(guess, x), x)

public fun sqrt(x: Double): Double = sqrtIter(1.0, x)

public class S1_1_7SqrtTest :
    FunSpec({
        test("the session of the text, computed for real") {
            sqrt(9.0) shouldBe 3.00009155413138
            sqrt(100.0 + 37.0) shouldBe 11.704699917758145
            sqrt(sqrt(2.0) + sqrt(3.0)) shouldBe 1.7739279023207892
            sqrt(1000.0) * sqrt(1000.0) shouldBe 1000.000369924366
        }
    })
