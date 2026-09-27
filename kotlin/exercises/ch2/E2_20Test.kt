// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.20

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_20Test :
    FunSpec({
        test("Exercise 2.20: sameParity(1, 2, 3, 4, 5, 6, 7) keeps the odd ones").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_20() shouldBe listOf(1L, 3L, 5L, 7L)
        }
    })
