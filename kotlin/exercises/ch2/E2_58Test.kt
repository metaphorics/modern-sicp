// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.58

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_58Test :
    FunSpec({
        test("Exercise 2.58 returns the simplified derivative expression tree").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_58() shouldBe Expr.Num(4)
        }
    })
