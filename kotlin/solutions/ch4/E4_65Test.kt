// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_65

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_65Test :
    FunSpec({
        test("Exercise 4.65: the wheel listing") {
            wheelQuery() shouldBe
                listOf(
                    "query: (wheel ?who)",
                    "(wheel (Bitdiddle Ben))",
                    "(wheel (Warbucks Oliver))",
                    "(wheel (Warbucks Oliver))",
                    "(wheel (Warbucks Oliver))",
                    "(wheel (Warbucks Oliver))",
                    "Warbucks appears 4 times",
                    "Ben appears 1 time",
                )
        }
    })
