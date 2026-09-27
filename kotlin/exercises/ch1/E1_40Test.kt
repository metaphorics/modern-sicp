// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.40

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_40Test :
    FunSpec({
        test("Exercise 1.40: newtonsMethod on cubic(0, 0, -8) from guess 1.0 finds the cube root of 8").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_40() shouldBe 2.000000000036784
        }
    })
