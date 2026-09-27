// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_64

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_64Test :
    FunSpec({
        test("Exercise 4.64: Louis's swapped outranked-by") {
            louisOutranked() shouldBe
                listOf(
                    "(outranked-by (Bitdiddle Ben) (Warbucks Oliver)) under Louis's swapped rule:",
                    "first answer: -1 -- the recursion-first and re-enumerates the whole closure at construction, before any answer exists",
                    "the book's conjunct order answers completely:",
                    "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))",
                )
        }
    })
