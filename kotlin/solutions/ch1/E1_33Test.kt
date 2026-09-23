// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.33

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_33Test :
    FunSpec({
        test("sum of squares of primes in [2, 20] is 4+9+25+49+121+169+289+361") {
            sumSquaresOfPrimes(2L, 20L) shouldBe 1027L
        }
        test("product of positive integers below 10 relatively prime to 10 is 1*3*7*9") {
            productOfRelativePrimes(10L) shouldBe 189L
        }
        test("ex_1_33 packages both answers") {
            ex_1_33() shouldBe Pair(1027L, 189L)
        }
    })
