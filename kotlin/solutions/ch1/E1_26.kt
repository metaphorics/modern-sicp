// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.26

package sicp.ch1.exercises

private fun square(x: Long): Long = x * x

/** The correct version: one recursive call per level, wrapped in `square`. */
private fun expmodSquare(
    base: Long,
    exp: Long,
    m: Long,
    calls: IntArray,
): Long {
    calls[0] += 1
    return when {
        exp == 0L -> 1L
        exp % 2L == 0L -> square(expmodSquare(base, exp / 2L, m, calls)) % m
        else -> (base * expmodSquare(base, exp - 1L, m, calls)) % m
    }
}

/** Louis's version: an explicit multiplication evaluates the recursive call twice per even level. */
private fun expmodDouble(
    base: Long,
    exp: Long,
    m: Long,
    calls: IntArray,
): Long {
    calls[0] += 1
    return when {
        exp == 0L -> 1L
        exp % 2L == 0L -> (expmodDouble(base, exp / 2L, m, calls) * expmodDouble(base, exp / 2L, m, calls)) % m
        else -> (base * expmodDouble(base, exp - 1L, m, calls)) % m
    }
}

public fun ex_1_26(): Pair<Int, Int> {
    val squareCalls = intArrayOf(0)
    expmodSquare(4L, 64L, 97L, squareCalls)
    val doubleCalls = intArrayOf(0)
    expmodDouble(4L, 64L, 97L, doubleCalls)
    return squareCalls[0] to doubleCalls[0]
}
