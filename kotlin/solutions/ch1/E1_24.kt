// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.24

package sicp.ch1.exercises

import arrow.core.getOrElse
import sicp.runtime.Random

private fun square(x: Long): Long = x * x

private fun expmod(
    base: Long,
    exp: Long,
    m: Long,
): Long =
    when {
        exp == 0L -> 1L
        exp % 2L == 0L -> square(expmod(base, exp / 2L, m)) % m
        else -> (base * expmod(base, exp - 1L, m)) % m
    }

private fun fermatTest(
    n: Long,
    rng: Random,
): Boolean {
    val a = 1L + rng.random(n - 1L)
    return expmod(a, n, n) == a
}

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

/** The 12 primes exercise 1.22 finds above 1000, 10,000, 100,000, and 1,000,000. */
private val primesFrom1_22 =
    listOf(
        1009L,
        1013L,
        1019L,
        10_007L,
        10_009L,
        10_037L,
        100_003L,
        100_019L,
        100_043L,
        1_000_003L,
        1_000_033L,
        1_000_037L,
    )

public fun ex_1_24(): List<Boolean> =
    primesFrom1_22.map { n ->
        fastPrime(n, 20, Random.seeded(42UL).getOrElse { error("42 is a nonzero seed") })
    }
