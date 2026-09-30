// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_38

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_38Test :
    FunSpec({
        test("Exercise 5.38: the arithmetic probe answers alike however it is dispatched") {
            openCodingReport().last() shouldBe "compiled and direct runs agree: true"
        }
    })
