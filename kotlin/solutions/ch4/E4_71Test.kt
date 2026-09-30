// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.71

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_71Test :
    FunSpec({
        test("Exercise 4.71: the delay debate") {
            delayDebate() shouldBe
                listOf(
                    "delayed engine, first three of the unanchored outranked query:",
                    "?staff-person = [Hacker, Alyssa, P]",
                    "?boss = [Bitdiddle, Ben]",
                    "?staff-person = [Hacker, Alyssa, P]",
                    "?boss = [Warbucks, Oliver]",
                    "?staff-person = [Fect, Cy, D]",
                    "?boss = [Bitdiddle, Ben]",
                    "married cycle, first answer:",
                    "?who = Minnie",
                )
        }
    })
