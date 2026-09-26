// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.30

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_30Test :
    FunSpec({
        test("Exercise 5.30: the caught failures and the clean factorial").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            errorSignalingRuns() shouldBe emptyList()
        }
    })
