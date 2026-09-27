// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.44

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_44Test :
    FunSpec({
        test("Exercise 1.44: smooth(square)(2) and the 5-fold smoothed square, both near 4").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_44() shouldBe Pair(4.000000000066667, 4.000000000333333)
        }
    })
