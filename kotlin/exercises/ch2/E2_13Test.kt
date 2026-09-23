// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.13

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E2_13Test :
    FunSpec({
        test("Exercise 2.13: the exact and approximate tolerances agree to within 1e-4").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val (exact, approx) = ex_2_13()
            approx shouldBe 0.03
            exact shouldBe (approx plusOrMinus 1e-4)
        }
    })
