// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.33

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_33Test :
    FunSpec({
        test("Exercise 4.33: plain data is not a procedural pair").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            plainQuoteCarTranscript() shouldBe "null"
        }

        test("Exercise 4.33: the lifted constructor builds a lazy pair").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lazyQuoteCarTranscript() shouldBe "a\n"
        }

        test("Exercise 4.33: the section's list operations run on constructed lists").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lazyQuoteListRefTranscript() shouldBe "d\n"
        }
    })
