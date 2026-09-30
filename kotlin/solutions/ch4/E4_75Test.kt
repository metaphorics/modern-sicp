// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_75

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_75Test :
    FunSpec({
        test("Exercise 4.75: the unique special form") {
            uniqueDemos() shouldBe
                listOf(
                    "unique (job ?x (computer wizard)):",
                    "?x = [Bitdiddle, Ben]",
                    "unique (job ?x (computer programmer)): 0 answer(s)",
                    "jobs held by exactly one:",
                    "?x = [Bitdiddle, Ben]",
                    "?j = [computer, wizard]",
                    "?x = [Tweakit, Lem, E]",
                    "?j = [computer, technician]",
                    "?x = [Reasoner, Louis]",
                    "?j = [computer, programmer, trainee]",
                    "?x = [Warbucks, Oliver]",
                    "?j = [administration, big, wheel]",
                    "?x = [Scrooge, Eben]",
                    "?j = [accounting, chief, accountant]",
                    "?x = [Cratchet, Robert]",
                    "?j = [accounting, scrivener]",
                    "?x = [Aull, DeWitt]",
                    "?j = [administration, secretary]",
                    "bosses with exactly one report:",
                    "?person = [Reasoner, Louis]",
                    "?boss = [Hacker, Alyssa, P]",
                    "?person = [Cratchet, Robert]",
                    "?boss = [Scrooge, Eben]",
                )
        }
    })
