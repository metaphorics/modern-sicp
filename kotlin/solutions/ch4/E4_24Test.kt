// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.24

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt

public class E4_24Test :
    FunSpec({
        test("Exercise 4.24: both engines compute the same fib and both medians are positive") {
            val direct = directEvaluatorTiming()
            val analyzed = analyzerTiming()
            direct.value shouldBe VInt(144)
            analyzed.value shouldBe VInt(144)
            (direct.medianNanos > 0.0) shouldBe true
            (analyzed.medianNanos > 0.0) shouldBe true
        }
    })
