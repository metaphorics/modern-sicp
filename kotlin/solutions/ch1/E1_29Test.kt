// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.29

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.abs

public class E1_29Test :
    FunSpec({
        test("Simpson's Rule at n=100 and n=1000 lands within 1e-16 of the exact 1/4") {
            ex_1_29() shouldBe Pair(0.25000000000000006, 0.25000000000000006)
        }
        test("both are closer to 1/4 than section 1.3.1's integral at the same n") {
            val integralAt001 = 0.24998750000000042
            val integralAt0001 = 0.249999875000001
            val (n100, n1000) = ex_1_29()
            (abs(n100 - 0.25) < abs(integralAt001 - 0.25)) shouldBe true
            (abs(n1000 - 0.25) < abs(integralAt0001 - 0.25)) shouldBe true
        }
    })
