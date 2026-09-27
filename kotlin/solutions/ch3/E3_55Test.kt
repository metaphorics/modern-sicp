// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.55

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_55Test :
    FunSpec({
        test("Exercise 3.55: partial sums of the integers are the triangular numbers") {
            partialSums(integers).take(10) shouldBe
                listOf(1L, 3L, 6L, 10L, 15L, 21L, 28L, 36L, 45L, 55L)
            partialSums(ones).take(6) shouldBe listOf(1L, 2L, 3L, 4L, 5L, 6L)
        }

        test("Exercise 3.55: the pi series rows come out of the real partial sums") {
            val summands =
                streamMap(
                    { k -> if (k % 2 == 0L) 1.0 / (2 * k + 1) else -1.0 / (2 * k + 1) },
                    integersStartingFrom(0L),
                )
            scaleStream(partialSums(summands), 4.0).take(8) shouldBe
                listOf(
                    4.0,
                    2.666666666666667,
                    3.466666666666667,
                    2.8952380952380956,
                    3.3396825396825403,
                    2.9760461760461765,
                    3.2837384837384844,
                    3.017071817071818,
                )
        }
    })
