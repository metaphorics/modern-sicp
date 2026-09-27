// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.4

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_04Test :
    FunSpec({
        test("a nonpositive b switches the operator to subtraction") {
            ex_1_04(2L, 3L) shouldBe 5L
            ex_1_04(2L, -3L) shouldBe 5L
            ex_1_04(7L, 0L) shouldBe 7L
            ex_1_04(-4L, 2L) shouldBe -2L
        }
    })
