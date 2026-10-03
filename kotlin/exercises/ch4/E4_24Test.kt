// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.24

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_24Test :
    FunSpec({
        test("Exercise 4.24: both engines compute the same fib and both medians are positive").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val direct = directEvaluatorTiming()
            val analyzed = analyzerTiming()
            direct.value shouldBe 144L
            analyzed.value shouldBe 144L
            (direct.medianNanos > 0.0) shouldBe true
            (analyzed.medianNanos > 0.0) shouldBe true
        }
    })
