// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26a: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E426aTest :
    FunSpec({
        test("Exercise 4.26a: before the derivation the name is unbound") {
            whenBeforeTranscript() shouldBe "error\n"
        }

        test("Exercise 4.26a: the derived when evaluates") {
            whenAfterTranscript() shouldBe "yes\n"
        }

        test("Exercise 4.26a: a false condition with no else arm answers the derived false") {
            whenNoElseTranscript() shouldBe "false\n"
        }

        test("Exercise 4.26a: a multi-expression body answers its last expression") {
            whenBodySequenceTranscript() shouldBe "3\n"
        }

        test("Exercise 4.26a: the lazy mirror leaves the unchosen arm unforced") {
            whenLazyTranscript() shouldBe "42\n"
        }
    })
