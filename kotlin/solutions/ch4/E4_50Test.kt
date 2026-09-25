// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.50

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_50Test :
    FunSpec({
        test("Exercise 4.50: the seeded shuffle enumerates 3, 2, 5, 1, 4") {
            rambEnumeration() shouldBe listOf("(3)", "(2)", "(5)", "(1)", "(4)")
        }

        test("Exercise 4.50: the ramb generator escapes the boring first words") {
            rambGeneratedFirst() shouldBe
                "(sentence (simple-noun-phrase (article a) (noun professor)) (verb lectures))"
        }
    })
