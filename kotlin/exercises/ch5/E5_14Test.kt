// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.14

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_14Test :
    FunSpec({
        test("Exercise 5.14: factorial stack statistics as a function of n").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
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
    })
