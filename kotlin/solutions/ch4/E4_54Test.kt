// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.54

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_54Test :
    FunSpec({
        test("Exercise 4.54: the special form prunes the odd choices") {
            requireFilteredEvens() shouldBe listOf("2", "4")
        }

        test("Exercise 4.54: a satisfied requirement answers ok") {
            requireSatisfied() shouldBe "ok"
        }
    })
