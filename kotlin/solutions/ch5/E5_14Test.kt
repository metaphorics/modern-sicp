// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_14

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_14Test :
    FunSpec({
        test("Exercise 5.14: the factorial machine's stack statistics are linear in n") {
            factorialStackStatistics() shouldBe
                listOf(
                    "n = 1: (total-pushes = 0 maximum-depth = 0)",
                    "n = 2: (total-pushes = 2 maximum-depth = 2)",
                    "n = 3: (total-pushes = 4 maximum-depth = 4)",
                    "n = 4: (total-pushes = 6 maximum-depth = 6)",
                    "n = 5: (total-pushes = 8 maximum-depth = 8)",
                    "n = 6: (total-pushes = 10 maximum-depth = 10)",
                    "the measured machine for n = 5 prints: (total-pushes = 8 maximum-depth = 8)",
                )
        }
        test("Exercise 5.14: the machine's answer equals the host factorial") {
            factorialMachineFactorial(5) shouldBe 120L
        }
    })
