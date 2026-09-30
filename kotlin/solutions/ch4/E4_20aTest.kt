// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20a

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_20aTest :
    FunSpec({
        test("Exercise 4.20a: the shadowing initializer reads the inner f before its assignment") {
            shadowedPrematureReadTranscript() shouldBe "error\n"
        }

        test("Exercise 4.20a: with the outer binding in scope the same program recurses instead") {
            outerScopeRecursionTranscript() shouldBe "1\n"
        }
    })
