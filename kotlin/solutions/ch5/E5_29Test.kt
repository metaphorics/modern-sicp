// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.29

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_29Test :
    FunSpec({
        test("the depth grows by 5 per n, the pushes obey the recurrence with k = 40 and the closed form") {
            fibonacciStackMeasurements() shouldBe
                listOf(
                    "fib n=2: total-pushes = 72 maximum-depth = 13",
                    "fib n=3: total-pushes = 128 maximum-depth = 18",
                    "fib n=4: total-pushes = 240 maximum-depth = 23",
                    "fib n=5: total-pushes = 408 maximum-depth = 28",
                    "fib n=6: total-pushes = 688 maximum-depth = 33",
                    "fib n=7: total-pushes = 1136 maximum-depth = 38",
                    "fib n=8: total-pushes = 1864 maximum-depth = 43",
                    "fib n=9: total-pushes = 3040 maximum-depth = 48",
                    "maximum depth = 5n + 3 (every step 5), linear: true",
                    "S(n) = S(n-1) + S(n-2) + 40, k constant: true",
                    "S(n) = 56 * Fib(n+1) - 40, holds on every measured n: true",
                )
        }
    })
