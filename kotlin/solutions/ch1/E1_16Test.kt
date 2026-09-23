// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.16

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_16Test :
    FunSpec({
        test("the iterative process computes 2^10") {
            ex_1_16(2L, 10L) shouldBe 1024L
        }
        test("an odd exponent and a base of 1 are both handled") {
            ex_1_16(3L, 13L) shouldBe 1_594_323L
            ex_1_16(5L, 0L) shouldBe 1L
        }
    })
