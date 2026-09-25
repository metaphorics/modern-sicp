// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.13

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_13Test :
    FunSpec({
        test("Exercise 4.13: make-unbound! removes the binding and the next lookup faults") {
            unboundTranscript() shouldBe "3\nok\nError: unbound variable: x\n"
        }

        test("Exercise 4.13: unbinding a shadow restores the outer binding") {
            unboundShadowTranscript() shouldBe "1\n1\n"
        }

        test("Exercise 4.13: unbinding an absent name is a no-op answering ok") {
            unboundAbsentTranscript() shouldBe "ok\n"
        }
    })
