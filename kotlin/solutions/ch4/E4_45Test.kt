// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.45

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_45Test :
    FunSpec({
        test("Exercise 4.45: exactly five parses, then exhaustion") {
            val parses = ambiguousParses()
            parses.size shouldBe 5
            parses[0] shouldBe
                "(sentence (simple-noun-phrase (article the) (noun professor)) " +
                "(verb-phrase (verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) " +
                "(simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep in) " +
                "(simple-noun-phrase (article the) (noun class)))) (prep-phrase (prep with) " +
                "(simple-noun-phrase (article the) (noun cat)))))"
            parses[4] shouldBe
                "(sentence (simple-noun-phrase (article the) (noun professor)) " +
                "(verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase " +
                "(simple-noun-phrase (article the) (noun student)) (prep-phrase (prep in) " +
                "(noun-phrase (simple-noun-phrase (article the) (noun class)) " +
                "(prep-phrase (prep with) (simple-noun-phrase (article the) (noun cat)))))))))"
        }
    })
