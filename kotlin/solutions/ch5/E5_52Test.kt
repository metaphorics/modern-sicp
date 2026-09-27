// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.52

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_52Test :
    FunSpec({
        test("the C backend compiles the evaluator and answers 120") {
            compiledInterpreterRuns() shouldBe listOf("120")
        }
    })
