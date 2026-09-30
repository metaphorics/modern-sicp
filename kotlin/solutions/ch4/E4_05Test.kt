// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.5: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_05Test :
    FunSpec({
        test("Exercise 4.5: the arrow clause evaluates its test exactly once") {
            arrowClauseTranscript() shouldBe "42\n1\n7\n1\n"
        }
    })
