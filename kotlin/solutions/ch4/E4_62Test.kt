// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_62

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_62Test :
    FunSpec({
        test("Exercise 4.62: the last-pair queries") {
            lastPairQueries() shouldBe
                listOf(
                    "query: (last-pair (3) ?x)",
                    "(last-pair (3) (3))",
                    "query: (last-pair (1 2 3) ?x)",
                    "(last-pair (1 2 3) (3))",
                    "query: (last-pair (2 ?x) (3))",
                    "(last-pair (2 3) (3))",
                    "query: (last-pair ?x (3)) -- divergent; first three answers:",
                    "(last-pair (3) (3))",
                    "(last-pair (?u-20 3) (3))",
                    "(last-pair (?u-20 ?u-22 3) (3))",
                )
        }
    })
