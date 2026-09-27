// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.28

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_28Test :
    FunSpec({
        test("Exercise 2.28: fringe of ((1 2) (3 4)) is the four leaves").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_28() shouldBe listOf(1L, 2L, 3L, 4L)
        }
    })
