// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.2.6

package sicp.ch1.examples

import arrow.core.getOrElse
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Random

private tailrec fun findDivisor(
    n: Long,
    testDivisor: Long,
): Long =
    when {
        testDivisor * testDivisor > n -> n
        divides(testDivisor, n) -> testDivisor
        else -> findDivisor(n, testDivisor + 1L)
    }

private fun divides(
    a: Long,
    b: Long,
): Boolean = b % a == 0L

/** Theta(sqrt n) steps: only divisors up to sqrt(n) are ever tried. */
public fun smallestDivisor(n: Long): Long = findDivisor(n, 2L)

/** [n] is prime exactly when it is its own smallest divisor. */
public fun isPrime(n: Long): Boolean = n == smallestDivisor(n)

/**
 * Fermat's Little Theorem, applied by successive squaring: Theta(log n)
 * steps, the same shape as [fastExpt] with a `remainder` folded into
 * every squaring step so intermediate values never grow past `m`.
 */
public fun expmod(
    base: Long,
    exp: Long,
    m: Long,
): Long =
    when {
        exp == 0L -> 1L
        exp % 2L == 0L -> square(expmod(base, exp / 2L, m)) % m
        else -> (base * expmod(base, exp - 1L, m)) % m
    }

/** One trial of the Fermat test, drawing `a` from the shared seeded generator of decision 0001. */
public fun fermatTest(
    n: Long,
    rng: Random,
): Boolean {
    val a = 1L + rng.random(n - 1L)
    return expmod(a, n, n) == a
}

/** True if [n] passes the Fermat test [times] times in a row. */
public tailrec fun fastPrime(
    n: Long,
    times: Int,
    rng: Random,
): Boolean =
    when {
        times == 0 -> true
        fermatTest(n, rng) -> fastPrime(n, times - 1, rng)
        else -> false
    }

public class S1_2_6PrimalityTest :
    FunSpec({
        test("smallestDivisor and isPrime agree on the section's own examples") {
            smallestDivisor(15L) shouldBe 3L
            isPrime(7L) shouldBe true
            isPrime(15L) shouldBe false
        }
        test("expmod matches naive modular exponentiation") {
            expmod(7L, 200L, 13L) shouldBe 3L
        }
        test("the Fermat test accepts a prime and rejects a composite, with a fixed seed") {
            fastPrime(97L, 20, Random.seeded(42UL).getOrElse { error("42 is a nonzero seed") }) shouldBe true
            fastPrime(100L, 20, Random.seeded(42UL).getOrElse { error("42 is a nonzero seed") }) shouldBe false
        }
    })
