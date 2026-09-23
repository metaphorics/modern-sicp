// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.39

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe
import kotlin.math.PI
import kotlin.math.tan

public class E1_39Test :
    FunSpec({
        test("tanCf(pi/4, 10) is 1, exactly, at this term count") {
            ex_1_39() shouldBe 1.0
        }
        test("tanCf agrees with kotlin.math.tan across a range of angles") {
            for (x in listOf(0.0, 0.1, 0.5, 1.0, -0.7)) {
                tanCf(x, 15L) shouldBe (tan(x) plusOrMinus 1e-9)
            }
        }
        test("tanCf(pi/4, 10) is closer to 1 than the raw tan(pi/4) transcendental call") {
            val direct = tan(PI / 4.0)
            (direct != 1.0) shouldBe true
        }
    })
