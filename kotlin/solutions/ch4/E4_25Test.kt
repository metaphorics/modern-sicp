// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.25: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct

public class E425Test :
    FunSpec({
        test("Exercise 4.25: the lazy recursion bottoms out at the guard and answers 120") {
            lazyFactorialTranscript() shouldBe "120\n"
        }

        test("Exercise 4.25: the armed strict call raises before the exceptional arm is reached") {
            val result = Direct.run(ARMED_UNLESS_PROGRAM).fold({ e -> throw AssertionError(e.toString()) }, { it })
            result.output shouldBe ""
            result.error?.category shouldBe "DivisionByZero"
            strictArmedCategory() shouldBe "DivisionByZero"
            strictArmedUnlessTranscript() shouldBe "Error: DivisionByZero\n"
        }

        test("Exercise 4.25: the strict descent never reaches the guard; the entry budget reports it") {
            strictFactorialTranscript() shouldBe "entry budget exhausted\n"
        }
    })
