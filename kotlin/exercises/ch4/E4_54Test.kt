// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.54

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_54Test :
    FunSpec({
        test("Exercise 4.54: the special form prunes the odd choices").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            requireFilteredEvens() shouldBe listOf("2", "4")
        }

        test("Exercise 4.54: a satisfied requirement answers ok").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            requireSatisfied() shouldBe "ok"
        }
    })
