// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.11

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_11Test :
    FunSpec({
        test("Exercise 5.11: the save and restore disciplines compared").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            restoreDisciplineRuns() shouldBe emptyList()
        }
    })
