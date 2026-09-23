// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.28

package sicp.ch1.exercises

import arrow.core.getOrElse
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Random

public class E1_28Test :
    FunSpec({
        test("Miller-Rabin is not fooled by any Carmichael number") {
            ex_1_28() shouldBe
                mapOf(561L to false, 1105L to false, 1729L to false, 2465L to false, 2821L to false, 6601L to false)
        }
        test("Miller-Rabin accepts an ordinary prime and rejects an ordinary composite") {
            millerRabinPrime(97L, 30, Random.seeded(42UL).getOrElse { error("42 is a nonzero seed") }) shouldBe true
            millerRabinPrime(100L, 30, Random.seeded(42UL).getOrElse { error("42 is a nonzero seed") }) shouldBe false
        }
    })
