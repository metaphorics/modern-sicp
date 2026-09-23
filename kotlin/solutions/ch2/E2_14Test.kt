// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.14

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E2_14Test :
    FunSpec({
        test("a / a reports roughly double a's own tolerance instead of zero") {
            val a = makeCenterPercent(100.0, 0.05)
            percent(divInterval(a, a)) shouldBe (0.0997506234413964 plusOrMinus 1e-9)
        }
        test("a / b, independent intervals, reports roughly the sum of the tolerances") {
            val a = makeCenterPercent(100.0, 0.05)
            val b = makeCenterPercent(200.0, 0.1)
            percent(divInterval(a, b)) shouldBe (0.14925373134328362 plusOrMinus 1e-9)
        }
        test("ex_2_14 pairs both tolerances") {
            val (aOverA, aOverB) = ex_2_14()
            aOverA shouldBe (0.0997506234413964 plusOrMinus 1e-9)
            aOverB shouldBe (0.14925373134328362 plusOrMinus 1e-9)
        }
    })
