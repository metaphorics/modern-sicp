// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.13

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_13Test :
    FunSpec({
        test("Exercise 5.13: registers derived from the controller text").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            derivedRegisterRuns() shouldBe emptyList()
        }
    })
