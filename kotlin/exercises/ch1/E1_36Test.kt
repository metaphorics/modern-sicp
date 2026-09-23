// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.36

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_36Test :
    FunSpec({
        test("Exercise 1.36: average damping cuts x^x = 1000's step count from 34 to 9").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_36() shouldBe Pair(34, 9)
        }
    })
