// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.26

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_26Test :
    FunSpec({
        test("appendList, cons, and vlist print the three predicted forms") {
            ex_2_26() shouldBe listOf("(1 2 3 4 5 6)", "((1 2 3) 4 5 6)", "((1 2 3) (4 5 6))")
        }
    })
