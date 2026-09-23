// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.5

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_05Test :
    FunSpec({
        test("Exercise 2.5: consPow(3, 4) recovers as the pair (3, 4)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_05() shouldBe (3L to 4L)
        }
    })
