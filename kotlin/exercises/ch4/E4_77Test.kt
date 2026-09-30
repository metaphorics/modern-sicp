// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.77

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_77Test :
    FunSpec({
        test("Exercise 4.77: the delayed filters").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            delayedFilterDemos() shouldBe
                listOf(
                    "not-first: 0 frame(s), the unbound filter drops everything",
                    "lisp-value-first: 0 frame(s), the unbound guard drops everything",
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
