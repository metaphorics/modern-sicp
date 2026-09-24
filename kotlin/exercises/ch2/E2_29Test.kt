// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.29

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_29Test :
    FunSpec({
        test("Exercise 2.29: totalWeight of the sample mobile is 12").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_29() shouldBe 12L
        }
    })
