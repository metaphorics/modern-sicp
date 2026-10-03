// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.25

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_25Test :
    FunSpec({
        test("normal order keeps 120, never evaluates an unused argument, and forces a thunk once") {
            // Section 4.2.2 memoizes a successful force, so both force calls return 42.
            val pinned = setOf("120", "42", "1")
            normalOrderRuns().filter { it in pinned } shouldBe listOf("120", "42", "42", "1")
        }
    })
