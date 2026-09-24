// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.2.4, internal definitions

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.abs

/**
 * The book's `sqrt` with internal definitions. Each call of `sqrt` opens
 * a frame binding `x`; the three local functions are bound in that frame,
 * each seeing `x` as a free variable. Kotlin binds names in declaration
 * order, so a local function sees only the names declared above it, and
 * `sqrtIter`, which calls the other two, is declared last. The `tailrec`
 * mark turns the one self-recursive call into a loop.
 */
public fun sqrt(x: Double): Double {
    fun goodEnough(guess: Double): Boolean = abs(guess * guess - x) < 0.001

    fun improve(guess: Double): Double = (guess + x / guess) / 2.0

    tailrec fun sqrtIter(guess: Double): Double = if (goodEnough(guess)) guess else sqrtIter(improve(guess))

    return sqrtIter(1.0)
}

public class S3_2_4InternalDefinitionsTest :
    FunSpec({
        test("the book's walkthrough: sqrt(2) lands within 0.001 of the root") {
            (abs(sqrt(2.0) - 1.4142135623730951) < 0.001) shouldBe true
        }

        test("sqrt(9) lands within 0.001 of 3") {
            (abs(sqrt(9.0) - 3.0) < 0.001) shouldBe true
        }

        test("each call of sqrt gets its own x: the answer follows the argument") {
            (abs(sqrt(16.0) - 4.0) < 0.001) shouldBe true
        }
    })
