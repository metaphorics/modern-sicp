// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.1

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_01Test :
    FunSpec({
        test("Exercise 4.1: the kernel's statement order evaluates operands left to right").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            leftToRightTranscript() shouldBe "(1 . 2)\n1\n2\n"
        }

        test("Exercise 4.1: the right-to-left list-of-values runs the operands backwards").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            rightToLeftTranscript() shouldBe "(1 . 2)\n2\n1\n"
        }

        test("Exercise 4.1: the operands deliver the same values either way").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            leftToRightTranscript().lines().first() shouldBe rightToLeftTranscript().lines().first()
        }
    })
