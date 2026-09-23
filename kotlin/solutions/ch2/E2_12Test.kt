// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.12

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E2_12Test :
    FunSpec({
        test("makeCenterPercent(50, 0.1) is the interval [45, 55]") {
            makeCenterPercent(50.0, 0.1) shouldBe Interval(45.0, 55.0)
        }
        test("percent recovers the fraction makeCenterPercent was given") {
            percent(makeCenterPercent(50.0, 0.1)) shouldBe (0.1 plusOrMinus 1e-12)
        }
        test("ex_2_12 matches makeCenterPercent(50, 0.1)") {
            ex_2_12() shouldBe Interval(45.0, 55.0)
        }
    })
