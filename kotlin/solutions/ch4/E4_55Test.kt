// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.55: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E455Test :
    FunSpec({
        test("Exercise 4.55: the three simple queries answer in data-base order") {
            simpleQueries() shouldBe
                listOf(
                    "?name = [Hacker, Alyssa, P]",
                    "?name = [Fect, Cy, D]",
                    "?name = [Tweakit, Lem, E]",
                    "?name = [Scrooge, Eben]",
                    "?title = [chief, accountant]",
                    "?name = [Cratchet, Robert]",
                    "?title = [scrivener]",
                    "?name = [Bitdiddle, Ben]",
                    "?where = [[Ridge, Road], 10]",
                    "?name = [Reasoner, Louis]",
                    "?where = [[Pine, Tree, Road], 80]",
                    "?name = [Aull, DeWitt]",
                    "?where = [[Onion, Square], 5]",
                )
        }
    })
