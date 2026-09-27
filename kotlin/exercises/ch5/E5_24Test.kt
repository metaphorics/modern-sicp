// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.24

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_24Test :
    FunSpec({
        test("Exercise 5.24: the cond sessions through the clause-loop evaluator").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            condBasicFormRuns() shouldBe emptyList()
        }
    })
