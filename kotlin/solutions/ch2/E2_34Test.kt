// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.34

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_34Test :
    FunSpec({
        test("hornerEval(2, (1 3 0 5 0 1)) is 79") {
            ex_2_34() shouldBe 79L
        }
        test("hornerEval(3, (1 2)) evaluates 1 + 2x to 7") {
            hornerEval(3L, listOf(1L, 2L)) shouldBe 7L
        }
        test("the zero polynomial evaluates to 0") {
            hornerEval(5L, emptyList()) shouldBe 0L
        }
    })
