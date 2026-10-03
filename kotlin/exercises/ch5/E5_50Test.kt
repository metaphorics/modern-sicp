// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.50

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_50Test :
    FunSpec({
        test("Exercise 5.50: the compiled interpreter answers 120 and the levels are timed").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            compiledMetacircularRuns() shouldBe
                listOf(
                    "direct answer: 120.0",
                    "explicit-control answer: 120.0",
                    "compiled machine answer: 120.0",
                    "the three executions agree: true",
                )
        }
    })
