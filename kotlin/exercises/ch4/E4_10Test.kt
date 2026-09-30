// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.10

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_10Test :
    FunSpec({
        test("Exercise 4.10: the transformed demo computes through the rewrite").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            defunTranscript() shouldBe "343\n512\n"
        }

        test("Exercise 4.10: without the rewrite the program never admits").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            defunPlainRejection() shouldBe "UndeclaredName"
        }

        test("Exercise 4.10: the rewrite is structural and recursive").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            nestedFunTranscript() shouldBe "49\n"
        }
    })
