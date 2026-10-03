// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.65: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E465Test :
    FunSpec({
        test("Exercise 4.65: the wheel stream carries one frame per chain") {
            wheelQuery() shouldBe
                listOf(
                    "?who = [Bitdiddle, Ben]",
                    "?who = [Warbucks, Oliver]",
                    "?who = [Warbucks, Oliver]",
                    "?who = [Warbucks, Oliver]",
                    "?who = [Warbucks, Oliver]",
                    "Warbucks appears 4 times",
                    "Ben appears 1 times",
                )
        }
    })
