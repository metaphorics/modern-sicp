// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E0_01Test :
    FunSpec({
        test("the arithmetic session evaluates in order") {
            listOf(486L, 100L, 12L, 1L, 6L).forEachIndexed { i, expected ->
                ex_0_01()[i] shouldBe expected
            }
        }
        test("the names session evaluates in order") {
            val a = 3L
            val b = 4L
            (a == b) shouldBe false
            ex_0_01().drop(5).take(5) shouldBe listOf(19L, 4L, 16L, 6L, 16L)
        }
        test("the square session evaluates in order") {
            ex_0_01().drop(10) shouldBe listOf(441L, 49L, 81L)
        }
        test("the whole transcript matches the Scheme session") {
            ex_0_01() shouldBe
                listOf(486L, 100L, 12L, 1L, 6L, 19L, 4L, 16L, 6L, 16L, 441L, 49L, 81L)
        }
    })
