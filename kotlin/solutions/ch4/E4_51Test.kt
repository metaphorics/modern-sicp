// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.51: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E451Test :
    FunSpec({
        test("Exercise 4.51: permanent-set! accumulates across answers and survives") {
            permanentSetTranscript() shouldBe "(a b 2)\n(a c 3)\n(b a 4)\n4\n"
        }

        test("Exercise 4.51: set! rolls each failed trial back") {
            setBangTranscript() shouldBe "(a b 1)\n(a c 1)\n(b a 1)\n0\n"
        }
    })
