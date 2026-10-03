// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.6

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_06Test :
    FunSpec({
        test("Exercise 4.6: the rewrite is the `GLam`/`GApp` application, answering alike").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letEquivalenceTranscript() shouldBe "7\n7\n"
        }

        test("Exercise 4.6: the derived let computes the body in the new frame").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letBodyTranscript() shouldBe "7\n"
        }

        test("Exercise 4.6: lets nest as derived expressions at every depth").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letNestedTranscript() shouldBe "3\n"
        }
    })
