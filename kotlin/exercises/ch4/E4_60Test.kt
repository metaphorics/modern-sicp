// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.60

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_60Test :
    FunSpec({
        test("Exercise 4.60: the lives-near queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            livesNearQueries() shouldBe
                listOf(
                    "?person = [Fect, Cy, D]",
                    "?person-1 = [Bitdiddle, Ben]",
                    "?person-2 = [Reasoner, Louis]",
                    "?person-1 = [Bitdiddle, Ben]",
                    "?person-2 = [Aull, DeWitt]",
                    "?person-1 = [Hacker, Alyssa, P]",
                    "?person-2 = [Fect, Cy, D]",
                    "?person-1 = [Fect, Cy, D]",
                    "?person-2 = [Hacker, Alyssa, P]",
                    "?person-1 = [Reasoner, Louis]",
                    "?person-2 = [Bitdiddle, Ben]",
                    "?person-1 = [Reasoner, Louis]",
                    "?person-2 = [Aull, DeWitt]",
                    "?person-1 = [Aull, DeWitt]",
                    "?person-2 = [Bitdiddle, Ben]",
                    "?person-1 = [Aull, DeWitt]",
                    "?person-2 = [Reasoner, Louis]",
                    "?person-1 = [Fect, Cy, D]",
                    "?person-2 = [Hacker, Alyssa, P]",
                    "?person-1 = [Reasoner, Louis]",
                    "?person-2 = [Bitdiddle, Ben]",
                    "?person-1 = [Aull, DeWitt]",
                    "?person-2 = [Bitdiddle, Ben]",
                    "?person-1 = [Aull, DeWitt]",
                    "?person-2 = [Reasoner, Louis]",
                )
        }
    })
