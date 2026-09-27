// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.23

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

private fun divides(
    a: Long,
    b: Long,
): Boolean = b % a == 0L

private tailrec fun findDivisorNaive(
    n: Long,
    testDivisor: Long,
): Long =
    when {
        testDivisor * testDivisor > n -> n
        divides(testDivisor, n) -> testDivisor
        else -> findDivisorNaive(n, testDivisor + 1L)
    }

private fun smallestDivisorNaive(n: Long): Long = findDivisorNaive(n, 2L)

public class E1_23Test :
    FunSpec({
        test("the faster search still finds 19999's smallest divisor") {
            ex_1_23(19_999L) shouldBe 7L
        }
        test("skipping even test divisors never changes the answer") {
            listOf(199L, 1999L, 19_999L, 1_000_003L, 100L, 561L).forEach { n ->
                ex_1_23(n) shouldBe smallestDivisorNaive(n)
            }
        }
    })
