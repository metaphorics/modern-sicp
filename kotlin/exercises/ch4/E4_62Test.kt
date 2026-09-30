// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.62

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_62Test :
    FunSpec({
        test("Exercise 4.62: the last-pair queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lastPairQueries() shouldBe
                listOf(
                    "?x = [3]",
                    "?x = [3]",
                    "?x = 3",
                    "backward answer 0: x has 1 item(s), ends 3",
                    "backward answer 1: x has 2 item(s), ends 3",
                    "backward answer 2: x has 3 item(s), ends 3",
                )
        }
    })
