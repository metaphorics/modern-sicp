// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.36

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_36Test :
    FunSpec({
        test("Exercise 2.36: accumulateN sums the book's columns to (22 26 30)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_36() shouldBe listOf(22L, 26L, 30L)
        }
    })
