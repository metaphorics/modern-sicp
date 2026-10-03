// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.4

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_04Test :
    FunSpec({
        test("Exercise 4.4: the book's and examples").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            andExamplesTranscript() shouldBe "true\ntrue\n3\nfalse\n"
        }

        test("Exercise 4.4: the book's or examples").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            orExamplesTranscript() shouldBe "true\nfalse\n7\na\n"
        }

        test("Exercise 4.4: and answers the last value, or the first true one").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            shortCircuitTranscript() shouldBe "false\nb\n"
        }

        test("Exercise 4.4: the forms re-enter the whole evaluator, nested").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            nestedFormsTranscript() shouldBe "2\n"
        }
    })
