// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_77

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_77Test :
    FunSpec({
        test("Exercise 4.77: the delayed filters") {
            delayedFilterDemos() shouldBe
                listOf(
                    "stock not-first: 0 line(s), the unbound filter drops everything",
                    "stock lisp-value-first: 0 line(s), the unbound guard drops everything",
                    "postponed not-first: 6 answer(s), the bound order's: true",
                    "postponed lisp-value-first: 5 answer(s), the bound order's: true",
                    "stock rule filter, bound later by the caller: 0 line(s)",
                    "postponed rule filter, bound later by the caller: true",
                    "postponed bound order: true",
                    "a filter no conjunct binds runs at the end, as stock: true",
                    "?x = [Tweakit, Lem, E]",
                    "?y = [Bitdiddle, Ben]",
                    "?x = [Reasoner, Louis]",
                    "?y = [Hacker, Alyssa, P]",
                    "?x = [Bitdiddle, Ben]",
                    "?y = [Warbucks, Oliver]",
                    "?x = [Scrooge, Eben]",
                    "?y = [Warbucks, Oliver]",
                    "?x = [Cratchet, Robert]",
                    "?y = [Scrooge, Eben]",
                    "?x = [Aull, DeWitt]",
                    "?y = [Warbucks, Oliver]",
                    "?who = [Bitdiddle, Ben]",
                    "?amount = 60000",
                    "?who = [Hacker, Alyssa, P]",
                    "?amount = 40000",
                    "?who = [Fect, Cy, D]",
                    "?amount = 35000",
                    "?who = [Warbucks, Oliver]",
                    "?amount = 150000",
                    "?who = [Scrooge, Eben]",
                    "?amount = 75000",
                )
        }
    })
