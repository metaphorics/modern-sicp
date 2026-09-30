// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_29

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_29Test :
    FunSpec({
        test("the depth grows linearly and the pushes obey the recurrence and its closed form") {
            fibonacciStackMeasurements().takeLast(3) shouldBe
                listOf(
                    "maximum depth linear in n: true",
                    "pushes recurrence with constant k: true",
                    "pushes closed form over Fib: true",
                )
        }
    })
