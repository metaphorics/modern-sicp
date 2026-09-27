// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.13

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.double
import io.kotest.property.arbitrary.filter
import io.kotest.property.checkAll

public class E2_13Test :
    FunSpec({
        test("the exact percentage tolerance is close to p1 + p2 for small tolerances") {
            val x = makeCenterPercent(100.0, 0.01)
            val y = makeCenterPercent(50.0, 0.02)
            percent(mulInterval(x, y)) shouldBe (approxProductPercent(0.01, 0.02) plusOrMinus 1e-4)
        }
        test("the approximation error shrinks as the tolerances shrink, for many generated small tolerances") {
            val smallTolerance = Arb.double(0.0001..0.02).filter { it in 0.0001..0.02 }
            checkAll(smallTolerance, smallTolerance) { p1, p2 ->
                val x = makeCenterPercent(100.0, p1)
                val y = makeCenterPercent(50.0, p2)
                val exact = percent(mulInterval(x, y))
                val approx = approxProductPercent(p1, p2)
                val error = kotlin.math.abs(exact - approx)
                error shouldBe (0.0 plusOrMinus (p1 * p2 + 1e-9))
            }
        }
        test("ex_2_13 pairs the exact tolerance with the approximation, 0.03") {
            val (exact, approx) = ex_2_13()
            approx shouldBe 0.03
            exact shouldBe (approx plusOrMinus 1e-4)
        }
    })
