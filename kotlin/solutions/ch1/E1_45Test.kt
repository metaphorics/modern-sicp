// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.45

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe
import kotlin.math.pow

public class E1_45Test :
    FunSpec({
        test("the 4th root of 16 and the 8th root of 256 are both 2") {
            val (fourth, eighth) = ex_1_45()
            fourth shouldBe (2.0 plusOrMinus 1e-6)
            eighth shouldBe (2.0 plusOrMinus 1e-6)
        }
        test("nthRoot(x, n) raised to the n-th power recovers x, for n from 2 to 16") {
            for (n in 2L..16L) {
                val root = nthRoot(16.0, n)
                root.pow(n.toDouble()) shouldBe (16.0 plusOrMinus 0.01)
            }
        }
        test("the square root via nthRoot matches the ordinary one damp fixed point search") {
            nthRoot(2.0, 2L) shouldBe (1.4142135623746899 plusOrMinus 1e-9)
        }
    })
