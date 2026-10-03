// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_34

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_34Test :
    FunSpec({
        test("Exercise 5.34: the compiled iterative factorial keeps its depth independent of n") {
            compiledFactorialAnnotation().last() shouldBe "compiled maximum depth independent of n: true"
        }
    })
