// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.24

package sicp.ch1.exercises

import arrow.core.getOrElse
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Random

public class E1_24Test :
    FunSpec({
        test("fastPrime accepts all 12 primes found in exercise 1.22") {
            ex_1_24() shouldBe List(12) { true }
        }
        test("fastPrime rejects an ordinary composite, with the same fixed seed") {
            fastPrime(100L, 20, Random.seeded(42UL).getOrElse { error("42 is a nonzero seed") }) shouldBe false
        }
    })
