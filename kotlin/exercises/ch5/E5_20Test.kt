// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.20

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_20Test :
    FunSpec({
        test("Exercise 5.20: the memory-vector drawing of the three pair allocations").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            memoryVectorDrawing() shouldBe
                listOf(
                    "index    0   1   2   3   4   5   6   7",
                    "the-cars e0  n1  p1  p1  e0  e0  e0  e0",
                    "the-cdrs e0  n2  e0  p2  e0  e0  e0  e0",
                    "x = p1",
                    "y = p3",
                    "free = p4",
                )
        }
    })
