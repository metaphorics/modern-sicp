// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_27

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_27Test :
    FunSpec({
        test("the depth and the pushes are both linear in n, on every measured point") {
            recursiveFactorialMeasurements().takeLast(2) shouldBe
                listOf(
                    "maximum depth linear in n: true",
                    "total pushes linear in n: true",
                )
        }
    })
