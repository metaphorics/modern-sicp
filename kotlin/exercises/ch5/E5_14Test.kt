// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.14

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_14Test :
    FunSpec({
        test("Exercise 5.14: factorial stack statistics as a function of n").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            factorialStackStatistics() shouldBe emptyList()
        }
    })
