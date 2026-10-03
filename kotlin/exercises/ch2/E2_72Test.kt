// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.72

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_72Test :
    FunSpec({
        test("Exercise 2.72: for n = 5 the most frequent symbol costs 4 steps and the least frequent 10").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_72(5) shouldBe Pair(4, 10)
        }
    })
