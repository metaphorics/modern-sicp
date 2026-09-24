// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.61

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_61Test :
    FunSpec({
        test("Exercise 2.61: adjoinSetOrdered inserts 4 into [1, 3, 6, 10]").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_61() shouldBe listOf(1L, 3L, 4L, 6L, 10L)
        }
    })
