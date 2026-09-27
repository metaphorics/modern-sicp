// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.33

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_33Test :
    FunSpec({
        test("Exercise 4.33: a data quote is not a procedural pair") {
            plainQuoteCarTranscript() shouldBe "Error: not a procedure: (a b c)\n"
        }

        test("Exercise 4.33: the lifted quote builds a lazy pair") {
            lazyQuoteCarTranscript() shouldBe "a\n"
        }

        test("Exercise 4.33: the section's list operations run on quoted lists") {
            lazyQuoteListRefTranscript() shouldBe "d\n"
        }
    })
