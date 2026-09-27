// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.48

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_48Test :
    FunSpec({
        test("Exercise 4.48: two adjectives attach to the noun phrase") {
            adjectiveParse() shouldBe
                "(sentence (noun-phrase (article the) ((adjective quick) (adjective brown)) " +
                "(noun cat)) (verb sleeps))"
        }

        test("Exercise 4.48: the empty modifier choice keeps plain sentences parsing") {
            noAdjectiveParse() shouldBe
                "(sentence (noun-phrase (article the) () (noun cat)) (verb sleeps))"
        }
    })
