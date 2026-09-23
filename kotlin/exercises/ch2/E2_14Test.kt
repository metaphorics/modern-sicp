// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.14

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe

public class E2_14Test :
    FunSpec({
        test("Exercise 2.14: a / a reports far more tolerance than the exact answer of zero").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val (aOverA, aOverB) = ex_2_14()
            aOverA shouldBe (0.0997506234413964 plusOrMinus 1e-9)
            aOverB shouldBe (0.14925373134328362 plusOrMinus 1e-9)
        }
    })
