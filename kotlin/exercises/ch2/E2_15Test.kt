// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.15

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E2_15Test :
    FunSpec({
        test("Exercise 2.15: par2's tolerance is far tighter than par1's, on the same resistors").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val (par1Percent, par2Percent) = ex_2_15()
            par1Percent shouldBe (0.22613352145193347 plusOrMinus 1e-9)
            par2Percent shouldBe (0.0705260392723452 plusOrMinus 1e-9)
        }
    })
