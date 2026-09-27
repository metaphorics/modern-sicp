// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_10

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_10Test :
    FunSpec({
        test("Exercise 5.10: the new syntax expands to the book's instructions and runs") {
            newSyntaxRuns() shouldBe
                listOf(
                    "gcd(206, 40) in the new syntax = 2",
                    "countdown(3) sum = 3",
                )
        }
    })
