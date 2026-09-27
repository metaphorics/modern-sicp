// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.17

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_17Test :
    FunSpec({
        test("3 times 7 by doubling and halving") {
            ex_1_17(3L, 7L) shouldBe 21L
        }
        test("a zero factor and an even factor are both handled") {
            ex_1_17(0L, 5L) shouldBe 0L
            ex_1_17(6L, 8L) shouldBe 48L
        }
    })
