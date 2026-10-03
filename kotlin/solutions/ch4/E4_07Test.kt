// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.7: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_07Test :
    FunSpec({
        test("Exercise 4.7: sequential inits read predecessors, shadowing binds inward") {
            letStarTranscript() shouldBe "7\n2\n"
        }
    })
