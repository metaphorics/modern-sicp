// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.10: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_10Test :
    FunSpec({
        test("Exercise 4.10: lowered syntax defines and applies") {
            newSyntaxTranscript() shouldBe "49\n42\n"
        }
    })
