// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.28

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_28Test :
    FunSpec({
        test("Exercise 4.28: the forced operator dispatches on the primitive").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            forcedOperatorTranscript() shouldBe "5\n"
        }

        test("Exercise 4.28: the unforced operator dispatches on a thunk").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            unforcedOperatorTranscript() shouldBe "Error: not a procedure: #[thunk]\n"
        }
    })
