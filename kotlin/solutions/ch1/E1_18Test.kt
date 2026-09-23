// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.18

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_18Test :
    FunSpec({
        test("3 times 7, iteratively") {
            ex_1_18(3L, 7L) shouldBe 21L
        }
        test("agrees with exercise 1.17's recursive version across several pairs") {
            listOf(0L to 5L, 6L to 8L, 12L to 13L).forEach { (a, b) ->
                ex_1_18(a, b) shouldBe ex_1_17(a, b)
            }
        }
    })
