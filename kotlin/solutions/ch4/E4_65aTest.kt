// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.65a: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E465aTest :
    FunSpec({
        test("Exercise 4.65a: the deduplicating view keeps one line per wheel") {
            deduplicatedWheel() shouldBe
                listOf(
                    "?who = [Bitdiddle, Ben]",
                    "?who = [Warbucks, Oliver]",
                )
        }
    })
