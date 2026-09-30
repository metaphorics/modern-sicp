// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20a

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_20aTest :
    FunSpec({
        test("Exercise 4.20a: the shadowing initializer reads the inner f before its assignment").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            shadowedPrematureReadTranscript() shouldBe "UnassignedRead"
        }

        test("Exercise 4.20a: with the outer binding in scope the same program recurses instead").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            outerScopeRecursionTranscript() shouldBe "1\n"
        }
    })
