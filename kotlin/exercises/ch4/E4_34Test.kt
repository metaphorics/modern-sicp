// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.34

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_34Test :
    FunSpec({
        test("Exercise 4.34: dotted and proper lazy pairs print in their shapes").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lazyPairPrintTranscript() shouldBe "[1 | 2]\n[1, 2]\n"
        }

        test("Exercise 4.34: the infinite list prints prefix plus ellipsis").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            onesBudgetPrintTranscript() shouldBe "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ...]\n"
        }

        test("Exercise 4.34: car forces only the demanded element").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            carOfOnesTranscript() shouldBe "1\n"
        }

        test("Exercise 4.34: nested lazy pairs print recursively").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            nestedLazyPrintTranscript() shouldBe "[[1], 2]\n"
        }
    })
