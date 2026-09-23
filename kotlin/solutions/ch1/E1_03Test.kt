// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.3

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_03Test :
    FunSpec({
        test("the two larger numbers are squared and summed") {
            ex_1_03(1L, 2L, 3L) shouldBe 13L
            ex_1_03(3L, 2L, 1L) shouldBe 13L
            ex_1_03(2L, 3L, 1L) shouldBe 13L
            ex_1_03(2L, 2L, 3L) shouldBe 13L
        }
        test("ties count both, and negatives square as themselves") {
            ex_1_03(5L, 5L, 5L) shouldBe 50L
            ex_1_03(-1L, -2L, -3L) shouldBe 5L
        }
    })
