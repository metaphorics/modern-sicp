// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.28

package sicp.ch1.exercises

import arrow.core.getOrElse
import sicp.runtime.Random

/** Squares [x] modulo [m], signaling (by returning 0) a nontrivial square root of 1. */
private fun squareModCheckingRoot(
    x: Long,
    m: Long,
): Long {
    val sq = (x * x) % m
    return if (sq == 1L && x != 1L && x != m - 1L) 0L else sq
}

private fun millerRabinExpmod(
    base: Long,
    exp: Long,
    m: Long,
): Long =
    when {
        exp == 0L -> 1L
        exp % 2L == 0L -> squareModCheckingRoot(millerRabinExpmod(base, exp / 2L, m), m)
        else -> (base * millerRabinExpmod(base, exp - 1L, m)) % m
    }

private fun millerRabinTest(
    n: Long,
    rng: Random,
): Boolean {
    val a = 1L + rng.random(n - 1L)
    return millerRabinExpmod(a, n - 1L, n) == 1L
}

/** True if [n] passes the Miller-Rabin test [times] times in a row. */
public tailrec fun millerRabinPrime(
    n: Long,
    times: Int,
    rng: Random,
): Boolean =
    when {
        times == 0 -> true
        millerRabinTest(n, rng) -> millerRabinPrime(n, times - 1, rng)
        else -> false
    }

private val carmichaelNumbers = listOf(561L, 1105L, 1729L, 2465L, 2821L, 6601L)

public fun ex_1_28(): Map<Long, Boolean> =
    carmichaelNumbers.associateWith { n ->
        millerRabinPrime(n, 30, Random.seeded(42UL).getOrElse { error("42 is a nonzero seed") })
    }
