// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.19

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_19Test :
    FunSpec({
        test("Exercise 5.19: a breakpoint session on the gcd machine").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            breakpointSession() shouldBe
                listOf(
                    "break at test-b: a = 40, b = 6",
                    "break at test-b: a = 4, b = 2",
                    "finished: gcd(206, 40) = 2",
                    "cancel and restart: gcd(206, 40) = 2",
                )
        }
    })
