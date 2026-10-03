// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_78

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_78Test :
    FunSpec({
        test("Exercise 4.78: the query as a nondeterministic program") {
            ambQueryDemos() shouldBe
                listOf(
                    "Hacker Alyssa P (choice 1)",
                    "Fect Cy D (choice 2)",
                    "Tweakit Lem E (choice 3)",
                    "?x = [Hacker, Alyssa, P]",
                    "?x = [Fect, Cy, D]",
                    "?x = [Tweakit, Lem, E]",
                    "total choices: 8",
                )
        }
    })
