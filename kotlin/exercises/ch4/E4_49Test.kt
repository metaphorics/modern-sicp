// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.49

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_49Test :
    FunSpec({
        test("Exercise 4.49: the first six generated sentences bore their way down one recursion").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val sentences = generatedSentences()
            sentences.size shouldBe 6
            sentences[0] shouldBe
                "(sentence (simple-noun-phrase (article the) (noun student)) (verb studies))"
            sentences[1] shouldBe
                "(sentence (simple-noun-phrase (article the) (noun student)) (verb-phrase " +
                "(verb studies) (prep-phrase (prep for) " +
                "(simple-noun-phrase (article the) (noun student)))))"
        }
    })
