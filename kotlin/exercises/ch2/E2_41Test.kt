// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.41

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_41Test :
    FunSpec({
        test("Exercise 2.41: tripleSum(5, 10) finds (1 4 5) and (2 3 5)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_41() shouldBe listOf(listOf(1L, 4L, 5L), listOf(2L, 3L, 5L))
        }
    })
