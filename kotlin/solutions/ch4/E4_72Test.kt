// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_72

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_72Test :
    FunSpec({
        test("Exercise 4.72: interleaving reaches the supervisor branch; appending never does") {
            interleaveVersusAppend() shouldBe
                listOf(
                    "interleaved, first 4 answers:",
                    "?a = [Minnie, Mouse]",
                    "?b = [Mickey, Mouse]",
                    "?a = [Hacker, Alyssa, P]",
                    "?b = [Bitdiddle, Ben]",
                    "?a = [Mickey, Mouse]",
                    "?b = [Minnie, Mouse]",
                    "?a = [Fect, Cy, D]",
                    "?b = [Bitdiddle, Ben]",
                    "supervisor answers in the interleaved 4: 2",
                    "appended, first 4 answers:",
                    "?a = [Minnie, Mouse]",
                    "?b = [Mickey, Mouse]",
                    "?a = [Mickey, Mouse]",
                    "?b = [Minnie, Mouse]",
                    "?a = [Minnie, Mouse]",
                    "?b = [Mickey, Mouse]",
                    "?a = [Mickey, Mouse]",
                    "?b = [Minnie, Mouse]",
                    "supervisor answers in the appended 4: 0",
                )
        }
    })
