// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.8

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_08Test :
    FunSpec({
        test("f(0) + f(1) is always 0: Kotlin evaluates the left operand first") {
            val f = makeF()
            (f(0) + f(1)) shouldBe 0
        }

        test("swapping the call order flips which operand keeps its value") {
            val f = makeF()
            (f(1) + f(0)) shouldBe 1
        }

        test("only the first call of a fresh f keeps its argument") {
            val f = makeF()
            f(42) shouldBe 42
            f(7) shouldBe 0
            f(3) shouldBe 0
        }
    })
