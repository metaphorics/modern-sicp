// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.54

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_54Test :
    FunSpec({
        test("Exercise 2.54: myEqual matches the book's two worked examples").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_54() shouldBe listOf(true, false)
        }
    })
