// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.46

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_46Test :
    FunSpec({
        test("Exercise 4.46: the left operand cycles last, proving left-to-right").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            operandOrderEnumeration() shouldBe listOf("(1 3)", "(1 4)", "(2 3)", "(2 4)")
        }
    })
