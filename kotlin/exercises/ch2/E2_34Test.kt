// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.34

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_34Test :
    FunSpec({
        test("Exercise 2.34: hornerEval(2, (1 3 0 5 0 1)) is 79").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_34() shouldBe 79L
        }
    })
