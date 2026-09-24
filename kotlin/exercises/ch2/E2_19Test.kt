// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.19

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_19Test :
    FunSpec({
        test("Exercise 2.19: cc(100, usCoins) counts 292 ways to change 100 cents").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_19() shouldBe 292L
        }
    })
