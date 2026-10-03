// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.19: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_19Test :
    FunSpec({
        test("Exercise 4.19: source order reads the outer binding") {
            benRuleTranscript() shouldBe "16\n"
        }

        test("Exercise 4.19: the reservation rejects the fellow read") {
            alyssaRuleTranscript() shouldBe "error\n"
        }

        test("Exercise 4.19: reserve-then-assign lets b see the final a") {
            evaRuleTranscript() shouldBe "20\n"
        }
    })
