// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_30

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_30Test :
    FunSpec({
        test("the caught failures report their categories and a clean factorial still answers 120") {
            errorSignalingRuns() shouldBe
                listOf(
                    "unbound variable rejected before effects: true",
                    "arity mismatch rejected before effects: true",
                    "non-Boolean condition rejected before effects: true",
                    "operation failed: DivisionByZero",
                    "clean factorial: 120",
                )
        }
    })
