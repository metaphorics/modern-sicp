// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.28

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_28Test :
    FunSpec({
        test("Exercise 5.28: both factorials' stack tables on the non-tail-recursive evaluator").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            nonTailRecursiveMeasurements() shouldBe emptyList()
        }
    })
