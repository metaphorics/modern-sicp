// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.26

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_26Test :
    FunSpec({
        test("Exercise 5.26: the iterative factorial's stack table and the two answers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            iterativeFactorialMeasurements().takeLast(2) shouldBe
                listOf(
                    "maximum depth independent of n: true",
                    "total pushes linear in n: true",
                )
        }
    })
