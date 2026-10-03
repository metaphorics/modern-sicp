// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.73

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_73Test :
    FunSpec({
        test("Exercise 4.73: the delayed flatten delivers a prefix beside an infinite branch").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
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
