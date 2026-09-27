// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.35

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_35Test :
    FunSpec({
        test("the identified expression reproduces Figure 5.18") {
            val result = figure5_18Compilation()
            result.first() shouldBe "compiled to the figure: (define (f x) (+ x (g (+ x 2))))"
            result.last() shouldBe "figure matches: true"
        }
    })
