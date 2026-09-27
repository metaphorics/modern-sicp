// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_55

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_55Test :
    FunSpec({
        test("Exercise 4.55: the three simple queries") {
            simpleQueries() shouldBe
                listOf(
                    "query: (supervisor ?name (Bitdiddle Ben))",
                    "(supervisor (Hacker Alyssa P) (Bitdiddle Ben))",
                    "(supervisor (Fect Cy D) (Bitdiddle Ben))",
                    "(supervisor (Tweakit Lem E) (Bitdiddle Ben))",
                    "query: (job ?name (accounting . ?title))",
                    "(job (Scrooge Eben) (accounting chief accountant))",
                    "(job (Cratchet Robert) (accounting scrivener))",
                    "query: (address ?name (Slumerville . ?where))",
                    "(address (Bitdiddle Ben) (Slumerville (Ridge Road) 10))",
                    "(address (Reasoner Louis) (Slumerville (Pine Tree Road) 80))",
                    "(address (Aull DeWitt) (Slumerville (Onion Square) 5))",
                )
        }
    })
