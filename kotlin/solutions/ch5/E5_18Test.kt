// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_18

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_18Test :
    FunSpec({
        test("Exercise 5.18: every store to a traced register reports old and new content") {
            tracedRegisterGcdLog() shouldBe
                listOf(
                    "a: *unassigned* -> 12",
                    "b: *unassigned* -> 8",
                    "t: *unassigned* -> 4",
                    "a: 12 -> 8",
                    "b: 8 -> 4",
                    "t: 4 -> 0",
                    "a: 8 -> 4",
                    "b: 4 -> 0",
                    "gcd(12, 8) = 4",
                )
        }
    })
