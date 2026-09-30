// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.33: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E433Test :
    FunSpec({
        test("Exercise 4.33: an armed literal dies at construction") {
            plainArmedQuoteTranscript() shouldBe "Error: DivisionByZero\n"
        }

        test("Exercise 4.33: a lifted quote answers its head") {
            lazyQuoteCarTranscript() shouldBe "a\n"
        }

        test("Exercise 4.33: the section's list operations run on quoted lists") {
            lazyQuoteListRefTranscript() shouldBe "d\n"
        }

        test("Exercise 4.33: the armed lazy tail survives construction") {
            lazyArmedQuoteTranscript() shouldBe "1\n"
        }
    })
