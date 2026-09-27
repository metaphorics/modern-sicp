// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.23

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_23Test :
    FunSpec({
        test("Exercise 4.23: Alyssa's sequences produce the same values") {
            textSequenceTranscript() shouldBe "20\n60\n"
            alyssaSequenceTranscript() shouldBe "20\n60\n"
        }

        test("Exercise 4.23: on a two-expression body Alyssa analyzes less at definition and one more per call") {
            textSequenceProfile().twoExpressions shouldBe AnalysisProfile(atDefinition = 3, perCall = 3)
            alyssaSequenceProfile().twoExpressions shouldBe AnalysisProfile(atDefinition = 2, perCall = 4)
        }

        test("Exercise 4.23: on a one-expression body the two versions do identical work") {
            textSequenceProfile().oneExpression shouldBe AnalysisProfile(atDefinition = 2, perCall = 3)
            alyssaSequenceProfile().oneExpression shouldBe AnalysisProfile(atDefinition = 2, perCall = 3)
        }
    })
