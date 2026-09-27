// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.1

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_01Test :
    FunSpec({
        test("the arithmetic expressions evaluate in order") {
            ex_1_01().take(5) shouldBe listOf(10L, 12L, 8L, 3L, 6L)
        }
        test("the names session evaluates in order") {
            val a = 3L
            val b = a + 1L
            (a == b) shouldBe false
            ex_1_01().drop(5) shouldBe listOf(19L, 4L, 16L, 6L, 16L)
        }
    })
