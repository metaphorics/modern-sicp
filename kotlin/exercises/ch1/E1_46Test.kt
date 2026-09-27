// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.46

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E1_46Test :
    FunSpec({
        test("Exercise 1.46: sqrt(9) and a fixed point of cosine, both rewritten via iterativeImprove").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val (sqrtNine, fixedPointOfCos) = ex_1_46()
            sqrtNine shouldBe (3.0 plusOrMinus 0.01)
            fixedPointOfCos shouldBe (0.7390851332151607 plusOrMinus 0.001)
        }
    })
