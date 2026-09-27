// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.1.3

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The tree the text draws: each node combines the values of its branches. */
public val treeAccumulation: Long = (2L + 4L * 6L) * (3L + 5L + 7L)

public class S1_1_3EvaluationTest :
    FunSpec({
        test("operand values percolate upward") {
            treeAccumulation shouldBe 390L
        }
        test("a name out of scope is a compile-time error, not a run-time one") {
            // `x + 1` does not compile unless the environment of the program
            // provides `x`; the compiler is what consults that environment.
            val x = 9L
            (x + 1L) shouldBe 10L
        }
    })
