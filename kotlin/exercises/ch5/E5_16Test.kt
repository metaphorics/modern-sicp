// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.16

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_16Test :
    FunSpec({
        test("Exercise 5.16: the traced gcd run").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            tracedGcdTrace() shouldBe emptyList()
        }
    })
