// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.38

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_38Test :
    FunSpec({
        test("Exercise 1.38: Euler's continued fraction for e, at 20 terms, matches kotlin.math.E").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_38() shouldBe kotlin.math.E
        }
    })
