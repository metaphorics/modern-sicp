// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.11: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_11Test :
    FunSpec({
        test("Exercise 4.11: define, lookup, and outward set! over alist frames") {
            alistTranscript() shouldBe "2\n10\n10\n"
        }

        test("Exercise 4.11: a call-frame binding dies with the call") {
            alistFreshFrameTranscript() shouldBe "2\nerror\n"
        }

        test("Exercise 4.11: extension refuses an arity mismatch") {
            alistArityTranscript() shouldBe "error\n"
        }
    })
