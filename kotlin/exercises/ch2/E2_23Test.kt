// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.23

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_23Test :
    FunSpec({
        test("Exercise 2.23 visits the recorded values from left to right").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_23() shouldBe listOf(57L, 321L, 88L)
        }
    })
