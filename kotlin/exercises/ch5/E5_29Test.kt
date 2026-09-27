// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.29

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_29Test :
    FunSpec({
        test("Exercise 5.29: fib's stack table and the depth, recurrence, and closed-form lines").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            fibonacciStackMeasurements() shouldBe emptyList()
        }
    })
