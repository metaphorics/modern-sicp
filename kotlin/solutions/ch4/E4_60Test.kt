// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_60

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_60Test :
    FunSpec({
        test("Exercise 4.60: the lives-near queries") {
            livesNearQueries() shouldBe
                listOf(
                    "query: (lives-near ?person (Hacker Alyssa P))",
                    "(lives-near (Fect Cy D) (Hacker Alyssa P))",
                    "query: (lives-near ?person-1 ?person-2)",
                    "(lives-near (Bitdiddle Ben) (Reasoner Louis))",
                    "(lives-near (Fect Cy D) (Hacker Alyssa P))",
                    "(lives-near (Bitdiddle Ben) (Aull DeWitt))",
                    "(lives-near (Hacker Alyssa P) (Fect Cy D))",
                    "(lives-near (Reasoner Louis) (Bitdiddle Ben))",
                    "(lives-near (Reasoner Louis) (Aull DeWitt))",
                    "(lives-near (Aull DeWitt) (Bitdiddle Ben))",
                    "(lives-near (Aull DeWitt) (Reasoner Louis))",
                    "query: (lives-near-unique ?person-1 ?person-2)",
                    "(lives-near-unique (Fect Cy D) (Hacker Alyssa P))",
                    "(lives-near-unique (Reasoner Louis) (Bitdiddle Ben))",
                    "(lives-near-unique (Aull DeWitt) (Bitdiddle Ben))",
                    "(lives-near-unique (Aull DeWitt) (Reasoner Louis))",
                )
        }
    })
