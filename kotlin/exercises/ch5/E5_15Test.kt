// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.15

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_15Test :
    FunSpec({
        test("Exercise 5.15: counting executed instructions").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            gcdInstructionCounts() shouldBe emptyList()
        }
    })
