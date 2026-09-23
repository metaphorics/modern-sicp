// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.31

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E1_31Test :
    FunSpec({
        test("Exercise 1.31: factorial via product, and the Wallis approximation to pi").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_1_31() shouldBe Pair(720L, 3.143160705532257)
        }
    })
