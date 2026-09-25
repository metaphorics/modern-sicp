// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.47

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_47Test :
    FunSpec({
        test("Exercise 4.47: Louis's version delivers the text's first parse") {
            louisFirstParse() shouldBe
                "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))"
        }

        test("Exercise 4.47: Louis's try-again diverges once the input is spent") {
            louisTryAgainFault() shouldBe "choice budget exhausted after 500 choices"
        }

        test("Exercise 4.47: the interchanged order diverges outright") {
            interchangedFault() shouldBe "choice budget exhausted after 300 choices"
        }
    })
