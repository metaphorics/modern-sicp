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

        test("Exercise 4.47: try-again reaches the host search horizon after one parse") {
            louisTryAgainFault() shouldBe "answers: 1, choices: 500"
        }

        test("Exercise 4.47: recursion-first reaches the horizon before a parse") {
            interchangedFault() shouldBe "answers: 0, choices: 300"
        }
    })
