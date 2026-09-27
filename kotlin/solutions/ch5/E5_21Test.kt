// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.21

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_21Test :
    FunSpec({
        test("Exercise 5.21: both machines answer with the oracle, and the recursive machine pays more stack") {
            countLeavesRuns() shouldBe
                listOf(
                    "(1 2 (3 (4 5))): recursive n5, iterative n5, oracle 5; " +
                        "recursive stack (total-pushes = 21 maximum-depth = 14), " +
                        "iterative stack (total-pushes = 14 maximum-depth = 10)",
                    "((7)): recursive n1, iterative n1, oracle 1; " +
                        "recursive stack (total-pushes = 6 maximum-depth = 4), " +
                        "iterative stack (total-pushes = 4 maximum-depth = 4)",
                    "(): recursive n0, iterative n0, oracle 0; " +
                        "recursive stack (total-pushes = 0 maximum-depth = 0), " +
                        "iterative stack (total-pushes = 0 maximum-depth = 0)",
                )
        }
    })
