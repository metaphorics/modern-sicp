// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.25

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_25Test :
    FunSpec({
        test("Exercise 4.25: the unless factorial bottoms out under delayed arguments") {
            lazyFactorialTranscript() shouldBe "120\n"
        }

        test("Exercise 4.25: the strict evaluator raises before unless is called") {
            strictArmedUnlessTranscript() shouldBe "Error: division by zero\n"
        }

        test("Exercise 4.25: the strict recursion never reaches the guard, budgeted honestly") {
            strictFactorialTranscript() shouldBe "Error: machine fault: step budget exhausted after 200 steps\n"
        }
    })
