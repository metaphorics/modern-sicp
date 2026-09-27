// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.37

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_37Test :
    FunSpec({
        test("Exercise 2.37: matrixStarVector with (1 1 1 1) gives the row sums (10 21 30)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_37() shouldBe listOf(10L, 21L, 30L)
        }
    })
