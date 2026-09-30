// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.23

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_23Test :
    FunSpec({
        test("Exercise 5.23: the `when` and `let` sessions through the extended evaluator").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            derivedExpressionRuns() shouldBe
                listOf(
                    "zero",
                    "one",
                    "many",
                    "missing",
                    "six",
                    "transformed syntax matches the core program: true",
                    "direct and explicit-control runs agree: true",
                )
        }
    })
