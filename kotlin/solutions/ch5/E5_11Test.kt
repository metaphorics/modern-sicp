// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_11

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_11Test :
    FunSpec({
        test("Exercise 5.11: the three save and restore disciplines on the same machines") {
            restoreDisciplineRuns() shouldBe
                listOf(
                    "out-of-order restore under (a) name-blind: y = 2",
                    "out-of-order restore under (b) checking: restore y but the stack holds x",
                    "out-of-order restore under (c) per-register: y = 1",
                    "fib(3) under (a): val = 2",
                    "fib(3) under (b): val = 2",
                    "fib(3) under (c): val = 2",
                    "fib(3) with the eliminated assign, discipline (a): val = 2",
                    "fib(5) with the eliminated assign, discipline (a): val = 5",
                    "the eliminated controller under (b): restore n but the stack holds val",
                )
        }
    })
