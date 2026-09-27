// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.2

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_02Test :
    FunSpec({
        test("the nested-call form equals the infix form") {
            val numerator = 5.0 + 4.0 + (2.0 - (3.0 - (6.0 + 4.0 / 5.0)))
            val denominator = 3.0 * (6.0 - 2.0) * (2.0 - 7.0)
            ex_1_02() shouldBe numerator / denominator
            ex_1_02() shouldBe -0.24666666666666667
        }
    })
