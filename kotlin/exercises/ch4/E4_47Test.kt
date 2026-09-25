// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.47

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_47Test :
    FunSpec({
        test("Exercise 4.47: Louis's version delivers the text's first parse").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            louisFirstParse() shouldBe
                "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))"
        }

        test("Exercise 4.47: Louis's try-again diverges once the input is spent").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            louisTryAgainFault() shouldBe "choice budget exhausted after 500 choices"
        }

        test("Exercise 4.47: the interchanged order diverges outright").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            interchangedFault() shouldBe "choice budget exhausted after 300 choices"
        }
    })
