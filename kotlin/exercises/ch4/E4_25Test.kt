// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.25

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_25Test :
    FunSpec({
        test("Exercise 4.25: the unless factorial bottoms out under delayed arguments").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lazyFactorialTranscript() shouldBe "120\n"
        }

        test("Exercise 4.25: the strict evaluator raises before unless is called").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            strictArmedUnlessTranscript() shouldBe "DivisionByZero"
        }

        test("Exercise 4.25: the strict recursion never reaches the guard, budgeted honestly").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            strictFactorialTranscript() shouldBe "BudgetExhausted after 200 steps\n"
        }
    })
