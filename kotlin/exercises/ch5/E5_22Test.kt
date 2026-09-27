// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.22

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_22Test :
    FunSpec({
        test("Exercise 5.22: the append and append! machines over the memory").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            appendRuns() shouldBe emptyList()
        }
    })
