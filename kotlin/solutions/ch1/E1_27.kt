// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.27

package sicp.ch1.exercises

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

/** True exactly when `a^n` is congruent to `a` modulo `n` for every `a` from 1 up to (not including) `n`. */
public fun foolsFermat(n: Long): Boolean = (1 until n).all { a -> expmod(a, n, n) == a }

private val carmichaelNumbers = listOf(561L, 1105L, 1729L, 2465L, 2821L, 6601L)

public fun ex_1_27(): Map<Long, Boolean> = carmichaelNumbers.associateWith { foolsFermat(it) }
