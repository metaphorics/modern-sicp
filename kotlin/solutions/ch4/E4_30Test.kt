// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.30

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_30Test :
    FunSpec({
        test("Exercise 4.30: for-each prints under the text's eval-sequence") {
            forEachTextRuleTranscript() shouldBe "\n57\n321\n88done\n"
        }

        test("Exercise 4.30: for-each prints the same under Cy's rule") {
            forEachCyRuleTranscript() shouldBe "\n57\n321\n88done\n"
        }

        test("Exercise 4.30: p1 mutates, p2's set! stays delayed under the text's rule") {
            p1P2TextRuleTranscript() shouldBe "(1 2)\n1\n"
        }

        test("Exercise 4.30: Cy's rule forces p2's delayed set!") {
            p1P2CyRuleTranscript() shouldBe "(1 2)\n(1 2)\n"
        }
    })
