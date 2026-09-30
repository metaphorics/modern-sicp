// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.16

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_16Test :
    FunSpec({
        test("Exercise 4.16: mutually recursive internal definitions work under the scan").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            mutualRecursionTranscript() shouldBe "true\n"
        }

        test("Exercise 4.16: a premature read raises the typed fault").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            prematureReadCategory() shouldBe "UnassignedRead"
        }

        test("Exercise 4.16: the unscanned kernel answers its own fault convention").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            basePrematureCategory() shouldBe "null"
        }
    })
