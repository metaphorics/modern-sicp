// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.58

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_58Test :
    FunSpec({
        test("Exercise 2.58: the precedence-aware derivative of x + 3 * (x + y + 2) with respect to x is 4").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_58() shouldBe "4"
        }
    })
