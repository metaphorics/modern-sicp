// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.12: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_12Test :
    FunSpec({
        test("Exercise 4.12: lookup and set! through the scan abstractions") {
            scannedTranscript() shouldBe "2\n10\n10\n"
        }

        test("Exercise 4.12: the walk stops without a hit past the call frame") {
            scannedFreshFrameTranscript() shouldBe "2\nerror\n"
        }

        test("Exercise 4.12: extension refuses an arity mismatch") {
            scannedArityTranscript() shouldBe "error\n"
        }
    })
