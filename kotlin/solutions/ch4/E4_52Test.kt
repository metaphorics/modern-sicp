// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.52: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E452Test :
    FunSpec({
        test("Exercise 4.52: all odd means the alternative answers") {
            ifFailAllOddTranscript() shouldBe "all-odd\n"
        }

        test("Exercise 4.52: one even answer first, then the alternative after exhaustion") {
            ifFailEightTranscript() shouldBe "8\nall-odd\n"
        }
    })
