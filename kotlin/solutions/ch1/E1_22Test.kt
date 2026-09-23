// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.22

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_22Test :
    FunSpec({
        test("the three smallest primes above each threshold") {
            ex_1_22() shouldBe
                mapOf(
                    1_000L to listOf(1009L, 1013L, 1019L),
                    10_000L to listOf(10_007L, 10_009L, 10_037L),
                    100_000L to listOf(100_003L, 100_019L, 100_043L),
                    1_000_000L to listOf(1_000_003L, 1_000_033L, 1_000_037L),
                )
        }
        test("timedPrimeTest reports no result for a composite and a nonnegative elapsed time for a prime") {
            timedPrimeTest(100L) shouldBe null
            val result = timedPrimeTest(101L)
            (result != null && result.elapsedNanos >= 0L) shouldBe true
        }
    })
