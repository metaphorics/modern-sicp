// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.62

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_62Test :
    FunSpec({
        test("Exercise 2.62: unionSetOrdered unions [1, 3, 6, 10] and [3, 6, 9]").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_62() shouldBe listOf(1L, 3L, 6L, 9L, 10L)
        }
    })
