// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_31

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_31Test :
    FunSpec({
        test("Exercise 5.31: each combination's saves are read back by the register analysis") {
            val report = superfluousSaves()
            report.size shouldBe 6
            report[4] shouldBe "every save pairs with one restore: true"
            report[5] shouldBe "compiled and direct runs agree: true"
        }
    })
