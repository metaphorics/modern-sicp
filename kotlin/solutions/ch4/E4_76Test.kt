// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_76

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_76Test :
    FunSpec({
        test("Exercise 4.76: merge-and") {
            mergeAndDemos() shouldBe
                listOf(
                    "?x = [Hacker, Alyssa, P]",
                    "?boss = [Bitdiddle, Ben]",
                    "?x = [Fect, Cy, D]",
                    "?boss = [Bitdiddle, Ben]",
                    "merge agrees with and: true",
                    "compatibility checks: 16",
                )
        }
    })
