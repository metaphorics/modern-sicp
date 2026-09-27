// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.25

package sicp.ch1.exercises

import java.math.BigInteger

private fun square(x: Long): Long = x * x

/** The section's own `expmod`: every intermediate value stays below `m`. */
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

/** Alyssa's shortcut over plain `Long`: the JVM wraps this silently once `base^exp` overflows. */
private fun fastExptWrapping(
    base: Long,
    exp: Long,
): Long =
    when {
        exp == 0L -> {
            1L
        }

        exp % 2L == 0L -> {
            val half = fastExptWrapping(base, exp / 2L)
            half * half
        }

        else -> {
            base * fastExptWrapping(base, exp - 1L)
        }
    }

/** Alyssa's shortcut over `BigInteger`: correct, at the cost of building the full unreduced power. */
private fun fastExptBig(
    base: BigInteger,
    exp: Long,
): BigInteger =
    when {
        exp == 0L -> {
            BigInteger.ONE
        }

        exp % 2L == 0L -> {
            val half = fastExptBig(base, exp / 2L)
            half * half
        }

        else -> {
            base * fastExptBig(base, exp - 1L)
        }
    }

public fun ex_1_25(): Triple<Long, Long, Long> {
    val correct = expmod(7L, 200L, 13L)
    val naiveLongWrapped = fastExptWrapping(7L, 200L) % 13L
    val naiveBigInteger = fastExptBig(BigInteger.valueOf(7L), 200L).mod(BigInteger.valueOf(13L)).toLong()
    return Triple(correct, naiveLongWrapped, naiveBigInteger)
}
