// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_57

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_57Test :
    FunSpec({
        test("Exercise 4.57: the can-replace queries") {
            canReplaceQueries() shouldBe
                listOf(
                    "?x = [Bitdiddle, Ben]",
                    "?x = [Hacker, Alyssa, P]",
                    "?person-1 = [Fect, Cy, D]",
                    "?person-2 = [Hacker, Alyssa, P]",
                    "?person-1 = [Aull, DeWitt]",
                    "?person-2 = [Warbucks, Oliver]",
                )
        }
    })
