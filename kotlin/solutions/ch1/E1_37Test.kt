// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.37

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.abs

private const val ONE_OVER_PHI = 0.6180339887498948

public class E1_37Test :
    FunSpec({
        test("k=9 is not yet accurate to 4 decimal places, k=10 is") {
            val nine = contFrac({ 1.0 }, { 1.0 }, 9L)
            val ten = contFrac({ 1.0 }, { 1.0 }, 10L)
            (abs(nine - ONE_OVER_PHI) >= 0.0001) shouldBe true
            (abs(ten - ONE_OVER_PHI) < 0.0001) shouldBe true
        }
        test("ex_1_37 returns k=10 and matching recursive/iterative approximations") {
            ex_1_37() shouldBe Triple(10L, 0.6179775280898876, 0.6179775280898876)
        }
        test("contFrac and contFracIterative agree for every k from 1 to 20") {
            for (k in 1L..20L) {
                contFrac({ 1.0 }, { 1.0 }, k) shouldBe contFracIterative({ 1.0 }, { 1.0 }, k)
            }
        }
    })
