// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.47

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_47Test :
    FunSpec({
        test("Exercise 5.47: the compound branch is shown and the book's session answers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            compoundCallRuns() shouldBe emptyList()
        }
    })
