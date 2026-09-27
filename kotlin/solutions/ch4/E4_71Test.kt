// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_71

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_71Test :
    FunSpec({
        test("Exercise 4.71: the delay debate") {
            delayDebate() shouldBe
                listOf(
                    "delayed engine, first three answers of the unanchored query:",
                    "(outranked-by (Hacker Alyssa P) (Bitdiddle Ben))",
                    "(outranked-by (Hacker Alyssa P) (Warbucks Oliver))",
                    "(outranked-by (Fect Cy D) (Bitdiddle Ben))",
                    "louis (plain stream-append in simple_query, plain interleave in disjoin): 1 answer(s)",
                    "constructing the first answer reached 3 rule applications -- returned",
                    "married cycle, delayed engine, first three answers:",
                    "(married Mickey Minnie)",
                    "(married Mickey Minnie)",
                    "(married Mickey Minnie)",
                    "married cycle under louis: -1 answer(s) -- the construction diverges before any answer (engine recursion exhausted)",
                )
        }
    })
