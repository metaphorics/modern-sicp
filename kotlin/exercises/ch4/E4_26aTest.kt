// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26a

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_26aTest :
    FunSpec({
        test("Exercise 4.26a: before the derivation, when is an unbound application").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            whenBeforeTranscript() shouldBe "Error: unbound variable: when\n"
        }

        test("Exercise 4.26a: after the derivation, when evaluates").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            whenAfterTranscript() shouldBe "yes\n"
        }

        test("Exercise 4.26a: a false condition answers the missing alternative").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            whenNoElseTranscript() shouldBe "#f\n"
        }

        test("Exercise 4.26a: a multi-expression body answers the last expression").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            whenBodySequenceTranscript() shouldBe "3\n"
        }
    })
