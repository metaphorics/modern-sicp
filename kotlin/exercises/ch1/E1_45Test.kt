// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.45

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E1_45Test :
    FunSpec({
        test("Exercise 1.45: the 4th root of 16 and the 8th root of 256 are both 2").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val (fourth, eighth) = ex_1_45()
            fourth shouldBe (2.0 plusOrMinus 1e-6)
            eighth shouldBe (2.0 plusOrMinus 1e-6)
        }
    })
