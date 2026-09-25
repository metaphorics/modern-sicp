// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_58

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_58Test :
    FunSpec({
        test("Exercise 4.58: the big-shot query") {
            bigShotQuery() shouldBe
                listOf(
                    "query: (big-shot ?person ?division)",
                    "(big-shot (Warbucks Oliver) administration)",
                    "(big-shot (Bitdiddle Ben) computer)",
                    "(big-shot (Scrooge Eben) accounting)",
                )
        }
    })
