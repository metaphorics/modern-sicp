// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.40

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_40Test :
    FunSpec({
        test("uniquePairs(6) enumerates the 15 ordered pairs, larger i first") {
            ex_2_40().size shouldBe 15
            ex_2_40().first() shouldBe listOf(2L, 1L)
            ex_2_40().last() shouldBe listOf(6L, 5L)
        }
        test("uniquePairs never repeats a pair in both orders") {
            val pairs = uniquePairs(5L)
            pairs.forEach { (i, j) -> require(i > j) }
            pairs.toSet().size shouldBe pairs.size
        }
        test("primeSumPairsViaUniquePairs finds the same prime sums as the chapter text") {
            primeSumPairsViaUniquePairs(6L).map { it[2] } shouldBe listOf(3L, 5L, 5L, 7L, 7L, 7L, 11L)
        }
    })
