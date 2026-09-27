// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.26

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_26Test :
    FunSpec({
        test("Exercise 2.26: appendList, cons, and vlist print the three predicted forms").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_26() shouldBe listOf("(1 2 3 4 5 6)", "((1 2 3) 4 5 6)", "((1 2 3) (4 5 6))")
        }
    })
