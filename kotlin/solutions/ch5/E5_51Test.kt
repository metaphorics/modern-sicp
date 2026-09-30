// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_51

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_51Test :
    FunSpec({
        test("Exercise 5.51: the C translation of the explicit-control evaluator runs the factorial session") {
            translatedEvaluatorRuns() shouldBe
                listOf(
                    "120",
                    "the C evaluator agrees with the explicit-control run: true",
                )
        }
    })
