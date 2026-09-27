// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.1.1

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The deeply nested combination of the text, laid out as the text lays it out. */
public val prettyPrintedCombination: Long =
    3L * (
        2L * 4L +
            (3L + 5L)
    ) +
        ((10L - 7L) + 6L)

public class S1_1_1ExpressionsTest :
    FunSpec({
        test("primitive expressions evaluate to the numbers they name") {
            486L shouldBe 486L
            137L + 349L shouldBe 486L
            1000L - 334L shouldBe 666L
            5L * 99L shouldBe 495L
            10L / 5L shouldBe 2L
            2.7 + 10.0 shouldBe 12.7
        }
        test("a chain of operators covers what variadic prefix forms covered") {
            21L + 35L + 12L + 7L shouldBe 75L
            25L * 4L * 12L shouldBe 1_200L
        }
        test("nesting combines the values of subexpressions") {
            3L * 5L + (10L - 6L) shouldBe 19L
        }
        test("the pretty-printed expression evaluates to 57") {
            prettyPrintedCombination shouldBe 57L
        }
    })
