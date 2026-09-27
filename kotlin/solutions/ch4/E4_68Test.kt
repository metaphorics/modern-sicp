// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_68

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_68Test :
    FunSpec({
        test("Exercise 4.68: the reverse queries") {
            reverseQueries() shouldBe
                listOf(
                    "query: (reverse (1 2 3) ?x)",
                    "(reverse (1 2 3) (3 2 1))",
                    "query: (reverse (a b c d) ?x)",
                    "(reverse (a b c d) (d c b a))",
                    "query: (reverse ?x (1 2 3)) -- first answer:",
                    "the unanchored generation exhausts the heap before the first answer survives the append filter",
                )
        }
    })
