// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.51

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_51Test :
    FunSpec({
        test("the C evaluator builds and answers the factorial session") {
            translatedEvaluatorRuns() shouldBe listOf("ok", "120")
        }
    })
