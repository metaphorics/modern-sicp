// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.11

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_11Test :
    FunSpec({
        test("Exercise 4.11: define, lookup, and set! run over alist frames") {
            alistTranscript() shouldBe "2\n10\n10\n"
        }

        test("Exercise 4.11: a binding that died with its call frame is unbound at top level") {
            alistFreshFrameTranscript() shouldBe "2\nError: unbound variable: z\n"
        }

        test("Exercise 4.11: extend-environment keeps the arity contract on alist frames") {
            alistArityTranscript() shouldBe "Error: extend: wrong number of arguments, expected 2, got 1\n"
        }
    })
