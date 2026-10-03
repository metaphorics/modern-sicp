// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.2

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_02Test :
    FunSpec({
        test("Exercise 4.2a: a definition reaches the application arm as an unbound operator").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            applicationsFirstTranscript() shouldBe "define-as-application = null\n"
        }

        test("Exercise 4.2b: the call-prefixed application applies like the bare one").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            callSyntaxTranscript() shouldBe "call = 49\nbare = 49\n"
        }
    })
