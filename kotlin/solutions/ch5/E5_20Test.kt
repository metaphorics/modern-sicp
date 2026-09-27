// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.20

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_20Test :
    FunSpec({
        test("Exercise 5.20: the cells come out at p1, p2, p3, both elements of y are p1, free ends at p4") {
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
