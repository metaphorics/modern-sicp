// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_61

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_61Test :
    FunSpec({
        test("Exercise 4.61: the next-to queries") {
            nextToQueries() shouldBe
                listOf(
                    "?x = 1",
                    "?y = [2, 3]",
                    "?x = [2, 3]",
                    "?y = 4",
                    "?x = 2",
                    "?x = 3",
                )
        }
    })
