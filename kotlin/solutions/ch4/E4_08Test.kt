// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.8: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_08Test :
    FunSpec({
        test("Exercise 4.8: the named loop sums the countdown") {
            namedLetTranscript() shouldBe "15\n"
        }
    })
