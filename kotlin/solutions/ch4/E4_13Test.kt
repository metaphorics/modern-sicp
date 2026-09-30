// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.13: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_13Test :
    FunSpec({
        test("Exercise 4.13: unbinding removes the global binding") {
            unboundTranscript() shouldBe "3\nok\nerror\n"
        }

        test("Exercise 4.13: unbinding a shadow reveals the outer binding") {
            unboundShadowTranscript() shouldBe "1\n1\n"
        }

        test("Exercise 4.13: unbinding an absent name answers ok") {
            unboundAbsentTranscript() shouldBe "ok\n"
        }
    })
