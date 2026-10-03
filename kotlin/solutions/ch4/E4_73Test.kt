// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_73

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_73Test :
    FunSpec({
        test("Exercise 4.73: the delayed flatten delivers a prefix beside an infinite branch") {
            flattenDelayDebate() shouldBe
                listOf(
                    "?who = [Hacker, Alyssa, P]",
                    "?who = Minnie",
                    "?who = [Fect, Cy, D]",
                    "?who = Minnie",
                    "?who = [Tweakit, Lem, E]",
                    "?who = Minnie",
                )
        }
    })
