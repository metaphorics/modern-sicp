// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.70

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

public class E3_70Test :
    FunSpec({
        test("Exercise 3.70: mergeWeighted keeps equal weights adjacent").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val left = consStream(1L to 1L) { consStream(1L to 5L) { LStream.Empty } }
            val right = consStream(2L to 2L) { consStream(3L to 3L) { LStream.Empty } }
            mergeWeighted(left, right) { (i, j) -> i + j }.take(4) shouldBe
                listOf(1L to 1L, 2L to 2L, 1L to 5L, 3L to 3L)
        }

        test("Exercise 3.70a: integer pairs ordered by the sum i + j").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            weightedPairs(integers, integers) { (i, j) -> i + j }.take(10) shouldBe
                listOf(
                    1L to 1L,
                    1L to 2L,
                    1L to 3L,
                    2L to 2L,
                    1L to 4L,
                    2L to 3L,
                    1L to 5L,
                    2L to 4L,
                    3L to 3L,
                    1L to 6L,
                )
        }

        test("Exercise 3.70b: pairs of 2, 3, 5-free integers ordered by 2i + 3j + 5ij").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            integersNo235.take(8) shouldBe listOf(1L, 7L, 11L, 13L, 17L, 19L, 23L, 29L)
            weightedPairs(integersNo235, integersNo235) { (i, j) -> 2 * i + 3 * j + 5 * i * j }.take(10) shouldBe
                listOf(
                    1L to 1L,
                    1L to 7L,
                    1L to 11L,
                    1L to 13L,
                    1L to 17L,
                    1L to 19L,
                    1L to 23L,
                    1L to 29L,
                    1L to 31L,
                    7L to 7L,
                )
        }
    })
