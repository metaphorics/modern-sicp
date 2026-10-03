// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_32

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_32Test :
    FunSpec({
        test("Exercise 5.32: the two dispatch styles compile to comparable cost and answer alike") {
            symbolOperatorComparison().last() shouldBe "the two runs answer alike: true"
        }
    })
