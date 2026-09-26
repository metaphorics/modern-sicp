// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.25

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_25Test :
    FunSpec({
        test("Exercise 5.25: the lazy evaluator's factorial, laziness, and memoization sessions").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            normalOrderRuns() shouldBe emptyList()
        }
    })
