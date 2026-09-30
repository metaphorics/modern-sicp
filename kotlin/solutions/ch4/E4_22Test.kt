// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.22: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_22Test :
    FunSpec({
        test("Exercise 4.22: one analysis serves two executions") {
            analyzedLetTranscript() shouldBe "7\n7\n1\n2\n"
        }
    })
