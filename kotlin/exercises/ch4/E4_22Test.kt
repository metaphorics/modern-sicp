// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.22

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_22Test :
    FunSpec({
        test("Exercise 4.22: let evaluates through the analyzer at every depth").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letTranscript() shouldBe "7\n2\n5\n101\n"
        }
    })
