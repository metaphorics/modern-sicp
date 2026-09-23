// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.15

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.doubles.shouldBeLessThan
import io.kotest.matchers.shouldBe

public class E2_15Test :
    FunSpec({
        test("Eva Lu Ator is right: par2, with no repeated variable, is strictly tighter than par1") {
            val (par1Percent, par2Percent) = ex_2_15()
            par2Percent shouldBeLessThan par1Percent
        }
        test("the two percentages match the resistor example exactly") {
            val (par1Percent, par2Percent) = ex_2_15()
            par1Percent shouldBe (0.22613352145193347 plusOrMinus 1e-9)
            par2Percent shouldBe (0.0705260392723452 plusOrMinus 1e-9)
        }
    })
