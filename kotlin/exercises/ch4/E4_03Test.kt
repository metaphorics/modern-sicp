// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.3

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_03Test :
    FunSpec({
        test("Exercise 4.3: the case analysis dispatches through the table").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            caseThroughTableTranscript() shouldBe "yes\n"
        }

        test("Exercise 4.3: definitions and applications run through table clauses").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            applicationThroughTableTranscript() shouldBe "49\n"
        }

        test("Exercise 4.3: constructor data denotes itself through the table").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            constructedDataTranscript() shouldBe "[a, b]\n"
        }

        test("Exercise 4.3: self-evaluating values and variables stay residual cases").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            residualDispatchTranscript() shouldBe "19\n"
        }

        test("Exercise 4.3: a clause installed after construction extends the language").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            installedClauseTranscript() shouldBe "ran\nno\n"
        }

        test("Exercise 4.3: a later put overwrites an installed clause").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            overwrittenClauseTranscript() shouldBe "replaced\n"
        }
    })
