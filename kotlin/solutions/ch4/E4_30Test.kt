// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.30: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E430Test :
    FunSpec({
        test("Exercise 4.30: the for-each session streams the same under both rules") {
            forEachTextRuleTranscript() shouldBe "\n57\n321\n88done\n"
            forEachCyRuleTranscript() shouldBe "\n57\n321\n88done\n"
        }

        test("Exercise 4.30: under the text's rule the delayed change never runs") {
            p1P2TextRuleTranscript() shouldBe "2\n1\n"
        }

        test("Exercise 4.30: under Cy's rule sequencing forces the delayed change") {
            p1P2CyRuleTranscript() shouldBe "2\n2\n"
        }
    })
