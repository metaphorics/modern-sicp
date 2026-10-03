// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.5

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_05Test :
    FunSpec({
        test("Exercise 4.5: the book's lookup example answers through the arrow").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            arrowLookupTranscript() shouldBe "2\n"
        }

        test("Exercise 4.5: the arrow test evaluates exactly once").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            arrowTestOnceTranscript() shouldBe "2\n1\n"
        }

        test("Exercise 4.5: the recipient receives the test's value, not its truth").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            arrowValueTranscript() shouldBe "198\n"
        }

        test("Exercise 4.5: plain clauses still chain in order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            plainClausesTranscript() shouldBe "second\n"
        }

        test("Exercise 4.5: no match with no fallback answers false").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            noMatchTranscript() shouldBe "false\n"
        }
    })
