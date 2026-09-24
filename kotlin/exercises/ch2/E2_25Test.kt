// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.25

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_25Test :
    FunSpec({
        test("Exercise 2.25: three accessor chains pick 7 from each list").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_25() shouldBe listOf(7L, 7L, 7L)
        }
    })
