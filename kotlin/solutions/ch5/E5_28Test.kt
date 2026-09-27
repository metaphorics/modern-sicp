// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.28

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_28Test :
    FunSpec({
        test("with the naive sequence evaluation both factorials demand space that grows with n") {
            nonTailRecursiveMeasurements() shouldBe
                listOf(
                    "non-tail iterative factorial n=1: total-pushes = 70 maximum-depth = 17",
                    "non-tail iterative factorial n=2: total-pushes = 107 maximum-depth = 20",
                    "non-tail iterative factorial n=3: total-pushes = 144 maximum-depth = 23",
                    "non-tail iterative factorial n=4: total-pushes = 181 maximum-depth = 26",
                    "non-tail iterative factorial n=5: total-pushes = 218 maximum-depth = 29",
                    "non-tail recursive factorial n=1: total-pushes = 18 maximum-depth = 11",
                    "non-tail recursive factorial n=2: total-pushes = 52 maximum-depth = 19",
                    "non-tail recursive factorial n=3: total-pushes = 86 maximum-depth = 27",
                    "non-tail recursive factorial n=4: total-pushes = 120 maximum-depth = 35",
                    "non-tail recursive factorial n=5: total-pushes = 154 maximum-depth = 43",
                    "iterative maximum depth now grows with n: true",
                )
        }
    })
