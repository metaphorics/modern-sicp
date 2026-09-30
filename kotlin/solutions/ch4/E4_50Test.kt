// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.50: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E450Test :
    FunSpec({
        test("Exercise 4.50: the seeded shuffle enumerates 4, 2, 3, 5, 1") {
            rambEnumeration() shouldBe listOf("(4)", "(2)", "(3)", "(5)", "(1)")
        }

        test("Exercise 4.50: the ramb generator escapes the boring first words") {
            rambGeneratedFirst() shouldBe
                "(sentence (simple-noun-phrase (article a) (noun class)) (verb lectures))"
        }

        test("Exercise 4.50: the seeded enumeration is reproducible from its seed") {
            rambEnumeration() shouldBe rambEnumeration()
        }
    })
