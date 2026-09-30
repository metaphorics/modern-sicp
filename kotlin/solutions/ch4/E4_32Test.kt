// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.32: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct

public class E432Test :
    FunSpec({
        test("Exercise 4.32: the armed tail is skipped until demanded") {
            lazyPairSlotsTranscript() shouldBe "7\nError: DivisionByZero\n"
        }

        test("Exercise 4.32: the strict constructor forces its slot at construction") {
            eagerConstructorTranscript() shouldBe "Error: DivisionByZero\n"
        }

        test("Exercise 4.32: delayed construction closes the self-reference in one step") {
            onesOneStepTranscript() shouldBe "1\n"
        }

        test("Exercise 4.32: ten demanded heads of the infinite list") {
            onesHeadsTranscript() shouldBe "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]\n"
        }

        test("Exercise 4.32: the strict self-reference is rejected before any effect") {
            val rejection =
                Direct.run(STRICT_ONES_PROGRAM).fold(
                    { e -> e },
                    { throw AssertionError("the strict self-reference must be rejected at admission") },
                )
            rejection.category.isNotBlank() shouldBe true
            strictOnesTranscript() shouldBe "Error: ${rejection.category}\n"
        }
    })
