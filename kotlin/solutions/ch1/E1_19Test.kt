// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.19

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import java.math.BigInteger

public class E1_19Test :
    FunSpec({
        test("Fib(20) by successive squaring of the transform") {
            ex_1_19(20L) shouldBe BigInteger.valueOf(6765L)
        }
        test("agrees with the direct definition across the first 30 terms") {
            var a = BigInteger.ZERO
            var b = BigInteger.ONE
            for (n in 0..30) {
                fibLog(n.toLong()) shouldBe a
                val next = a + b
                a = b
                b = next
            }
        }
        test("reaches Fib(1000), a 209-digit number no Long could hold") {
            val fib1000 = fibLog(1000L)
            fib1000.toString().length shouldBe 209
        }
    })
