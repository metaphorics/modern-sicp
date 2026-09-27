// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.42

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_42Test :
    FunSpec({
        test("Exercise 2.42: queens(8) finds all 92 solutions").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_42() shouldBe 92
        }
    })
