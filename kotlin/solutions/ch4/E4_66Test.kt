// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_66

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_66Test :
    FunSpec({
        test("Exercise 4.66: the salary sums") {
            salarySums() shouldBe
                listOf(
                    "sum over the book's query = 75000",
                    "Ben's scheme on the wheel query = 660000 (duplicate frames count)",
                    "salvage, distinct answers only = 210000",
                )
        }
    })
