// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.41

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_41Test :
    FunSpec({
        test("tripleSum(5, 10) finds (1 4 5) and (2 3 5)") {
            ex_2_41() shouldBe listOf(listOf(1L, 4L, 5L), listOf(2L, 3L, 5L))
        }
        test("tripleSum(6, 15) finds only (4 5 6)") {
            tripleSum(6L, 15L) shouldBe listOf(listOf(4L, 5L, 6L))
        }
        test("tripleSum(3, 100) finds nothing") {
            tripleSum(3L, 100L) shouldBe emptyList()
        }
    })
