// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.3

package sicp.ch1.exercises

/**
 * The sum of the squares of the two larger of three numbers: branch on the
 * smallest, then square the surviving pair.
 */
public fun ex_1_03(
    a: Long,
    b: Long,
    c: Long,
): Long {
    fun square(x: Long): Long = x * x
    return when {
        a <= b && a <= c -> square(b) + square(c)
        b <= a && b <= c -> square(a) + square(c)
        else -> square(a) + square(b)
    }
}
