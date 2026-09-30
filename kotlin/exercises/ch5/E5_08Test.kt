// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.08

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_08Test :
    FunSpec({
        test("Exercise 5.08: a doubly defined label is an assembly error").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            duplicateLabelOutcome() shouldBe "DuplicateLabel: here"
        }
    })
