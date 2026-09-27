// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.47

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_47Test :
    FunSpec({
        test("compiled caller invokes a procedure defined later by the evaluator") {
            compoundCallRuns() shouldBe listOf("ok", "ok", "42")
        }
    })
