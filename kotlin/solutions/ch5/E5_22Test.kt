// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.22

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_22Test :
    FunSpec({
        test("Exercise 5.22: append copies three cells and shares y, append! splices without allocating") {
            appendRuns() shouldBe
                listOf(
                    "append: z = p7 = (1 2 3 4 5)",
                    "append: x is still (1 2 3) (p2), free moved to p8, three fresh cells",
                    "append!: before, the last pair of x points at e0:",
                    "index    0   1   2   3   4   5   6   7\n" +
                        "the-cars n3  n2  n1  n5  n4  e0  e0  e0\n" +
                        "the-cdrs e0  p0  p1  e0  p3  e0  e0  e0",
                    "append!: after, it points at y:",
                    "index    0   1   2   3   4   5   6   7\n" +
                        "the-cars n3  n2  n1  n5  n4  e0  e0  e0\n" +
                        "the-cdrs p4  p0  p1  e0  p3  e0  e0  e0",
                    "append!: the answer is x itself, now (1 2 3 4 5), the same pointer p2 " +
                        "the caller passed, and free is still p5",
                )
        }
    })
