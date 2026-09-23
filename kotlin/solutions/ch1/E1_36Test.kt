// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.36

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E1_36Test :
    FunSpec({
        test("average damping cuts the step count from 34 to 9") {
            ex_1_36() shouldBe Pair(34, 9)
        }
        test("both traces converge to the same root of x^x = 1000, near 4.5555") {
            val withoutDamping = fixedPointTraced(::xToTheXForTest, 2.0)
            val withDamping = fixedPointTraced(averageDampForTest(::xToTheXForTest), 2.0)
            withoutDamping.last() shouldBe (4.555532270803653 plusOrMinus 0.0001)
            withDamping.last() shouldBe (4.555537551999825 plusOrMinus 0.0001)
        }
    })

private fun xToTheXForTest(x: Double): Double = kotlin.math.ln(1000.0) / kotlin.math.ln(x)

private fun averageDampForTest(f: (Double) -> Double): (Double) -> Double = { x -> (x + f(x)) / 2.0 }
