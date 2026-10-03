// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.28: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.LazyModule

public class E428Test :
    FunSpec({
        test("Exercise 4.28: the forced operator dispatches on the function value") {
            forcedOperatorTranscript() shouldBe "5\n"
        }

        test("Exercise 4.28: applying the unforced thunk is rejected before any effect") {
            val rejection =
                LazyModule.run(UNFORCED_OPERATOR_PROGRAM).fold(
                    { e -> e },
                    { throw AssertionError("admission must reject the unforced operator call") },
                )
            rejection.category.isNotBlank() shouldBe true
            unforcedOperatorTranscript() shouldBe "Error: ${rejection.category}\n"
        }
    })
