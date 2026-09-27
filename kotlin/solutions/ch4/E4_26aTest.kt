// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26a

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_26aTest :
    FunSpec({
        test("Exercise 4.26a: before the derivation, when is an unbound application") {
            whenBeforeTranscript() shouldBe "Error: unbound variable: when\n"
        }

        test("Exercise 4.26a: after the derivation, when evaluates") {
            whenAfterTranscript() shouldBe "yes\n"
        }

        test("Exercise 4.26a: a false condition answers the missing alternative") {
            whenNoElseTranscript() shouldBe "#f\n"
        }

        test("Exercise 4.26a: a multi-expression body answers the last expression") {
            whenBodySequenceTranscript() shouldBe "3\n"
        }
    })
