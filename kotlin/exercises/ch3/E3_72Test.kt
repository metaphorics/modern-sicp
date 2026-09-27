// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.72

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_72Test :
    FunSpec({
        test("Exercise 3.72: the first six sums of two squares in three ways").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            threeSquareNumbers().take(6) shouldBe
                listOf(
                    325L to listOf(1L to 18L, 6L to 17L, 10L to 15L),
                    425L to listOf(5L to 20L, 8L to 19L, 13L to 16L),
                    650L to listOf(5L to 25L, 11L to 23L, 17L to 19L),
                    725L to listOf(7L to 26L, 10L to 25L, 14L to 23L),
                    845L to listOf(2L to 29L, 13L to 26L, 19L to 22L),
                    850L to listOf(3L to 29L, 11L to 27L, 15L to 25L),
                )
        }

        test("Exercise 3.72: every representation squares back to its number").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            threeSquareNumbers().take(6).all { (n, reps) ->
                reps.size >= 3 && reps.all { (i, j) -> i * i + j * j == n }
            } shouldBe true
        }
    })
