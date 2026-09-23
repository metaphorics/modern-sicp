// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.46

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.cos

public class E1_46Test :
    FunSpec({
        test("sqrtViaIterativeImprove(9) matches section 1.1.7's sqrt exactly") {
            sqrtViaIterativeImprove(9.0) shouldBe 3.00009155413138
        }
        test("fixedPointViaIterativeImprove(cos, 1.0) lands within tolerance of section 1.3.3's fixedPoint") {
            fixedPointViaIterativeImprove(::cos, 1.0) shouldBe 0.7390893414033927
        }
        test("ex_1_46 packages both rewrites") {
            ex_1_46() shouldBe Pair(3.00009155413138, 0.7390893414033927)
        }
    })
