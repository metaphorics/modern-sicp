// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.15

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_15Test :
    FunSpec({
        test("Exercise 4.15: with halts? answering #f the run halts").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            boundedTranscript(haltsAnswer = false) shouldBe "halts\n"
        }

        test("Exercise 4.15: with halts? answering #t the budget fires, the typed fault").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            boundedTranscript(haltsAnswer = true) shouldBe
                "Error: machine fault: step budget exhausted after 1000 steps\n"
        }
    })
