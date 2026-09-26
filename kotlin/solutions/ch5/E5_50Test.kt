// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.50

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_50Test :
    FunSpec({
        test("the compiled object-language evaluator evaluates factorial") {
            compiledMetacircularRuns() shouldBe listOf("answers: ok, 120, (tick tick tick), 120")
        }
    })
