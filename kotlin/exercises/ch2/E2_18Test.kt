// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.18

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_18Test :
    FunSpec({
        test("Exercise 2.18: reverseList of (1 4 9 16 25) is (25 16 9 4 1)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_18() shouldBe "(25 16 9 4 1)"
        }
    })
