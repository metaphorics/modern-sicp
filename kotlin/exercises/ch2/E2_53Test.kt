// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.53 (replaced)

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_53Test :
    FunSpec({
        test("Exercise 2.53: the seven predicted toString() results match the hand-worked answers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_53() shouldBe
                listOf(
                    "(a b c)",
                    "((george))",
                    "((y1 y2))",
                    "(y1 y2)",
                    "#f",
                    "#f",
                    "(red shoes blue socks)",
                )
        }
    })
