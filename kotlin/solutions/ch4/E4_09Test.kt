// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.9: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_09Test :
    FunSpec({
        test("Exercise 4.9: while and until accumulate through the derived loops") {
            loopsTranscript() shouldBe "4\n10\n3\n12\n"
        }
    })
