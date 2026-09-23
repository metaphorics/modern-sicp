// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.35

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe
import kotlin.math.sqrt

public class E1_35Test :
    FunSpec({
        test("the fixed-point search lands within tolerance of the closed-form golden ratio") {
            val goldenRatio = (1.0 + sqrt(5.0)) / 2.0
            ex_1_35() shouldBe (goldenRatio plusOrMinus 0.00001)
        }
        test("the computed value matches, exactly, across repeated runs") {
            ex_1_35() shouldBe 1.6180327868852458
        }
    })
