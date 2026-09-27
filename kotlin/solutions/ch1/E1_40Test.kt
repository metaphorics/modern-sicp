// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.40

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E1_40Test :
    FunSpec({
        test("newtonsMethod on cubic(0, 0, -8) from guess 1.0 finds the cube root of 8") {
            ex_1_40() shouldBe 2.000000000036784
        }
        test("cubic(-6, 11, -6), which factors as (x-1)(x-2)(x-3), yields a root from any of its three guesses") {
            newtonsMethod(cubic(-6.0, 11.0, -6.0), 1.0) shouldBe 1.0
            newtonsMethod(cubic(-6.0, 11.0, -6.0), 4.0) shouldBe (3.0 plusOrMinus 1e-9)
        }
    })
