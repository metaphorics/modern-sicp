// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.27

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_27Test :
    FunSpec({
        test("the depth is 5n + 3 and the pushes are 32n - 16, the n=5 row the book's own session") {
            recursiveFactorialMeasurements() shouldBe
                listOf(
                    "recursive factorial n=1: total-pushes = 16 maximum-depth = 8",
                    "recursive factorial n=2: total-pushes = 48 maximum-depth = 13",
                    "recursive factorial n=3: total-pushes = 80 maximum-depth = 18",
                    "recursive factorial n=4: total-pushes = 112 maximum-depth = 23",
                    "recursive factorial n=5: total-pushes = 144 maximum-depth = 28",
                    "recursive factorial n=6: total-pushes = 176 maximum-depth = 33",
                    "maximum depth = 5n + 3, holds on every measured n: true",
                    "total pushes = 32n - 16, holds on every measured n: true",
                )
        }
    })
