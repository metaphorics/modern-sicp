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
                    "query: (?x next-to ?y in (1 (2 3) 4))",
                    "(1 next-to (2 3) in (1 (2 3) 4))",
                    "((2 3) next-to 4 in (1 (2 3) 4))",
                    "query: (?x next-to 1 in (2 1 3 1))",
                    "(2 next-to 1 in (2 1 3 1))",
                    "(3 next-to 1 in (2 1 3 1))",
                )
        }
    })
