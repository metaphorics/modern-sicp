// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_36

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_36Test :
    FunSpec({
        test("Exercise 5.36: the operand order's saves are well paired and the runs agree") {
            operandOrderReport().last() shouldBe "compiled and direct runs agree: true"
        }
    })
