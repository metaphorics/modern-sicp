// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.44

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E1_44Test :
    FunSpec({
        test("smooth(square)(2) and the 5-fold smoothed square both land close to the exact 4") {
            ex_1_44() shouldBe Pair(4.000000000066667, 4.000000000333333)
        }
        test("nFoldSmooth(f, 1) is the same function as smooth(f)") {
            nFoldSmooth(::squareForTest, 1L)(3.0) shouldBe smooth(::squareForTest)(3.0)
        }
        test("smoothing a constant function leaves it unchanged") {
            nFoldSmooth({ 7.0 }, 4L)(100.0) shouldBe (7.0 plusOrMinus 1e-9)
        }
    })

private fun squareForTest(x: Double): Double = x * x
