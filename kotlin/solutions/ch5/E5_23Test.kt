// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_23

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_23Test :
    FunSpec({
        test("a guard when dispatches through the if-chain transform and both engines answer alike") {
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
