// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.56

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_56Test :
    FunSpec({
        test("Exercise 4.56: the three compound queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            compoundQueries() shouldBe
                listOf(
                    "?person = [Hacker, Alyssa, P]",
                    "?where = [Cambridge, [Mass, Ave], 78]",
                    "?person = [Fect, Cy, D]",
                    "?where = [Cambridge, [Ames, Street], 3]",
                    "?person = [Tweakit, Lem, E]",
                    "?where = [Boston, [Bay, State, Road], 22]",
                    "?person = [Hacker, Alyssa, P]",
                    "?amount = 40000",
                    "?person = [Fect, Cy, D]",
                    "?amount = 35000",
                    "?person = [Tweakit, Lem, E]",
                    "?amount = 25000",
                    "?person = [Reasoner, Louis]",
                    "?amount = 30000",
                    "?person = [Cratchet, Robert]",
                    "?amount = 18000",
                    "?person = [Aull, DeWitt]",
                    "?amount = 25000",
                    "?person = [Bitdiddle, Ben]",
                    "?person = [Scrooge, Eben]",
                    "?person = [Cratchet, Robert]",
                    "?person = [Aull, DeWitt]",
                )
        }
    })
