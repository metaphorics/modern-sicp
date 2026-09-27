// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.26

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_26Test :
    FunSpec({
        test("the maximum depth is 10 for every n and the pushes fit 35n + 29 on every measured point") {
            iterativeFactorialMeasurements() shouldBe
                listOf(
                    "iterative factorial n=1: total-pushes = 64 maximum-depth = 10",
                    "iterative factorial n=2: total-pushes = 99 maximum-depth = 10",
                    "iterative factorial n=3: total-pushes = 134 maximum-depth = 10",
                    "iterative factorial n=4: total-pushes = 169 maximum-depth = 10",
                    "iterative factorial n=5: total-pushes = 204 maximum-depth = 10",
                    "iterative factorial n=6: total-pushes = 239 maximum-depth = 10",
                    "maximum depth: 10, independent of n = true",
                    "total pushes = 35n + 29, holds on every measured n: true",
                )
        }
    })
