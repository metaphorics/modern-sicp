// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.22

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_22Test :
    FunSpec({
        test("Exercise 2.22: Louis's iterative squareList answers (16 9 4 1), in reverse order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_22() shouldBe "(16 9 4 1)"
        }
    })
