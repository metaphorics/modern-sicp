// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.10

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_10Test :
    FunSpec({
        test("Exercise 5.10: the gcd machine in the new syntax").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            newSyntaxRuns() shouldBe emptyList()
        }
    })
