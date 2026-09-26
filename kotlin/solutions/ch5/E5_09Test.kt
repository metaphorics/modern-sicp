// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_09

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_09Test :
    FunSpec({
        test("Exercise 5.9: the strict assembler refuses a label used as an operation operand") {
            labelOperandOutcome() shouldBe
                "an operation input is written (reg r) or (const c), not (label b)"
        }
        test("Exercise 5.9: the book's base assembler computes the label's instruction address") {
            labelOperandUnderBaseAssembler() shouldBe 5L
        }
    })
