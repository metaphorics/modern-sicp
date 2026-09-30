// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_35

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_35Test :
    FunSpec({
        test("Exercise 5.35: the recovered source round-trips through the compiler") {
            reverseEngineeredFigure().last() shouldBe "recompiling reproduces the same statements: true"
        }
    })
