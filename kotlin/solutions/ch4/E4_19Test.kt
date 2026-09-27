// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.19

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_19Test :
    FunSpec({
        test("Exercise 4.19: Ben's sequential rule initializes b against the outer a and answers 16") {
            benRuleTranscript() shouldBe "16\n"
        }

        test("Exercise 4.19: Alyssa's scan-out rejects the read before assignment") {
            alyssaRuleTranscript() shouldBe "Error: type mismatch: a is read before it is assigned\n"
        }

        test("Exercise 4.19: Eva's simultaneous rule initializes b against the final a and answers 20") {
            evaRuleTranscript() shouldBe "20\n"
        }
    })
