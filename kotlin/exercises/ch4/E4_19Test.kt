// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.19

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_19Test :
    FunSpec({
        test("Exercise 4.19: Ben's sequential rule initializes b against the outer a and answers 16").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            benRuleTranscript() shouldBe "16\n"
        }

        test("Exercise 4.19: Alyssa's scan-out rejects the read before assignment").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            alyssaRuleTranscript() shouldBe "UnassignedRead"
        }

        test("Exercise 4.19: Eva's simultaneous rule initializes b against the final a and answers 20").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            evaRuleTranscript() shouldBe "20\n"
        }
    })
