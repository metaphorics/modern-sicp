// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.19a

package sicp.ch1.exercises

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_19aTest :
    FunSpec({
        test("the checked transform succeeds through Fib(91)") {
            ex_1_19a(91L) shouldBe 4_660_046_610_375_530_309L
        }
        test("Fib(92) itself fits a Long, yet the checked transform overflows computing it") {
            fibLog(92L).toLong() shouldBe 7_540_113_804_746_346_429L
            shouldThrow<ArithmeticException> { ex_1_19a(92L) }
        }
    })
