// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_50

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_50Test :
    FunSpec({
        test("Exercise 5.50: the compiled guest evaluator interprets its target program in all three executions") {
            compiledMetacircularRuns() shouldBe
                listOf(
                    "direct answer: 120",
                    "explicit-control answer: 120",
                    "compiled machine answer: 120",
                    "the three executions agree: true",
                )
        }
    })
