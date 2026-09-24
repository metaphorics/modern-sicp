// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.19

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_19Test :
    FunSpec({
        test("cc(100, usCoins) counts 292 ways, the book's answer") {
            cc(100L, usCoins) shouldBe 292L
        }
        test("the order of the coin list does not affect the answer") {
            cc(100L, usCoins.asReversed()) shouldBe 292L
        }
        test("changing 11 with pennies, nickels, and dimes has 4 ways") {
            cc(11L, listOf(10L, 5L, 1L)) shouldBe 4L
        }
        test("ex_2_19 returns 292") {
            ex_2_19() shouldBe 292L
        }
    })
