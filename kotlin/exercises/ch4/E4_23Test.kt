// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.23

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_23Test :
    FunSpec({
        test("Exercise 4.23: Alyssa's sequences produce the same values").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            textSequenceTranscript() shouldBe "20\n60\n"
            alyssaSequenceTranscript() shouldBe "20\n60\n"
        }

        test("Exercise 4.23: on a two-statement body Alyssa analyzes less at definition and one more per call").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            textSequenceProfile().twoStatements shouldBe AnalysisProfile(atDefinition = 3, perCall = 3)
            alyssaSequenceProfile().twoStatements shouldBe AnalysisProfile(atDefinition = 2, perCall = 4)
        }

        test("Exercise 4.23: on a one-statement body the two versions do identical work").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            textSequenceProfile().oneStatement shouldBe AnalysisProfile(atDefinition = 2, perCall = 3)
            alyssaSequenceProfile().oneStatement shouldBe AnalysisProfile(atDefinition = 2, perCall = 3)
        }
    })
