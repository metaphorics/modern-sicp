// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.12

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_12Test :
    FunSpec({
        test("Exercise 5.12: the assembler's summary of the gcd machine").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            gcdMachineSummary() shouldBe "MEASURE"
        }
    })
