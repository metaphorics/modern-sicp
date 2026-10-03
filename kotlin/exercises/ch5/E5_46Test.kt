// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.46

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_46Test :
    FunSpec({
        test("Exercise 5.46: the measured fib ratios are tabulated").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            fibStackRatioTable().takeLast(2) shouldBe
                listOf(
                    "the compiled machine uses less stack than the evaluator: true",
                    "the special-purpose machine uses less stack than the evaluator: true",
                )
        }
    })
