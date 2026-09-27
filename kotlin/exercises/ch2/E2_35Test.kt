// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.35

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_35Test :
    FunSpec({
        test("Exercise 2.35: countLeavesAccumulate counts the four leaves of (1 (2 (3 4)))").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_35() shouldBe 4L
        }
    })
